#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate rand;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_migrations::RunMigrationsError;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;
pub mod galaxy_objects;

mod tools;

use self::schema::types::GalaxyObjectType;

embed_migrations!();

pub fn run_migrations(
    connection: &PgConnection,
    out: &mut std::io::Stdout,
) -> Result<(), RunMigrationsError> {
    embedded_migrations::run_with_output(connection, out)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::*;