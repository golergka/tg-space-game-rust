extern crate diesel;
extern crate tg_space_game;

use self::tg_space_game::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_string: &String = &args.get(1).expect("Please provide sector id!");
    let target: i32 = target_string.parse().expect("Please provide numeric id");

    let connection = establish_connection();
    match delete_sector(&connection, target) {
        Ok(_) => println!("Deleted sector {}", target),
        Err(err) => println!("Error deleting sector {}, {:?}", target, err)
    }
}