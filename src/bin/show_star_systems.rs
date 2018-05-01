extern crate diesel;
extern crate tg_space_game;

use self::diesel::prelude::*;
use self::models::*;
use self::tg_space_game::*;

fn main() {
    use tg_space_game::schema::star_systems::dsl::*;

    let connection = establish_connection();
    let results = star_systems
        .limit(5)
        .load::<StarSystem>(&connection)
        .expect("Error loading star systems");

    println!("Displaying {} star systems", results.len());
    for system in results {
        println!("{}", system.name);
    }
}
