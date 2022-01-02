mod analysis;
mod db;
mod lichess;
mod models;
mod schema;
mod stockfish;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;

use crate::analysis::opening_tree::MoveEntry;
use crate::models::game;

use crate::lichess::AnalysisErrors;
use rocket::http::Status;

use crate::analysis::blunder::find_blunder;
use db::PgPool;
use rocket::State;
use std::collections::HashMap;

#[get("/analyse/match/<id>")]
async fn analyse(_dbpool: &State<PgPool>, id: &str) -> Result<String, Status> {
    let connection = db::pg_pool_handler(_dbpool).unwrap();
    match lichess::analyse_lichess_game(connection, id).await {
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
async fn analyse_player(_dbpool: &State<PgPool>, player: &str) -> Result<String, Status> {
    let connection = db::pg_pool_handler(_dbpool).unwrap();
    match lichess::analyse_player(connection, player).await {
        Ok(games) => match serde_json::to_string(&games) {
            Ok(s) => Ok(s),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
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

#[get("/blunder/<id>")]
async fn blunder(dbpool: &State<PgPool>, id: &str) -> Result<String, Status> {
    let conn = db::pg_pool_handler(dbpool).unwrap();
    match game::get_game(id, &conn) {
        Some(game) => match serde_json::to_string(&find_blunder(&game)) {
            Ok(blunders) => Ok(blunders),
            Err(_) => Err(Status::InternalServerError),
        },
        None => Err(Status::NotFound),
    }
}

#[get("/games")]
async fn games(dbpool: &State<PgPool>) -> Result<String, Status> {
    let conn = db::pg_pool_handler(dbpool).unwrap();
    match serde_json::to_string(&game::get_games(&conn)) {
        Ok(s) => Ok(s),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv::from_filename("../.env").ok();

    rocket::build().manage(db::establish_connection()).mount(
        "/api",
        routes![analyse, opening, analyse_player, blunder, games],
    )
}
