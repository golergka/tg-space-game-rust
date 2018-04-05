extern crate diesel;
#[macro_use]
extern crate text_io;
extern crate tg_space_game;

use self::tg_space_game::*;
use self::models::*;

fn main() {
    let connection = establish_connection();

    println!("What would be expected radius?");
    let radius: f32 = read!();

    println!("What would be expected amount of stars?");
    let stars: f32 = read!();

    let mut future: Option<StarSectorFuture> = Some(create_star_sector_future(&connection, None, stars, radius));
    loop {
        match future {
            Some(f) => {
                println!("Generating sector future with id {}, {} stars and radius {}", f.id, f.stars, f.radius);
                let new_sector: StarSector = fulfill_star_sector_future(&connection, f.id)
                    .expect("Error fulfillling star sector future");
                let mut children = get_star_sector_children_futures(&connection, &new_sector)
                    .expect("Error getting children futures");
                println!("New sector has id {} and {} sector futures in it", new_sector.id, children.len());
                for c in &children {
                    println!("Child with id {} has {} stars and radius {}", c.id, c.stars, c.radius);
                }
                future = children.pop();
            },
            None => break,
        }
    }
}
