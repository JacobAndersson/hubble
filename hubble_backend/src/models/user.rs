use serde::{Serialize, Deserialize};

use diesel::{Queryable};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::users;

#[derive(Insertable, Queryable, Deserialize, Identifiable, Serialize, Debug)]
#[table_name = "users"]
pub struct User {
    id: String,
    rating: Option<i32>
}

pub fn get_user(user_id: &str, conn: &PgConnection) -> User {
    users::table.filter(users::id.eq(user_id)).first::<User>(conn).expect("ERROR LOADING")
}

pub fn get_users(conn: &PgConnection) -> Vec<User> {
    users::table.load::<User>(conn).expect("ERROR LOADING")
}

pub fn insert_user(user_id: String, rating: i32, conn: &PgConnection) -> QueryResult<User> {
    let user = User {id: user_id, rating: Some(rating) };
    diesel::insert_into(users::table).values(&user).get_result(conn)
}

pub fn delete_user(user_id: &str, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(users::table.find(user_id)).execute(conn)
}

pub fn establish_connection() -> PgConnection {
    use std::env;
    use dotenv::dotenv;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[test]
fn test_db() {
    let connection = establish_connection(); 
    
    let user = insert_user("bror".to_string(), 2000, &connection).unwrap();
    assert_eq!(user.id, "bror");
    assert_eq!(user.rating, Some(2000));

    let user_db  = get_user("bror", &connection);
    assert_eq!(user_db.id, "bror");
    assert_eq!(user_db.rating, Some(2000));

    let users = get_users(&connection);
    assert_eq!(users.len(), 1);

    let delete = delete_user("bror", &connection);
    assert_eq!(delete.unwrap_or(0), 1);
}
