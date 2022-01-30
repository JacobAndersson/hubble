mod routes;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use crate::routes::*;

#[launch]
fn rocket() -> _ {
    dotenv::from_filename("../.env").ok();

    rocket::build()
        .manage(hubble_db::establish_connection())
        .mount(
            "/api",
            routes![
                analyse::analyse,
                analyse::analyse_player,
                blunder::blunder,
                crate::routes::game::games,
                opening::opening_player,
                opening::find_opening
            ],
        )
}
