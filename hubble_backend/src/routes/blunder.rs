use db::PgPool;
use rocket::http::Status;
use rocket::State;

use crate::analysis::blunder::find_blunder;
use crate::{db, models::game};

#[get("/blunder/<id>")]
pub async fn blunder(dbpool: &State<PgPool>, id: &str) -> Result<String, Status> {
    let conn = db::pg_pool_handler(dbpool).unwrap();
    match game::get_game(id, &conn) {
        Some(game) => match serde_json::to_string(&find_blunder(&game)) {
            Ok(blunders) => Ok(blunders),
            Err(_) => Err(Status::InternalServerError),
        },
        None => Err(Status::NotFound),
    }
}
