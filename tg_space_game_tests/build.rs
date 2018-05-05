extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;
extern crate tg_space_game;

use self::diesel::*;
use self::dotenv::dotenv;
use std::env;
use tg_space_game::*;

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

fn main() {
    run_migrations(&connection(), &mut std::io::stdout()).expect("Error running migrations!");
    /*
    let migrations_dir = diesel_migrations::find_migrations_directory().unwrap();
    diesel_migrations::run_pending_migrations_in_directory(
        &connection(),
        &migrations_dir,
        &mut io::sink(),
    ).unwrap();
    */
}
