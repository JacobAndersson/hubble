mod stockfish;
mod opening_tree;

use stockfish::{Stockfish, Scores};
use opening_tree::parse_common_moves;

use std::{thread, time};
use pgn_reader::{Visitor, Skip, BufferedReader, RawHeader, SanPlus};
use shakmaty::{CastlingMode, Chess, Position, fen, uci::Uci, Move};
use shakmaty::fen::Fen;
use std::io;

use std::fs::File;

struct GameAnalyser {
    engine: Stockfish,
    current_id: String, //current Game id
    scores: Vec<Option<f32>>,
    success: bool,
    pos: Chess
}


impl GameAnalyser {
    fn new() -> Self {
        Self {
            engine: Stockfish::new().unwrap(),
            current_id: String::from(""),
            scores: Vec::new(),
            success: true,
            pos: Chess::default()
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
        self.pos = Chess::default();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"FEN" {
            let fen = match Fen::from_ascii(value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    //eprintln!("invalid fen header in game {}: {} ({:?})", self.games, err, value);
                    self.success = false;
                    return;
                },
            };

            self.pos = match fen.position(CastlingMode::Chess960) {
                Ok(pos) => pos,
                Err(err) => {
                    //eprintln!("illegal fen header in game {}: {} ({})", self.games, err, fen);
                    self.success = false;
                    return;
                },
            };
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
                    println!("{:?}", m);
                    self.pos.play_unchecked(&m);
                    self.analyse_position();

                },
                Err(err) => {
                    //eprintln!("error in game {}: {} {}", self.games, err, san_plus);
                    self.success = false;
                },
            }
        }
    }

    fn end_game(&mut self) -> Self::Result {
        self.success
    }
}

fn play_through_match() -> io::Result<()>{
    let mut success = true;
    let file = File::open("test2.pgn")?;
    let mut reader = BufferedReader::new(file);

    let mut validator = GameAnalyser::new();
    while let Some(ok) = reader.read_game(&mut validator)? {
        success &= ok;
    }

    Ok(())
}

fn main() {
    /*
    let mut engine = Stockfish::new().unwrap();
    engine.flush();
    let eval = engine.eval_fen("5k2/p5pp/1b2n3/1N3n2/7B/P4P2/5P1P/4R1K1 w - - 1 32");
    println!("{:?}", eval);
    */

    //parse_common_moves();


    play_through_match();
}
