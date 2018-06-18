extern crate diesel;
#[macro_use]
extern crate text_io;
extern crate tg_space_game;

use self::tg_space_game::*;
use self::tg_space_game::galaxy_objects::*;

fn main() {
    let connection = establish_connection();

    println!("What would be expected radius?");
    let radius: f32 = read!();

    println!("What would be expected amount of stars?");
    let stars: f32 = read!();

    let mut sector =
        generate_star_sector(&connection, stars, radius, None).expect("Error creating star sector");
    let mut children = get_star_sector_children_futures(&connection, &sector)
        .expect("Error getting children futures");
    let mut future = children.pop();
    loop {
        match future {
            Some(f) => {
                sector = fulfill_star_sector_future(&connection, f.id)
                    .expect("Error fulfilling star sector future");
                children = get_star_sector_children_futures(&connection, &sector)
                    .expect("Error getting children futures");
                future = children.pop();
            }
            None => break,
        }
    }
}
