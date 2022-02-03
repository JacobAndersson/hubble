use crate::analysis::opening_tree::{MoveEntry, OpeningTree};
use crate::analysis::GameAnalyser;

use futures_util::StreamExt;
use pgn_reader::{AsyncBufferedReader, BufferedReader};
use std::collections::HashMap;

use regex::Regex;

use hubble_db::models::game::{get_game, save_game, save_games, Game};
use hubble_db::{PgPooledConnection, PgConnection};

const API_BASE: &str = "https://lichess.org";

pub async fn get_game_lichess(id: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/game/export/{}?clocks=false&evals=false", API_BASE, id);
    reqwest::get(url).await?.text().await
}

pub async fn get_games_player(username: &str, num: usize) -> Result<String, reqwest::Error> {
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
    conn: PgConnection,
    game_id: &str,
) -> Result<Game, AnalysisErrors> {
    if let Some(game) = get_game(game_id, &conn) {
        return Ok(game);
    }
    if let Ok(pgn) = get_game_lichess(game_id).await {
        if pgn.contains("<!DOCTYPE html>") {
            return Err(AnalysisErrors::NotFound);
        }

        let mut reader = AsyncBufferedReader::new_cursor(&pgn[..]);
        let mut analyser = GameAnalyser::new().await;

        if reader.read_game(&mut analyser).await.is_err() {
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

async fn analyse_games(pgns: String, analyser: &mut GameAnalyser) -> Vec<Game> {
    let mut reader = AsyncBufferedReader::new_cursor(&pgns[..]);
    let mut matches: Vec<Game> = Vec::new();

    while let Some(_ok) = reader.read_game(analyser).await.unwrap() {
        //game over
        let game = analyser.game.clone();
        println!("{:?}", &game);
        matches.push(game);
    }

    matches
}

pub async fn analyse_player(
    conn: PgConnection,
    player_id: &str,
) -> Result<Vec<Game>, AnalysisErrors> {
    let url = format!(
        "{}/api/games/user/{}?max=10&clocks=false&evals=false",
        API_BASE, player_id
    );
    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();

    let mut pgns = String::from("");
    let mut pgn_count = 0;

    let re = Regex::new(r"lichess.org/.{8}").unwrap();

    let mut all_games: Vec<Game> = Vec::new();

    while let Some(Ok(chunk)) = stream.next().await {
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

        let mut analyser = GameAnalyser::new().await;
        let games = analyse_games(pgns, &mut analyser).await;
        println!("{:?}", games);
        match save_games(games, &conn) {
            Ok(mut gs) => all_games.append(&mut gs),
            Err(_) => return Err(AnalysisErrors::Lichess),
        }
        pgn_count = 0;
        pgns = String::from("");
    }

    Ok(all_games)
}
