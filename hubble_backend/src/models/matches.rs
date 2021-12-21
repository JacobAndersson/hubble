use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::pg::types::sql_types::Jsonb;
use diesel::pg::Pg;
use diesel::types::FromSql;

use crate::schema::matches;
use serde_json;

#[derive(Queryable)]
pub struct MatchRaw {
    id: String,
    player_id: String,
    opening_id: String,
    moves: serde_json::Value,
    scores: serde_json::Value,
    winner: String,
    player_rating: Option<i32>,
    oponnent_rating: Option<i32>,
    is_white: bool
}
    
impl MatchRaw {
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

    pub fn to_match(self) -> Match {
        let scores = MatchRaw::read_json(self.scores);
        let moves = MatchRaw::read_json(self.moves);

        Match {
            id: self.id,
            player_id: self.player_id,
            moves,
            scores,
            winner: self.winner,
            player_rating: self.player_rating,
            oponnent_rating: self.oponnent_rating,
            is_white: self.is_white
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Match {
    id: String,
    player_id: String,
    moves: Vec<String>,
    scores: Vec<String>,
    winner: String,
    player_rating: Option<i32>,
    oponnent_rating: Option<i32>,
    is_white: bool
}

pub fn get_matches(user_id: &str, conn: &PgConnection) -> Vec<Match> {
    let raws = matches::table.load::<MatchRaw>(conn).expect("ERROR LOADING");
    let mut matches = Vec::new();
    for x in raws {
        matches.push(x.to_match());
    }
    matches
}
