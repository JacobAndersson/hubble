use crate::analyser::GameAnalyser;
use crate::opening_tree::{MoveEntry, OpeningTree};

use pgn_reader::BufferedReader;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    white: String,
    black: String,
    scores: Vec<f32>,
}

pub async fn analyse_lichess_game(id: &str) -> Result<Game, AnalysisErrors> {
    if let Ok(pgn) = get_game(id).await {
        if pgn.contains("<!DOCTYPE html>") {
            return Err(AnalysisErrors::NotFound);
        }

        let mut reader = BufferedReader::new_cursor(&pgn[..]);
        let mut analyser = GameAnalyser::new();

        match reader.read_game(&mut analyser) {
            Err(_) => return Err(AnalysisErrors::Pgn),
            _ => {}
        }
        let game = &analyser.scores;

        if game.contains(&None) {
            return Err(AnalysisErrors::Pgn);
        }
        let scores = game.iter().map(|x| x.unwrap_or(0.)).collect::<Vec<f32>>();

        Ok(Game {
            scores,
            black: analyser.black,
            white: analyser.white,
        })
    } else {
        return Err(AnalysisErrors::Lichess);
    }
}

pub async fn opening_player(
    username: &str,
) -> Result<HashMap<String, Vec<MoveEntry>>, AnalysisErrors> {
    if let Ok(pgn) = get_games_player(username, 2).await {
        let mut reader = BufferedReader::new_cursor(&pgn[..]);
        let mut opening = OpeningTree::new();

        while let res = reader.read_game(&mut opening) {
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
