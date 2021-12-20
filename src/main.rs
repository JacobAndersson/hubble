mod stockfish;
mod opening_tree;
mod lichess;
mod analyser;

#[macro_use] extern crate rocket;
use stockfish::Stockfish;
use opening_tree::parse_common_moves;

use std::io::{Write, Read, BufRead};
use std::process::{Command, Stdio, Child, ChildStdin};

#[get("/analyse/<id>")]
async fn analyse(id: &str) -> &'static str {
    let scores = lichess::analyse_lichess_game(id).await.unwrap();
    println!("{:?}", scores);
    "Hello, world!"
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
