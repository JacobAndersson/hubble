use crate::analysis::opening::match_length_sans;
use hubble_db::models::{get_all_openings, Opening};
use hubble_db::PgConnection;
use pgn_reader::{RawHeader, SanPlus, Skip, Visitor};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct OpeningResult {
    pub won: u32,
    pub tie: u32,
    pub lost: u32,
}

impl OpeningResult {
    fn new() -> Self {
        Self {
            won: 0,
            tie: 0,
            lost: 0,
        }
    }
}

impl fmt::Display for OpeningResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.won, self.tie, self.lost)
    }
}

#[derive(Debug)]
pub struct OpeningCounter {
    pub openings: HashMap<String, OpeningResult>,
    ecos: HashMap<String, Vec<Opening>>,
    current_eco: String,
    current_moves: Vec<String>,
    is_white: bool,
    result: String,
    player: String,
    white_only: Option<bool>, //true - only games where player is white is analysed, false - black, none - both
}

impl OpeningCounter {
    pub fn new(conn: &PgConnection, player: String, white_only: Option<bool>) -> Self {
        let mut ecos = HashMap::<String, Vec<Opening>>::new();
        let openings = get_all_openings(conn);
        for op in openings {
            if let Some(o) = ecos.get_mut(&op.eco) {
                o.push(op);
            } else {
                ecos.insert(op.eco.clone(), vec![op]);
            }
        }

        Self {
            openings: HashMap::new(),
            current_eco: String::from(""),
            current_moves: Vec::new(),
            ecos,
            player,
            is_white: true,
            result: String::from(""),
            white_only,
        }
    }
}

impl Visitor for OpeningCounter {
    type Result = bool;

    fn begin_game(&mut self) {
        self.current_moves = Vec::new();
        self.result = String::from("");
        self.is_white = true;
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if let Ok(value_str) = std::str::from_utf8(value.as_bytes()) {
            match key {
                b"ECO" => {
                    self.current_eco = value_str.to_string();
                }
                b"White" => {
                    self.is_white = value_str == self.player;
                }
                b"Result" => {
                    self.result = value_str.to_string();
                }
                _ => {}
            }
        }
    }

    fn end_headers(&mut self) -> Skip {
        match self.white_only {
            Some(val) => Skip(val == self.is_white),
            None => Skip(false),
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn san(&mut self, san_plus: SanPlus) {
        self.current_moves.push(san_plus.to_string());
    }

    fn end_game(&mut self) -> Self::Result {
        if self.current_moves.len() == 0 {
            return false;
        }

        if let Some(relevant_openings) = self.ecos.get(&self.current_eco) {
            let mut longest = 0;
            let mut opening_name = "";
            for op in relevant_openings {
                let length = match_length_sans(op, &self.current_moves);
                if length > longest {
                    longest = length;
                    opening_name = &op.name;
                }
            }

            if opening_name != "" {
                let opening_count = self
                    .openings
                    .entry(opening_name.to_string())
                    .or_insert(OpeningResult::new());
                let white_won = self.result == "1-0";
                if self.result == "1/2-1/2" {
                    opening_count.tie += 1;
                } else if white_won == self.is_white {
                    opening_count.won += 1;
                } else {
                    opening_count.lost += 1;
                }
            }
        }
        false
    }
}
