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
fn generate_star_sector_finishes_without_errors() {
    let connection = test_connection();
    generate_star_sector(&connection, 1f32, 1f32, None).unwrap();
}

#[test]
fn generate_star_sectors_creates_stars() {
    let connection = test_connection();
    let star_amount: usize = 5; // Less than 10 - threshold

    use tg_space_game::models::*;

    let sector = generate_star_sector(&connection, star_amount as f32, 1f32, None)
        .expect("Error generating star sector");

    use tg_space_game::schema::star_systems::dsl::*;

    let systems = star_systems
        .filter(sector_id.eq(sector.id))
        .load::<StarSystem>(&connection)
        .expect("Error loading star systems");

    assert_eq!(systems.len(), star_amount);
}

#[test]
fn generate_star_sectors_creates_futures() {
    let connection = test_connection();
    let star_amount: f32 = 100f32; // More than 10 - threshold

    let sector = generate_star_sector(&connection, star_amount, 1f32, None)
        .expect("Error generating star sector");

    let futures = get_star_sector_children_futures(&connection, &sector)
        .expect("Error loading star sector futures");

    assert_eq!(futures.len(), 10); // Constant sub_amount
}

#[test]
fn fulfill_star_sector_future_finishes_without_errors() {
    let connection = test_connection();
    
    let sector = generate_star_sector(&connection, 100f32, 1f32, None)
        .expect("Error generating star sector");

    let futures = get_star_sector_children_futures(&connection, &sector)
        .expect("Error loading star sector futures");

    fulfill_star_sector_future(&connection, futures[0].id)
        .expect("Error fulfilling star sector future");
}