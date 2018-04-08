#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
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

pub fn fulfill_star_sector_future(conn: &PgConnection, future_id: i32) -> Result<StarSector, Error> {
    use schema::star_sector_futures::dsl::*;

    conn.transaction::<StarSector, Error, _>(|| {
        let future = star_sector_futures
            .for_update()
            .find(future_id)
            .get_result::<StarSectorFuture>(conn)
            .expect(&format!("Unable to load star sector future {}", future_id));

        let new_sector = create_star_sector(conn, future.parent);

        let sub_amount = 10;
        let sub_stars = future.stars / (sub_amount as f32);

        if sub_stars < 2.0 {
            return Ok(new_sector); 
            // TODO: create stars
        }

        use std::f32;
        let sub_radius = future.radius / (sub_amount as f32).cbrt();
        let parent_id = new_sector.id;

        for _ in 0..sub_amount {
            create_star_sector_future(conn, Some(parent_id), sub_stars, sub_radius);
        }

        Ok(new_sector)
    })
}

pub fn get_star_sector_children_futures(conn: &PgConnection, sector: &StarSector) -> Result<Vec<StarSectorFuture>, Error> {
    StarSectorFuture::belonging_to(sector).load(conn)
}

pub fn delete_sector_futures(conn: &PgConnection, sector_id: i32) -> Result<usize, Error> {
    use schema::star_sector_futures::dsl::*;

    diesel::delete(star_sector_futures.filter(parent.eq(sector_id)))
        .execute(conn)
}

pub fn delete_sector(conn: &PgConnection, sector_id: i32) -> Result<(),Error> {
    conn.transaction::<_, Error, _>(|| {
        use schema::star_sectors::dsl::*;

        delete_sector_futures(conn, sector_id)?;

        let child_sectors: Vec<StarSector> = star_sectors.filter(parent.eq(sector_id)).for_update().load(conn)?;

        for c in child_sectors {
            try!(delete_sector(conn, c.id));
        }

        diesel::delete(star_sectors.filter(id.eq(sector_id))).execute(conn)?;

        Ok(())
    })
}