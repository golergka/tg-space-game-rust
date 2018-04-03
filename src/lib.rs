#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::*;

pub fn create_star_sector(conn: &PgConnection, parent: Option<i32>) -> StarSector {
    use schema::star_sectors;

    let new_sector = NewStarSector { parent: parent };

    diesel::insert_into(star_sectors::table)
        .values(&new_sector)
        .get_result(conn)
        .expect("Error creating a star sector")
}

pub fn create_star_sector_future(
    conn: &PgConnection,
    parent: Option<i32>,
    stars: f32,
    radius: f32,
) -> StarSectorFuture {
    use schema::star_sector_futures;

    let new_future = NewStarSectorFuture {
        parent: parent,
        radius: radius,
        stars: stars,
    };

    diesel::insert_into(star_sector_futures::table)
        .values(&new_future)
        .get_result(conn)
        .expect("Error creating a star sector future")
}

pub fn fulfill_star_sector_future(conn: &PgConnection, future_id: i32) {
    use schema::star_sector_futures;
    use schema::star_sector_futures::dsl::*;
    use diesel::result::Error;

    conn.transaction::<_, Error, _>(|| {
        let future = star_sector_futures
            .for_update()
            .find(future_id)
            .get_result::<StarSectorFuture>(conn)
            .expect(&format!("Unable to load star sector future {}", future_id));

        let new_sector = create_star_sector(conn, future.parent);

        let sub_amount = 10;
        let sub_stars = future.stars / (sub_amount as f32);

        if sub_stars < 2.0 {
            return Ok(()); // TODO
        }

        use std::f32;
        let sub_radius = future.radius / (sub_amount as f32).cbrt();
        let sub_range = [0..sub_amount];
        let sub_futures = sub_range
            .iter()
            .map(|_| create_star_sector_future(conn, Some(new_sector.id), sub_stars, sub_radius));

        Ok(())
    });
}
