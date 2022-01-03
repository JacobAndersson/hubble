use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;

use crate::schema::openings;

#[derive(Insertable, Queryable, Deserialize, Serialize, Debug)]
#[table_name = "openings"]
pub struct Opening {
    id: i32, //Means id needs to be set for insert. Not ideal but fine since insert is only done ones.
    eco: String,
    name: String,
    pgn: String,
}

impl Opening {
    pub fn new(id: i32, eco: String, name: String, pgn: String) -> Self {
        Self { id, eco, name, pgn }
    }
}

pub fn insert_opening(
    conn: &PgConnection,
    opening: Opening,
) -> Result<Opening, diesel::result::Error> {
    diesel::insert_into(openings::table)
        .values(opening)
        .get_result::<Opening>(conn)
}

pub fn insert_openings(
    conn: &PgConnection,
    openings: Vec<Opening>,
) -> Result<Vec<Opening>, diesel::result::Error> {
    diesel::insert_into(openings::table)
        .values(openings)
        .get_results::<Opening>(conn)
}

pub fn get_opening(conn: &PgConnection, moves: &str, eco: &str) -> Option<Opening> {
    return match openings::table
        .filter(openings::pgn.eq(moves))
        .filter(openings::eco.eq(eco))
        .first::<Opening>(conn)
    {
        Ok(ret) => Some(ret),
        Err(_) => None,
    };
}
