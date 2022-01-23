use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Queryable;

use crate::schema::users;

#[derive(Insertable, Queryable, Deserialize, Identifiable, Serialize, Debug)]
#[table_name = "users"]
pub struct User {
    id: String,
    rating: Option<i32>,
}

pub fn get_user(user_id: &str, conn: &PgConnection) -> User {
    users::table
        .filter(users::id.eq(user_id))
        .first::<User>(conn)
        .expect("ERROR LOADING")
}

pub fn get_users(conn: &PgConnection) -> Vec<User> {
    users::table.load::<User>(conn).expect("ERROR LOADING")
}

pub fn insert_user(user_id: String, rating: i32, conn: &PgConnection) -> QueryResult<User> {
    let user = User {
        id: user_id,
        rating: Some(rating),
    };
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
}

pub fn delete_user(user_id: &str, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(users::table.find(user_id)).execute(conn)
}
