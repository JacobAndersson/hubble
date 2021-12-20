use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::fen::Fen;
use shakmaty::{fen, CastlingMode, Chess, Position};
use std::fs::File;
use std::io;

use crate::Stockfish;

pub struct GameAnalyser {
    engine: Stockfish,
    current_id: String, //current Game id
    pub scores: Vec<Option<f32>>,
    success: bool,
    pos: Chess,
    pub black: String,
    pub white: String,
}

impl GameAnalyser {
    pub fn new() -> Self {
        Self {
            engine: Stockfish::new().unwrap(),
            current_id: String::from(""),
            scores: Vec::new(),
            success: true,
            pos: Chess::default(),
            black: String::from(""),
            white: String::from(""),
        }
    }

    fn analyse_position(&mut self) {
        let fen = fen::epd(&self.pos);
        let score = self.engine.eval_fen(&fen);
        println!("{:?}", &score);
        self.scores.push(score);
    }
}

impl Visitor for GameAnalyser {
    type Result = bool;

    fn begin_game(&mut self) {
        self.current_id = "".to_string();
        self.success = true;
        self.scores = Vec::new();
        self.black = "".to_string();
        self.white = "".to_string();
        self.pos = Chess::default();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"FEN" {
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
        } else if key == b"White" {
            self.white = std::str::from_utf8(value.as_bytes()).unwrap().to_string();
        } else if key == b"Black" {
            self.black = std::str::from_utf8(value.as_bytes()).unwrap().to_string();
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
                    println!("{}", &m);
                    self.analyse_position();
                }
                Err(err) => {
                    //eprintln!("error in game {}: {} {}", self.games, err, san_plus);
                    self.success = false;
                }
            }
        }
    }

    fn end_game(&mut self) -> Self::Result {
        //self.success
        false
    }
}

pub fn play_through_match() -> io::Result<()> {
    let mut success = true;
    let file = File::open("test2.pgn")?;
    let mut reader = BufferedReader::new(file);

    let mut validator = GameAnalyser::new();
    while let Some(ok) = reader.read_game(&mut validator)? {
        success &= ok;
        let game = &validator.scores;
    }

    Ok(())
}
