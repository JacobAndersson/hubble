use db::PgPool;
use rocket::http::Status;
use rocket::State;

use crate::db;
use crate::models::{get_opening, Opening};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::lichess::AnalysisErrors;
use crate::analysis::opening_tree::MoveEntry;
use std::collections::HashMap;
use crate::lichess;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct OpeningRequest {
    moves: String,
    eco: String,
}

#[post("/opening", format = "json", data = "<opening>")]
pub async fn find_opening(
    dbpool: &State<PgPool>,
    opening: Json<OpeningRequest>,
) -> Result<Json<Opening>, Status> {
    let conn = db::pg_pool_handler(dbpool).unwrap();

    match get_opening(&conn, &opening.moves, &opening.eco) {
        Some(opening) => Ok(Json(opening)),
        None => Err(Status::NotFound),
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
