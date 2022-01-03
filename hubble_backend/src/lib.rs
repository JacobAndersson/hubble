pub mod analysis;
pub mod db;
mod lichess;
pub mod models;
mod schema;
mod stockfish;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;
