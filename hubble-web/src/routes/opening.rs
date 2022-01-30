use rocket::http::Status;
use rocket::State;

use hubble_db::models::{get_openings, Opening};
use hubble_db::{pg_pool_handler, PgPool};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use hubble::analysis::opening_tree::MoveEntry;
use hubble::lichess;
use hubble::lichess::AnalysisErrors;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpeningRequest {
    moves: Vec<String>,
    eco: String,
}

#[post("/opening", format = "json", data = "<opening>")]
pub async fn find_opening(
    dbpool: &State<PgPool>,
    opening: Json<OpeningRequest>,
) -> Result<Json<Opening>, Status> {
    let conn = pg_pool_handler(dbpool).unwrap();
    let moves = &opening.moves.to_vec();

    if let Some(openings) = get_openings(&conn, &opening.eco) {
        let mut longest_match = 0;
        let mut longest_opening = None;

        for op in openings {
            let length = hubble::analysis::opening::match_length(&op, &moves);

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
