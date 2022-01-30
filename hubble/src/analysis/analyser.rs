use pgn_reader::{RawHeader, SanPlus, Skip, Visitor};
use shakmaty::fen::Fen;
use shakmaty::{fen, CastlingMode, Chess, Position};

use crate::stockfish::Stockfish;
use hubble_db::models::game::Game;

pub struct GameAnalyser {
    engine: Stockfish,
    success: bool,
    pos: Chess,
    pub game: Game,
}

impl GameAnalyser {
    pub fn new() -> Self {
        Self {
            engine: Stockfish::new().unwrap(),
            success: true,
            pos: Chess::default(),
            game: Game::empty(),
        }
    }

    fn analyse_position(&mut self) {
        let fen = fen::epd(&self.pos);
        match self.engine.eval_fen(&fen) {
            Some(score) => {
                self.game.scores.push(score.to_string());
            }
            None => {} //handle fail
        }
    }
}

impl Visitor for GameAnalyser {
    type Result = bool;

    fn begin_game(&mut self) {
        self.success = true;
        self.pos = Chess::default();
        self.game = Game::empty();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        match key {
            b"FEN" => {
                let fen = match Fen::from_ascii(value.as_bytes()) {
                    Ok(fen) => fen,
                    Err(_err) => {
                        self.success = false;
                        return;
                    }
                };

                self.pos = match fen.position(CastlingMode::Chess960) {
                    Ok(pos) => pos,
                    Err(_err) => {
                        self.success = false;
                        return;
                    }
                };
            }
            b"White" => {
                let name = std::str::from_utf8(value.as_bytes()).unwrap().to_string();
                self.game.white = name;
            }
            b"Black" => {
                let name = std::str::from_utf8(value.as_bytes()).unwrap().to_string();
                self.game.black = name;
            }
            b"WhiteElo" => {
                if let Ok(vs) = std::str::from_utf8(value.as_bytes()) {
                    if let Ok(rating) = vs.to_string().parse::<i32>() {
                        self.game.white_rating = Some(rating);
                    }
                }
            }
            b"BlackElo" => {
                if let Ok(vs) = std::str::from_utf8(value.as_bytes()) {
                    if let Ok(rating) = vs.to_string().parse::<i32>() {
                        self.game.black_rating = Some(rating);
                    }
                }
            }
            b"LichessURL" | b"Site" => {
                if let Ok(url) = std::str::from_utf8(value.as_bytes()) {
                    let id = String::from(url.split('/').nth(3).unwrap());
                    self.game.id = id;
                }
            }
            b"ECO" => {
                if let Ok(opening) = std::str::from_utf8(value.as_bytes()) {
                    self.game.opening_id = Some(opening.to_string());
                }
            }
            b"Result" => {
                if let Ok(result_string) = std::str::from_utf8(value.as_bytes()) {
                    self.game.winner = match result_string {
                        "1-0" => Some(self.game.white.clone()),
                        "0-1" => Some(self.game.black.clone()),
                        "1/2-1/2" | _ => None,
                    };
                }
            }
            _ => {}
        }
    }

    fn end_headers(&mut self) -> Skip {
        Skip(!self.success)
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.success {
            match san_plus.san.to_move(&self.pos) {
                Ok(m) => {
                    self.pos.play_unchecked(&m);
                    let uci = m.to_uci(self.pos.castles().mode()).to_string();
                    self.game.moves.push(uci);
                    self.analyse_position();
                }
                Err(_err) => {
                    self.success = false;
                }
            }
        }
    }

    fn end_game(&mut self) -> Self::Result {
        false
    }
}
