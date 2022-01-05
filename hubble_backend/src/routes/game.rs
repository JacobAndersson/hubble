use db::PgPool;
use rocket::http::Status;
use rocket::State;

use crate::{db, models::game};

#[get("/games")]
pub async fn games(dbpool: &State<PgPool>) -> Result<String, Status> {
    let conn = db::pg_pool_handler(dbpool).unwrap();
    match serde_json::to_string(&game::get_games(&conn)) {
        Ok(s) => Ok(s),
        Err(_) => Err(Status::InternalServerError),
    }
}
