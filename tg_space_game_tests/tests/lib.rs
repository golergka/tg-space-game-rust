extern crate diesel;
extern crate dotenv;
extern crate tg_space_game;

use self::diesel::*;
use self::dotenv::dotenv;
use std::env;

fn connection() -> PgConnection {
    let database_url = database_url_from_env("PG_DATABASE_URL");
    PgConnection::establish(&database_url).unwrap()
}

fn database_url_from_env(env_var: &str) -> String {
    dotenv().ok();
    match env::var(env_var) {
        Ok(val) => val,
        _ => env::var("DATABASE_URL").expect("DATABASE_URL must be set in order to run tests"),
    }
}

fn test_connection() -> PgConnection {
    let result = connection();
    result.begin_test_transaction().unwrap();
    result
}

use tg_space_game::*;

#[test]
fn generates_star_sector() {
    let connection = test_connection();
    generate_star_sector(&connection, 1f32, 1f32, None).unwrap();
}
