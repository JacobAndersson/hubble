use hubble_db::{pg_pool_handler, PgPool};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use hubble::analysis::blunder::find_blunder;
use hubble_db::models::game::get_game;

#[get("/blunder/<id>")]
pub fn blunder(
    dbpool: &State<PgPool>,
    id: &str,
) -> Result<Json<Vec<(usize, String)>>, Status> {
    let conn = pg_pool_handler(dbpool).unwrap();
    match get_game(id, &conn) {
        Some(game) => {
            let blunders = find_blunder(&game);
            Ok(Json(blunders))
        }
        None => Err(Status::NotFound),
    }
}
