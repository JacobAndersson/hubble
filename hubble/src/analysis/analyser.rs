use shakmaty::{CastlingMode, Chess, Move, uci::Uci, fen::Fen, Position};
use uciengine::uciengine::{UciEngine, GoJob};
use uciengine::analysis::Score;
use std::sync::Arc;
use pgn_reader::{RawHeader, SanPlus, Skip, AsyncVisitor};
use async_trait::async_trait;
use hubble_db::models::game::Game;
use tokio::time::{sleep, Duration};

async fn is_ready(engine: &Arc<UciEngine>) -> bool {
    let setup_job = GoJob::new()
        .uci_opt("Hash", 1024)
        .uci_opt("Threads", 10);
    println!("{:?}", setup_job.to_commands());

    let result = engine.check_ready(setup_job).await.unwrap();
    println!("ready {:?}", result.is_ready);
    result.is_ready
}

pub async fn eval_move(pos: &Chess, m: &Move, engine: &Arc<UciEngine>) -> i32 {
    let fen = Fen::from_setup(pos);
    let uci_move = Uci::from_move(m, CastlingMode::Standard);
    let analysis_job = GoJob::new()
        .uci_opt("Hash", 8192)
        .uci_opt("Threads", 10)
        .pos_fen(fen)
        .pos_moves(uci_move.to_string())
        .go_opt("nodes", 10 * 1000);

    let result = engine.go(analysis_job).await.unwrap();
    match result.ai.score {
        Score::Cp(value) => value,
        Score::Mate(mvs_mate) => {
            100_000 - mvs_mate
        }
    }
}


pub struct GameAnalyser {
    engine: Arc<UciEngine>,
    success: bool,
    pos: Chess,
    pub game: Game,
}

impl GameAnalyser {
    pub async fn new() -> Self {
        let engine = UciEngine::new("/home/jacob/programming/hubble/hubble/stockfish");

        while !is_ready(&engine).await {
            sleep(Duration::from_millis(1000)).await 
        }

        Self {
            engine,
            success: true,
            pos: Chess::default(),
            game: Game::empty(),
        }
    }

    /*
    fn analyse_position(&mut self) {
        let fen = fen::epd(&self.pos);
        match self.engine.eval_fen(&fen) {
            Some(score) => {
                self.game.scores.push(score.to_string());
            }
            None => {} //handle fail
        }
    }
    */
}

#[async_trait(?Send)]
impl AsyncVisitor for GameAnalyser {
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
                        _ => None,
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

    async fn san(&mut self, san_plus: SanPlus) {
        if self.success {
            match san_plus.san.to_move(&self.pos) {
                Ok(m) => {
                    let uci = m.to_uci(self.pos.castles().mode()).to_string();
                    self.game.moves.push(uci);
                    let score = eval_move(&self.pos, &m, &self.engine).await;
                    println!("score {}", score);
                    self.pos.play_unchecked(&m);
                    self.game.scores.push(score.to_string());
                    //self.analyse_position();
                }
                Err(_err) => {
                    self.success = false;
                }
            }
        }
    }

    async fn end_game(&mut self) -> Self::Result {
        false
    }
}
