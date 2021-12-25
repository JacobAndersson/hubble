use crate::analyser::GameAnalyser;
use crate::player::Player;
use crate::opening_tree::{MoveEntry, OpeningTree};
use crate::db::PgPooledConnection;

use pgn_reader::BufferedReader;
use serde::Serialize;
use std::collections::HashMap;
use futures_util::StreamExt;

use crate::models::{Game as GameModel, save_games};

const API_BASE: &str = "https://lichess.org";

pub async fn get_game(id: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/game/export/{}?clocks=false?evals=false", API_BASE, id);
    reqwest::get(url).await?.text().await
}

async fn get_games_player(username: &str, num: usize) -> Result<String, reqwest::Error> {
    let url = format!(
        "{}/api/games/user/{}?max={}&rated=true",
        API_BASE, username, num
    );
    println!("{}", url);
    reqwest::get(url).await?.text().await
}

#[derive(Debug)]
pub enum AnalysisErrors {
    Lichess,
    Pgn,
    NotFound,
}

#[derive(Debug, Serialize)]
pub struct Game {
    white: Player,
    black: Player,
    scores: Vec<f32>,
    moves: Vec<String>
}

pub async fn analyse_lichess_game(game_id: &str) -> Result<Game, AnalysisErrors> {
    if let Ok(pgn) = get_game(game_id).await {
        if pgn.contains("<!DOCTYPE html>") {
            return Err(AnalysisErrors::NotFound);
        }

        let mut reader = BufferedReader::new_cursor(&pgn[..]);
        let mut analyser = GameAnalyser::new("".to_string());

        if reader.read_game(&mut analyser).is_err() {
            return Err(AnalysisErrors::Pgn);
        }

        let game = &analyser.scores;
        let moves = analyser.moves.to_vec();

        if game.contains(&None) {
            return Err(AnalysisErrors::Pgn);
        }
        let scores = game.iter().map(|x| x.unwrap_or(0.)).collect::<Vec<f32>>();

        Ok(Game {
            scores,
            moves,
            black: analyser.black.clone(),
            white: analyser.white.clone(),
        })
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

fn analyse_games(pgns: String, analyser: &mut GameAnalyser) -> Vec<GameModel> {
    let mut reader = BufferedReader::new_cursor(&pgns[..]);
    let mut matches: Vec<GameModel> = Vec::new();

    while let Some(ok) = reader.read_game(analyser).unwrap() {
       //game over
       let game = analyser.game.clone();
       println!("{:?}", &game);
       matches.push(game);
    }

    matches
}

pub async fn analyse_player(conn: PgPooledConnection, player_id: &str) {
    let url = format!("{}/api/games/user/{}?max=100&clocks=false&evals=false", API_BASE, player_id);
    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();

    let mut pgns = String::from("");
    let mut pgn_count = 0;

    loop {
        match stream.next().await {
            Some(Ok(chunk)) => {
                pgns.push_str(&format!("{}\n", std::str::from_utf8(&chunk).unwrap()));
                pgn_count += 1;

                if pgn_count < 10 {
                    continue;
                }
                println!("{}", &pgn_count);

                let mut analyser = GameAnalyser::new("".to_string());
                let games = analyse_games(pgns, &mut analyser);
                println!("SAVING");
                save_games(games, &conn);
                println!("SAVED");
                pgn_count = 0;
                pgns = String::from("");
            },
            Some(Err(_)) | None => break,
        }
    }
}


