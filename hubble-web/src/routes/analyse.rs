use hubble_db::models::game::Game;
use hubble_db::{pg_pool_handler, PgPool};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use hubble::lichess;

#[get("/analyse/match/<id>")]
pub async fn analyse(dbpool: &State<PgPool>, id: &str) -> Result<Json<Game>, Status> {
    let connection = pg_pool_handler(dbpool).unwrap();
    match lichess::analyse_lichess_game(connection, id).await {
        Ok(game) => Ok(Json(game)),
        Err(e) => match e {
            lichess::AnalysisErrors::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

#[get("/analyse/player/<player>")]
pub async fn analyse_player(
    dbpool: &State<PgPool>,
    player: String,
) -> Result<Json<Vec<Game>>, Status> {
    let connection = pg_pool_handler(dbpool).unwrap();
    match lichess::analyse_player(connection, &player).await {
        Ok(games) => Ok(Json(games)),
        Err(_) => Err(Status::InternalServerError),
    }
}
