extern crate tg_space_game;
extern crate diesel;

use self::tg_space_game::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use tg_space_game::schema::star_systems::dsl::*;

    let connection = establish_connection();
    let results = star_systems.limit(5)
        .load::<StarSystem>(&connection)
        .expect("Error loading star systems");

    println!("Displaying {} star systems", results.len());
    for system in results {
        println!("{}", system.name);
    }
}