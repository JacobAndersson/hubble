use pgn_reader::{RawHeader, SanPlus, Skip, Visitor};
use shakmaty::fen::Fen;
use shakmaty::{fen, CastlingMode, Chess, Position};

use crate::stockfish::Stockfish;
use crate::player::Player;

pub struct GameAnalyser {
    engine: Stockfish,
    current_id: String, //current Game id
    pub scores: Vec<Option<f32>>,
    success: bool,
    pos: Chess,
    pub black: Player,
    pub white: Player,
    pub moves: Vec<String>
}

impl GameAnalyser {
    pub fn new() -> Self {
        Self {
            engine: Stockfish::new().unwrap(),
            current_id: String::from(""),
            scores: Vec::new(),
            success: true,
            pos: Chess::default(),
            black: Player::empty(),
            white: Player::empty(),
            moves: Vec::new()
        }
    }

    fn analyse_position(&mut self) {
        let fen = fen::epd(&self.pos);
        let score = self.engine.eval_fen(&fen);
        self.scores.push(score);
    }
}

impl Visitor for GameAnalyser {
    type Result = bool;

    fn begin_game(&mut self) {
        self.current_id = "".to_string();
        self.success = true;
        self.scores = Vec::new();
        self.black = Player::empty();
        self.white = Player::empty();
        self.pos = Chess::default();
        self.moves = Vec::new();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
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
            },
            b"White" => {
                self.white.name = std::str::from_utf8(value.as_bytes()).unwrap().to_string();
            },
            b"Black" => {
                self.black.name  = std::str::from_utf8(value.as_bytes()).unwrap().to_string();
            },
            b"WhiteElo" => {
                if let Ok(vs) = std::str::from_utf8(value.as_bytes()) {
                    if let Ok(rating) = vs.to_string().parse::<u32>() {
                        self.white.rating = rating;
                    }
                }
            },
            b"BlackElo" => {
                if let Ok(vs) = std::str::from_utf8(value.as_bytes()) {
                    if let Ok(rating) = vs.to_string().parse::<u32>() {
                        self.black.rating = rating;
                    }
                }
            },
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
                    self.analyse_position();
                }
                Err(_err) => {
                    //eprintln!("error in game {}: {} {}", self.games, err, san_plus);
                    self.success = false;
                }
            }
            self.moves.push(san_plus.to_string());
        }
    }

    fn end_game(&mut self) -> Self::Result {
        false
    }
}
