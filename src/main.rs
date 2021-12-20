mod stockfish;
mod opening_tree;
mod lichess;
mod analyser;

#[macro_use] extern crate rocket;
use stockfish::Stockfish;
use opening_tree::parse_common_moves;

use rocket::response::status;
use crate::lichess::{Game, AnalysisErrors};
use serde; 
use rocket::http::Status;

#[get("/analyse/<id>")]
async fn analyse(id: &str) -> Result<String, Status> {
    match lichess::analyse_lichess_game(id).await {
        Ok(game) => {
            match serde_json::to_string(&game) {
                Ok(s) => Ok(s),
                Err(_) => Err(Status::InternalServerError)
            }
        },
        Err(e) => {
            match e {
                AnalysisErrors::NotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError)
            }

        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![analyse])
}

/*
#[tokio::main]
async fn main() {
    let scores = lichess::analyse_lichess_game("KrIqzDbw").await.unwrap();
    println!("{:?}", scores);
}
*/
