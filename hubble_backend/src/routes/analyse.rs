use db::PgPool;
use rocket::http::Status;
use rocket::State;

use crate::db;
use crate::lichess;

#[get("/analyse/match/<id>")]
pub async fn analyse(_dbpool: &State<PgPool>, id: &str) -> Result<String, Status> {
    let connection = db::pg_pool_handler(_dbpool).unwrap();
    match lichess::analyse_lichess_game(connection, id).await {
        Ok(game) => match serde_json::to_string(&game) {
            Ok(s) => Ok(s),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(e) => match e {
            lichess::AnalysisErrors::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

#[get("/analyse/player/<player>")]
pub async fn analyse_player(_dbpool: &State<PgPool>, player: &str) -> Result<String, Status> {
    let connection = db::pg_pool_handler(_dbpool).unwrap();
    match lichess::analyse_player(connection, player).await {
        Ok(games) => match serde_json::to_string(&games) {
            Ok(s) => Ok(s),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
