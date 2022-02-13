use async_trait::async_trait;
use hubble_db::models::game::Game;
use pgn_reader::{AsyncVisitor, RawHeader, SanPlus, Skip};
use shakmaty::Rank;
use shakmaty::{
    bitboard::Bitboard, fen::Fen, uci::Uci, CastlingMode, Chess, Move, Position, Setup,
};
use std::sync::Arc;
use uciengine::analysis::Score;
use uciengine::uciengine::{GoJob, UciEngine};

async fn get_engine() -> Arc<UciEngine> {
    let engine = UciEngine::new("./stockfish");

    let setup_job = GoJob::new().uci_opt("Hash", 8192).uci_opt("Threads", 10);
    let _result = engine.check_ready(setup_job).await.unwrap();
    engine
}

pub async fn eval_move(pos: &Chess, m: &Move, engine: &Arc<UciEngine>) -> i32 {
    let fen = Fen::from_setup(pos);
    let uci_move = Uci::from_move(m, CastlingMode::Standard);
    let analysis_job = GoJob::new()
        .pos_fen(fen)
        .pos_moves(uci_move.to_string())
        .go_opt("nodes", 10 * 1000);

    let result = engine.go(analysis_job).await.unwrap();
    match result.ai.score {
        Score::Cp(value) => value,
        Score::Mate(mvs_mate) => 100_000 - mvs_mate,
    }
}

pub struct GameAnalyser {
    engine: Arc<UciEngine>,
    success: bool,
    pos: Chess,
    pub game: Game,
    middle_game_start: Option<usize>,
    end_game_start: Option<usize>,
    move_counter: usize,
}

impl GameAnalyser {
    pub async fn new() -> Self {
        Self {
            engine: get_engine().await,
            success: true,
            pos: Chess::default(),
            game: Game::empty(),
            middle_game_start: None,
            end_game_start: None,
            move_counter: 0,
        }
    }

    fn is_end_game(&mut self) {
        let board = self.pos.board();
        let kings = board.kings();
        let pawns = board.pawns();
        let pieces = board.occupied();
        let num_minor_major = (pieces & !kings & !pawns).count();

        if num_minor_major <= 6 {
            self.end_game_start = Some(self.move_counter);
        }
    }

    fn is_middle_game(&mut self) {
        let board = self.pos.board();
        let kings = board.kings();
        let pawns = board.pawns();
        let pieces = board.occupied();

        let num_minor_major = (pieces & !kings & !pawns).count();
        let white_backrank_count = (pieces & Bitboard::from_rank(Rank::First)).count();
        let black_backrank_count = (pieces & Bitboard::from_rank(Rank::Eighth)).count();

        if num_minor_major <= 10 || white_backrank_count < 4 || black_backrank_count < 4 {
            self.middle_game_start = Some(self.move_counter);
        }
    }
}

#[async_trait(?Send)]
impl AsyncVisitor for GameAnalyser {
    type Result = bool;

    fn begin_game(&mut self) {
        self.success = true;
        self.pos = Chess::default();
        self.game = Game::empty();
        self.middle_game_start = None;
        self.end_game_start = None;
        self.move_counter = 0;
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
                    self.pos.play_unchecked(&m);

                    if self.middle_game_start.is_none() {
                        self.is_middle_game()
                    } else if self.end_game_start.is_none() {
                        self.is_end_game();
                    }
                    self.game.scores.push(score.to_string());
                    self.move_counter += 1;
                }
                Err(_err) => {
                    self.success = false;
                }
            }
        }
    }

    async fn end_game(&mut self) -> Self::Result {
        println!(
            "middle game start {:?} end game start {:?}",
            self.middle_game_start, self.end_game_start
        );
        false
    }
}
