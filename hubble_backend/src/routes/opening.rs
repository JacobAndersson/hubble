use db::PgPool;
use rocket::http::Status;
use rocket::State;

use crate::db;
use crate::models::{get_openings, Opening};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::analysis::opening_tree::MoveEntry;
use crate::lichess;
use crate::lichess::AnalysisErrors;
use std::collections::HashMap;

use shakmaty::{san::San, Chess, Position};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpeningRequest {
    moves: Vec<String>,
    eco: String,
}

fn match_length(opening: &Opening, moves: Vec<String>) -> usize {
    let cleaned = opening.pgn.replace(".", "");
    let splits = cleaned.split(" ");
    let mut length = 0;
    let mut idx = 0;

    let mut board = Chess::default();

    for mv in splits {
        if mv.len() == 1 {
            continue;
        }
        if let Ok(san) = mv.parse::<San>() {
            if let Ok(parsed_move) = san.to_move(&board) {
                board.play_unchecked(&parsed_move);
                if moves[idx] == parsed_move.to_string().replace("-", "") {
                    length += 1;
                    idx += 1;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    length
}

#[post("/opening", format = "json", data = "<opening>")]
pub async fn find_opening(
    dbpool: &State<PgPool>,
    opening: Json<OpeningRequest>,
) -> Result<Json<Opening>, Status> {
    let conn = db::pg_pool_handler(dbpool).unwrap();
    let moves = &opening.moves;

    if let Some(openings) = get_openings(&conn, &opening.eco) {
        let mut longest_match = 0;
        let mut longest_opening = None;

        for op in openings {
            let length = match_length(&op, moves.to_vec());

            if length > longest_match {
                longest_match = length;
                longest_opening = Some(op);
            }
        }

        match longest_opening {
            Some(o) => Ok(Json(o)),
            None => Err(Status::NotFound),
        }
    } else {
        Err(Status::NotFound)
    }
}

#[get("/opening/<player>")]
pub async fn opening_player(player: &str) -> Result<String, Status> {
    let res: Result<HashMap<String, Vec<MoveEntry>>, AnalysisErrors> =
        lichess::opening_player(player).await;

    match res {
        Ok(opening) => match serde_json::to_string(&opening) {
            Ok(s) => Ok(s),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
