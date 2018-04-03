extern crate diesel;
#[macro_use]
extern crate text_io;
extern crate tg_space_game;

use self::tg_space_game::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use tg_space_game::schema::star_systems::dsl::*;

    let connection = establish_connection();

    println!("What would be expected radius?");
    let radius: f32 = read!();

    println!("What would be expected amount of stars?");
    let stars: f32 = read!();

    let future = create_star_sector_future(&connection, None, stars, radius);
}
