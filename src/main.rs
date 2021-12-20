mod analyser;
mod lichess;
mod opening_tree;
mod stockfish;

#[macro_use]
extern crate rocket;
use opening_tree::parse_common_moves;
use stockfish::Stockfish;

use crate::lichess::{AnalysisErrors, Game};
use rocket::http::Status;
use rocket::response::status;
use serde;

#[get("/analyse/<id>")]
async fn analyse(id: &str) -> Result<String, Status> {
    match lichess::analyse_lichess_game(id).await {
        Ok(game) => match serde_json::to_string(&game) {
            Ok(s) => Ok(s),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(e) => match e {
            AnalysisErrors::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![analyse])
}
