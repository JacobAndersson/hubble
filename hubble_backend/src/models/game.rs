use serde::{Serialize, Deserialize};

use diesel::{Queryable};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::games;
use serde_json;

#[derive(Insertable, Queryable, Deserialize, Identifiable, Serialize, Debug)]
#[table_name="games"]
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
}
    
impl GameRaw {
    fn read_json(key: serde_json::Value) -> Vec<String>{
        match key.get("data") {
            Some(data) => {
               match serde_json::from_str::<Vec<String>>(&data.to_string()) {
                    Ok(s) => s,
                    Err(_) => Vec::new()
               }
            },
            None => Vec::new()
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
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Game { //TODO REMOVE STRING use &str instead with lifetime
    pub id: String,
    pub opening_id: Option<String>,
    pub moves: Vec<String>,
    pub scores: Vec<String>,
    pub white: String,
    pub black: String,
    pub white_rating: Option<i32>,
    pub black_rating: Option<i32>,
    pub winner: Option<String>,
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
            winner: None
        }
    }
}

pub fn get_games(user_id: &str, conn: &PgConnection) -> Vec<Game> {
    let raws = games::table.filter(games::white.eq(user_id)).or_filter(games::black.eq(user_id)).load::<GameRaw>(conn).expect("ERROR LOADING");
    let mut games = Vec::new();

    for x in raws {
        games.push(x.to_game());
    }

    games
}
