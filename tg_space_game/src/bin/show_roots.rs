extern crate diesel;
extern crate tg_space_game;

use self::diesel::prelude::*;
use self::models::*;
use self::tg_space_game::*;

fn main() {
    use tg_space_game::schema::star_sectors::dsl::*;

    let connection = establish_connection();
    let results = star_sectors
        .limit(100)
        .filter(parent_id.is_null())
        .load::<StarSector>(&connection)
        .expect("Error loading sectors");

    println!("Got {} root sectors:", results.len());
    for s in results {
        println!("{}", s.id);
    }
}
