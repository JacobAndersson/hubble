mod analyser;
mod lichess;
mod opening_tree;
mod stockfish;
mod player;
mod models;
mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate rocket;
use crate::opening_tree::MoveEntry;

use crate::lichess::AnalysisErrors;
use rocket::http::Status;

use std::collections::HashMap;

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

#[get("/analyse/player/<player>")]
async fn analyse_player(player: &str) -> &str{
    lichess::analyse_player(player).await;
    return "TESTING";
}

#[get("/opening/<player>")]
async fn opening(player: &str) -> Result<String, Status> {
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![analyse, opening, analyse_player])
}
