use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;

use crate::schema::openings;

#[derive(Insertable, Queryable, Deserialize, Serialize, Debug)]
#[table_name = "openings"]
pub struct Opening {
    pub id: i32, //Means id needs to be set for insert. Not ideal but fine since insert is only done ones.
    pub eco: String,
    pub name: String,
    pub pgn: String,
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

#[allow(dead_code)]
pub fn insert_openings(
    conn: &PgConnection,
    openings: Vec<Opening>,
) -> Result<Vec<Opening>, diesel::result::Error> {
    diesel::insert_into(openings::table)
        .values(openings)
        .get_results::<Opening>(conn)
}

pub fn get_openings(conn: &PgConnection, eco: &str) -> Option<Vec<Opening>> {
    match openings::table
        .filter(openings::eco.eq(eco))
        .load::<Opening>(conn)
    {
        Ok(ret) => Some(ret),
        Err(_) => None,
    }
}

pub fn get_all_openings(conn: &PgConnection) -> Vec<Opening> {
    match openings::table.load::<Opening>(conn) {
        Ok(ret) => ret,
        Err(_) => vec![],
    }
}
