use serde::{Deserialize, Serialize};
use serde_json::json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;

use crate::schema::games;
use serde_json;

#[derive(Insertable, Queryable, Deserialize, Identifiable, Serialize, Debug)]
#[table_name = "games"]
pub struct GameRaw {
    id: String,
    opening_id: Option<String>,
    moves: serde_json::Value,
    scores: serde_json::Value,

    white: String,
    black: String,
    white_rating: Option<i32>,
    black_rating: Option<i32>,
    winner: Option<String>,
    middle_game: Option<i32>,
    end_game: Option<i32>
}

impl GameRaw {
    fn read_json(key: serde_json::Value) -> Vec<String> {
        match key.get("data") {
            Some(data) => match serde_json::from_str::<Vec<String>>(&data.to_string()) {
                Ok(s) => s,
                Err(_) => Vec::new(),
            },
            None => Vec::new(),
        }
    }

    pub fn to_game(self) -> Game {
        let scores = GameRaw::read_json(self.scores);
        let moves = GameRaw::read_json(self.moves);

        Game {
            id: self.id,
            opening_id: self.opening_id,
            moves,
            scores,
            winner: self.winner,
            white: self.white,
            black: self.black,
            white_rating: self.white_rating,
            black_rating: self.black_rating,
            end_game: self.end_game,
            middle_game: self.middle_game 
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Game {
    //TODO REMOVE STRING use &str instead with lifetime
    pub id: String,
    pub opening_id: Option<String>,
    pub moves: Vec<String>,
    pub scores: Vec<String>,
    pub white: String,
    pub black: String,
    pub white_rating: Option<i32>,
    pub black_rating: Option<i32>,
    pub winner: Option<String>,
    pub middle_game: Option<i32>,
    pub end_game: Option<i32>
}

impl Game {
    pub fn empty() -> Self {
        Self {
            id: "".to_string(),
            opening_id: None,
            moves: Vec::new(),
            scores: Vec::new(),
            white: "".to_string(),
            black: "".to_string(),
            white_rating: None,
            black_rating: None,
            winner: None,
            middle_game: None,
            end_game: None

        }
    }

    pub fn into_raw(self) -> GameRaw {
        let raw_moves = json!({"data": self.moves});
        let raw_scores = json!({"data": self.scores});

        GameRaw {
            id: self.id,
            opening_id: self.opening_id,

            moves: raw_moves,
            scores: raw_scores,

            white: self.white,
            black: self.black,
            white_rating: self.white_rating,
            black_rating: self.black_rating,
            winner: self.winner,

            end_game: self.end_game,
            middle_game: self.middle_game
        }
    }
}

#[allow(dead_code)]
pub fn get_games_player(user_id: &str, conn: &PgConnection) -> Vec<Game> {
    let raws = games::table
        .filter(games::white.eq(user_id))
        .or_filter(games::black.eq(user_id))
        .load::<GameRaw>(conn)
        .expect("ERROR LOADING");
    let mut games = Vec::new();

    for x in raws {
        games.push(x.to_game());
    }

    games
}

pub fn get_games(conn: &PgConnection) -> Vec<Game> {
    let raws = games::table.load::<GameRaw>(conn).expect("ERROR LOADING");

    let mut games = Vec::new();

    for x in raws {
        games.push(x.to_game());
    }

    games
}

pub fn save_games(
    games: Vec<Game>,
    conn: &PgConnection,
) -> Result<Vec<Game>, diesel::result::Error> {
    let raw_games = games
        .into_iter()
        .map(|x| x.into_raw())
        .collect::<Vec<GameRaw>>();
    match diesel::insert_into(games::table)
        .values(&raw_games)
        .get_results(conn)
    {
        Ok(returning) => Ok(returning
            .into_iter()
            .map(|x: GameRaw| x.to_game())
            .collect()),
        Err(e) => Err(e),
    }
}

pub fn save_game(game: Game, conn: &PgConnection) -> Result<Game, diesel::result::Error> {
    let raw = game.into_raw();

    match diesel::insert_into(games::table)
        .values(raw)
        .get_result::<GameRaw>(conn)
    {
        Ok(ret) => Ok(ret.to_game()),
        Err(e) => Err(e),
    }
}

pub fn get_game(id: &str, conn: &PgConnection) -> Option<Game> {
    match games::table.filter(games::id.eq(id)).first::<GameRaw>(conn) {
        Ok(ret) => Some(ret.to_game()),
        Err(_) => None,
    }
}
