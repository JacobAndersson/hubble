use reqwest;
use pgn_reader::BufferedReader;
use crate::analyser::GameAnalyser;

const API_BASE: &str = "https://lichess.org";

pub async fn get_game(id: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/game/export/{}?clocks=false?evals=false", API_BASE, id);
    reqwest::get(url).await?.text().await
}

#[derive(Debug)]
pub enum AnalysisErrors {
    Lichess,
    Pgn
}

#[derive(Debug)]
pub struct Game {
    white: String,
    black: String,
    scores: Vec<f32>
}

pub async fn analyse_lichess_game(id: &str) -> Result<Game, AnalysisErrors> {
    if let Ok(pgn) = get_game(id).await {
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
            white: analyser.white
        })
    } else {
        return Err(AnalysisErrors::Lichess);
    }
}
