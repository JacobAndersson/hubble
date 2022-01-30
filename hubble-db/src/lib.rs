#[macro_use]
extern crate diesel;

mod db;
pub mod models;
mod schema;
pub use db::*;
pub use diesel::pg::PgConnection;
