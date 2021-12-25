use crate::analyser::GameAnalyser;
use crate::db::PgPooledConnection;
use crate::opening_tree::{MoveEntry, OpeningTree};
use crate::player::Player;

use futures_util::StreamExt;
use pgn_reader::BufferedReader;
use serde::Serialize;
use std::collections::HashMap;

use regex::Regex;

use crate::models::game::{get_game, save_game, save_games, Game};

const API_BASE: &str = "https://lichess.org";

pub async fn get_game_lichess(id: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/game/export/{}?clocks=false?evals=false", API_BASE, id);
    reqwest::get(url).await?.text().await
}

async fn get_games_player(username: &str, num: usize) -> Result<String, reqwest::Error> {
    let url = format!(
        "{}/api/games/user/{}?max={}&rated=true",
        API_BASE, username, num
    );
    reqwest::get(url).await?.text().await
}

#[derive(Debug)]
pub enum AnalysisErrors {
    Lichess,
    Pgn,
    NotFound,
}

pub async fn analyse_lichess_game(
    conn: PgPooledConnection,
    game_id: &str,
) -> Result<Game, AnalysisErrors> {
    if let Some(game) = get_game(game_id, &conn) {
        return Ok(game);
    }

    if let Ok(pgn) = get_game_lichess(game_id).await {
        if pgn.contains("<!DOCTYPE html>") {
            return Err(AnalysisErrors::NotFound);
        }

        let mut reader = BufferedReader::new_cursor(&pgn[..]);
        let mut analyser = GameAnalyser::new("".to_string());

        if reader.read_game(&mut analyser).is_err() {
            return Err(AnalysisErrors::Pgn);
        }

        match save_game(analyser.game, &conn) {
            Ok(g) => Ok(g),
            Err(_) => Err(AnalysisErrors::Lichess),
        }
    } else {
        Err(AnalysisErrors::Lichess)
    }
}

pub async fn opening_player(
    username: &str,
) -> Result<HashMap<String, Vec<MoveEntry>>, AnalysisErrors> {
    if let Ok(pgn) = get_games_player(username, 2).await {
        let mut reader = BufferedReader::new_cursor(&pgn[..]);
        let mut opening = OpeningTree::new();

        loop {
            let res = reader.read_game(&mut opening);
            match res {
                Err(_) => return Err(AnalysisErrors::Pgn),
                Ok(None) => break,
                _ => {}
            }
        }

        Ok(opening.move_stat)
    } else {
        Err(AnalysisErrors::NotFound)
    }
}

fn analyse_games(pgns: String, analyser: &mut GameAnalyser) -> Vec<Game> {
    let mut reader = BufferedReader::new_cursor(&pgns[..]);
    let mut matches: Vec<Game> = Vec::new();

    while let Some(ok) = reader.read_game(analyser).unwrap() {
        //game over
        let game = analyser.game.clone();
        println!("{:?}", &game);
        matches.push(game);
    }

    matches
}

pub async fn analyse_player(conn: PgPooledConnection, player_id: &str) {
    let url = format!(
        "{}/api/games/user/{}?max=100&clocks=false&evals=false",
        API_BASE, player_id
    );
    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();

    let mut pgns = String::from("");
    let mut pgn_count = 0;

    let re = Regex::new(r"lichess.org/.{8}").unwrap();

    loop {
        match stream.next().await {
            Some(Ok(chunk)) => {
                let pgn = std::str::from_utf8(&chunk).unwrap();
                if let Some(mat) = re.find(pgn) {
                    let url = mat.as_str();
                    if let Some(id) = url.split('/').nth(1) {
                        if get_game(id, &conn).is_some() {
                            continue;
                        }
                    }
                }
                
                pgns.push_str(&format!("{}\n", pgn));
                pgn_count += 1;

                if pgn_count < 5 {
                    continue;
                }
                println!("{}", &pgn_count);

                let mut analyser = GameAnalyser::new("".to_string());
                let games = analyse_games(pgns, &mut analyser);
                println!("{:?}", games);
                save_games(games, &conn);
                pgn_count = 0;
                pgns = String::from("");
            }
            Some(Err(_)) | None => break,
        }
    }
}
