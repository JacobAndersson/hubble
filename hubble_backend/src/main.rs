mod analysis;
mod db;
mod lichess;
mod models;
mod routes;
mod schema;
mod stockfish;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::models::game;

use crate::lichess::AnalysisErrors;
use rocket::http::Status;

use crate::analysis::blunder::find_blunder;
use db::PgPool;
use rocket::State;

use crate::routes::*;

#[launch]
fn rocket() -> _ {
    dotenv::from_filename("../.env").ok();

    rocket::build().manage(db::establish_connection()).mount(
        "/api",
        routes![
            analyse::analyse,
            analyse::analyse_player,
            blunder::blunder,
            crate::routes::game::games,
            opening::opening_player,
            opening::find_opening
        ],
    )
}
