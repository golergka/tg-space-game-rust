extern crate diesel;
extern crate dotenv;
extern crate tg_space_game;

use self::diesel::*;
use self::dotenv::dotenv;
use std::env;
use tg_space_game::models::*;

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

fn generate_root_with_5_stars(connection: &PgConnection) -> (StarSector, Vec<StarSystem>) {
    use tg_space_game::models::*;
    use tg_space_game::schema::star_systems::dsl::*;

    let sector =
        generate_star_sector(connection, 5f32, 1f32, None).expect("Error generating star sector");

    let systems = star_systems
        .filter(sector_id.eq(sector.id))
        .load::<StarSystem>(connection)
        .expect("Error loading star systems");

    (sector, systems)
}

#[test]
fn generate_star_sectors_creates_stars() {
    let connection = test_connection();
    let star_amount: usize = 5; // Less than 10 - threshold

    let (_, systems) = generate_root_with_5_stars(&connection);

    assert_eq!(systems.len(), star_amount);
}

fn generate_root_with_futures(connection: &PgConnection) -> Vec<StarSectorFuture> {
    let star_amount: f32 = 100f32; // More than 10 - threshold

    let sector = generate_star_sector(&connection, star_amount, 1f32, None)
        .expect("Error generating star sector");

    get_star_sector_children_futures(&connection, &sector)
        .expect("Error loading star sector futures")
}

#[test]
fn generate_star_sectors_creates_futures() {
    let connection = test_connection();

    let futures = generate_root_with_futures(&connection);

    assert_eq!(futures.len(), 10); // Constant sub_amount
}

#[test]
fn fulfill_star_sector_future_finishes_without_errors() {
    let connection = test_connection();

    let future = &generate_root_with_futures(&connection)[0];

    fulfill_star_sector_future(&connection, future.id)
        .expect("Error fulfilling star sector future");
}

#[test]
fn fulfill_star_sector_future_saves_galaxy_object_id() {
    let connection = test_connection();

    let future = &generate_root_with_futures(&connection)[0];
    let sector = fulfill_star_sector_future(&connection, future.id)
        .expect("Error fulfilling star sector future");

    assert_eq!(future.id, sector.id);
}

#[test]
fn delete_sector_with_stars_finishes_without_errors() {
    let connection = test_connection();

    let (sector, _) = generate_root_with_5_stars(&connection);

    delete_sector(&connection, sector.id).expect("Error deleting sector");
}

#[test]
fn delete_sector_doesnt_delete_other_stars() {
    let connection = test_connection();

    let (sector_to_delete, _) = generate_root_with_5_stars(&connection);
    let (sector_to_stay, systems_to_stay) = generate_root_with_5_stars(&connection);

    delete_sector(&connection, sector_to_delete.id).expect("Error deleting sector");

    use tg_space_game::models::*;
    use tg_space_game::schema::star_systems::dsl::*;

    let systems_stayed = star_systems
        .filter(sector_id.eq(sector_to_stay.id))
        .load::<StarSystem>(&connection)
        .expect("Error loading star systems");

    assert_eq!(systems_to_stay.len(), systems_stayed.len());
}
