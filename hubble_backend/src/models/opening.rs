use serde::{Deserialize, Serialize};
use serde_json::json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;

use crate::schema::openings;
use serde_json;

#[derive(Insertable, Queryable, Deserialize, Serialize, Debug)]
#[table_name = "openings"]
pub struct Opening {
    eco: String,
    name: String,
    pgn: String,
}

impl Opening {
    pub fn new(eco: String, name: String, pgn: String) -> Self {
        Self {
            eco,
            name, 
            pgn,
        }
    }
}

pub fn insert_opening(conn: &PgConnection, opening: Opening) -> Result<Opening, diesel::result::Error> {
    diesel::insert_into(openings::table).values(opening).get_result::<Opening>(conn)
}

pub fn insert_openings(conn: &PgConnection, openings: Vec<Opening>) -> Result<Vec<Opening>, diesel::result::Error> {
    diesel::insert_into(openings::table).values(openings).get_results::<Opening>(conn)
}
