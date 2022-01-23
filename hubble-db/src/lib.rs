#[macro_use] extern crate diesel;

mod db;
mod schema;
pub mod models;
pub use db::*;
