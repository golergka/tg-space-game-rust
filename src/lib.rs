#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::*;

pub fn create_star_sector_future(
    conn: &PgConnection,
    parent: i32,
    stars_i: f32,
    radius_i: f32,
) -> StarSectorFuture {
    use schema::star_sector_futures::dsl::*;

    let new_future = NewStarSectorFuture {
        parent_id: parent,
        radius: radius_i,
        stars: stars_i,
    };

    diesel::insert_into(star_sector_futures)
        .values(&new_future)
        .get_result::<StarSectorFuture>(conn)
        .expect("Error creating a star sector future")
}

pub fn fulfill_star_sector_future(
    conn: &PgConnection,
    future_id: i32,
) -> Result<StarSector, Error> {
    use schema::star_sector_futures::dsl::*;

    conn.transaction::<StarSector, Error, _>(|| {
        let future = star_sector_futures
            .for_update()
            .find(future_id)
            .get_result::<StarSectorFuture>(conn)
            .expect(&format!("Unable to load star sector future {}", future_id));

        generate_star_sector(conn, future.stars, future.radius, Some(future.parent_id))
    })
}

pub fn generate_star_sector(
    conn: &PgConnection,
    stars: f32,
    radius: f32,
    parent: Option<i32>,
) -> Result<StarSector, Error> {
    conn.transaction::<StarSector, Error, _>(|| {
        use schema::star_sectors::dsl::*;

        let new_sector = NewStarSector {
            parent_id: parent,
        };

        let result: StarSector = diesel::insert_into(star_sectors)
            .values(&new_sector)
            .get_result(conn)
            .expect("Error creating a star sector");

        let sub_amount = 10;
        let sub_stars = stars / (sub_amount as f32);

        if sub_stars < 2.0 {
            use schema::star_systems::dsl::*;

            let stars_amount = stars.round() as i32;
            let new_stars = (0..stars_amount)
                .map(|_| NewStarSystem {
                    name: "StarName".to_string(),
                    sector_id: result.id,
                })
                .collect::<Vec<_>>();

            diesel::insert_into(star_systems)
                .values(&new_stars)
                .execute(conn)
                .expect("Couldn't create child systems!");

            return Ok(result);
        }

        use std::f32;
        let sub_radius = radius / (sub_amount as f32).cbrt();

        for _ in 0..sub_amount {
            create_star_sector_future(conn, result.id, sub_stars, sub_radius);
        }

        Ok(result)
    })
}

pub fn get_star_sector_children_futures(
    conn: &PgConnection,
    sector: &StarSector,
) -> Result<Vec<StarSectorFuture>, Error> {
    StarSectorFuture::belonging_to(sector).load(conn)
}

pub fn delete_sector_futures(conn: &PgConnection, sector_id: i32) -> Result<usize, Error> {
    use schema::star_sector_futures::dsl::*;

    diesel::delete(star_sector_futures.filter(parent_id.eq(sector_id))).execute(conn)
}

pub fn delete_sector(conn: &PgConnection, sector_id: i32) -> Result<(), Error> {
    conn.transaction::<_, Error, _>(|| {
        use schema::star_sectors::dsl::*;

        delete_sector_futures(conn, sector_id)?;

        let child_sectors: Vec<StarSector> = star_sectors
            .filter(parent_id.eq(sector_id))
            .for_update()
            .load(conn)?;

        for c in child_sectors {
            try!(delete_sector(conn, c.id));
        }

        diesel::delete(star_sectors.filter(galaxy_object_id.eq(sector_id))).execute(conn)?;

        Ok(())
    })
}
