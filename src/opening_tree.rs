use std::collections::HashMap;
use std::fs::File;
use std::io;

use shakmaty::fen::Fen;
use shakmaty::{fen, uci::Uci, CastlingMode, Chess, Move, Position};

use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};

use serde::Serialize;

fn insert_move(moves: &mut Vec<MoveEntry>, mv: &str) {
    let mut found = false;

    for entry in moves.iter_mut() {
        if entry.mv == mv {
            entry.n += 1;
            found = true;
        }
    }

    if !found {
        moves.push(MoveEntry {
            mv: mv.to_string(),
            n: 1,
        });
    }
}

fn find_most_popular(moves: &[MoveEntry]) -> String {
    let mut n_max = 0;
    let mut pop = "e2e4";

    for m in moves {
        if m.n > n_max {
            pop = &m.mv;
            n_max = m.n;
        }
    }
    pop.to_string()
}

#[derive(Debug, Serialize)]
pub struct MoveEntry {
    mv: String,
    n: usize,
}

pub struct OpeningTree {
    games: usize,
    pos: Chess,
    success: bool,
    pub move_stat: HashMap<String, Vec<MoveEntry>>,
}

impl OpeningTree {
    pub fn new() -> Self {
        Self {
            games: 0,
            pos: Chess::default(),
            success: true,
            move_stat: HashMap::new(),
        }
    }

    fn count_position(&mut self, mv: &Move) {
        let fen = fen::epd(&self.pos);
        let uci = mv.to_uci(CastlingMode::Standard);
        match self.move_stat.get_mut(&fen) {
            Some(entries) => insert_move(entries, &format!("{}", uci)),
            None => {
                let entry = vec![MoveEntry {
                    mv: uci.to_string(),
                    n: 1,
                }];
                self.move_stat.insert(fen, entry);
            }
        };
    }
}

impl Visitor for OpeningTree {
    type Result = bool;

    fn begin_game(&mut self) {
        self.games += 1;
        self.pos = Chess::default();
        self.success = true;
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"FEN" {
            let fen = match Fen::from_ascii(value.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    eprintln!(
                        "invalid fen header in game {}: {} ({:?})",
                        self.games, err, value
                    );
                    self.success = false;
                    return;
                }
            };

            self.pos = match fen.position(CastlingMode::Chess960) {
                Ok(pos) => pos,
                Err(err) => {
                    eprintln!(
                        "illegal fen header in game {}: {} ({})",
                        self.games, err, fen
                    );
                    self.success = false;
                    return;
                }
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
                    self.count_position(&m);
                    self.pos.play_unchecked(&m)
                }
                Err(err) => {
                    eprintln!("error in game {}: {} {}", self.games, err, san_plus);
                    self.success = false;
                }
            }
        }
    }

    fn end_game(&mut self) -> Self::Result {
        self.success
    }
}

pub fn parse_common_moves() -> io::Result<()> {
    let mut success = true;

    let file = File::open("test2.pgn")?;
    let mut reader = BufferedReader::new(file);

    let mut validator = OpeningTree::new();
    while let Some(ok) = reader.read_game(&mut validator)? {
        success &= ok;
    }

    if !success {
        ::std::process::exit(1);
    }

    let stats = validator.move_stat;
    let mut pos = Chess::default();

    for _ in 0..80 {
        let fen = fen::epd(&pos);
        match stats.get(&fen) {
            Some(ms) => {
                let most_popular = find_most_popular(ms);
                let m: Uci = most_popular.parse().unwrap();
                println!("{}", m);
                pos.play_unchecked(&m.to_move(&pos).unwrap());
            }
            None => {
                break;
            }
        }
    }

    Ok(())
}
