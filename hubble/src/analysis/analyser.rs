use async_trait::async_trait;
use hubble_db::models::game::{Game, Blunders};
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
        .go_opt("nodes", 1000 * 1000);

    let result = engine.go(analysis_job).await.unwrap();
    match result.ai.score {
        Score::Cp(value) => value,
        Score::Mate(mvs_mate) => 100_000 - mvs_mate,
    }
}

fn group_blunders_by_phase(
    blunders: &Vec<usize>,
    middle_game: Option<i32>,
    end_game: Option<i32>,
) -> Blunders { 
    let mut grouped_blunders = Blunders::empty();

    for idx in blunders {
        if Some(*idx as i32) < middle_game {
            grouped_blunders.opening.push(*idx as i32);
        } else if Some(*idx as i32) >= middle_game && Some(*idx as i32) < end_game {
            grouped_blunders.middle_game.push(*idx as i32);
        } else {
            grouped_blunders.end_game.push(*idx as i32);
        }
    }

    grouped_blunders
}

pub struct GameAnalyser {
    engine: Arc<UciEngine>,
    success: bool,
    pos: Chess,
    pub game: Game,
    move_counter: usize,
    last_score: i32,
    blunders: Vec<usize>,
}

impl GameAnalyser {
    pub async fn new() -> Self {
        Self {
            engine: get_engine().await,
            success: true,
            pos: Chess::default(),
            game: Game::empty(),
            move_counter: 0,
            last_score: 0,
            blunders: Vec::new(),
        }
    }

    fn is_end_game(&mut self) {
        let board = self.pos.board();
        let kings = board.kings();
        let pawns = board.pawns();
        let pieces = board.occupied();
        let num_minor_major = (pieces & !kings & !pawns).count();

        if num_minor_major <= 6 {
            self.game.end_game = Some(self.move_counter as i32);
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
            self.game.middle_game = Some(self.move_counter as i32);
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
        self.move_counter = 0;
        self.last_score = 0;
        self.blunders = Vec::new();
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

                    if self.game.middle_game.is_none() {
                        self.is_middle_game()
                    } else if self.game.end_game.is_none() {
                        self.is_end_game();
                    }

                    if self.last_score != 0 {
                        let score_diff = self.last_score - score;
                        let relative_score = (score as f64 / self.last_score as f64).abs();
                        if score.abs() > 100
                            && (relative_score > 2.3 && score_diff.abs() > 150
                                || relative_score < 0.5 && score_diff.abs() > 80)
                        {
                            self.blunders.push(self.move_counter);
                        }
                    }

                    self.last_score = score;
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
        let grouped =
            group_blunders_by_phase(&self.blunders, self.game.middle_game, self.game.end_game);
        println!("Blunders at {:?}", grouped);
        self.game.blunders = grouped;
        false
    }
}
