#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_migrations::RunMigrationsError;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::schema::types::GalaxyObjectType;

embed_migrations!();

pub fn run_migrations(
    connection: &PgConnection,
    out: &mut std::io::Stdout,
) -> Result<(), RunMigrationsError> {
    embedded_migrations::run_with_output(connection, out)
}

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

fn create_star_sector(conn: &PgConnection, parent: Option<i32>) -> Result<StarSector, Error> {
    conn.transaction::<StarSector, Error, _>(|| {
        use schema::galaxy_objects::dsl::*;
        use schema::star_sectors::dsl::*;

        let galaxy_object: GalaxyObject = diesel::insert_into(galaxy_objects)
            .values(&NewGalaxyObject {
                galaxy_object_type: GalaxyObjectType::Sector,
            })
            .get_result(conn)?;

        diesel::insert_into(star_sectors)
            .values(&NewStarSector {
                galaxy_object_id: galaxy_object.id,
                parent_id: parent,
            })
            .get_result(conn)
    })
}

pub fn generate_star_sector(
    conn: &PgConnection,
    stars: f32,
    radius: f32,
    parent: Option<i32>,
) -> Result<StarSector, Error> {
    conn.transaction::<StarSector, Error, _>(|| {
        let result = create_star_sector(conn, parent)?;

        let sub_amount = 10;
        let sub_stars = stars / (sub_amount as f32);

        if sub_stars < 10.0 {

            let stars_amount = stars.round() as i32;

            // Create galaxy objects for stars
            let new_star_galaxy_objects = (0..stars_amount)
                .map(|_| NewGalaxyObject {
                    galaxy_object_type: GalaxyObjectType::System
                })
                .collect::<Vec<_>>();
            use schema::galaxy_objects::dsl::*;
            let star_galaxy_objects: Vec<GalaxyObject> = diesel::insert_into(galaxy_objects)
                .values(&new_star_galaxy_objects)
                .get_results(conn)?;

            // Create stars themselves
            let new_stars = star_galaxy_objects
                .iter()
                .map(|g: &GalaxyObject| NewStarSystem {
                    galaxy_object_id: g.id,
                    name: "StarName".to_string(),
                    sector_id: result.id,
                })
                .collect::<Vec<_>>();
            use schema::star_systems::dsl::*;
            diesel::insert_into(star_systems)
                .values(&new_stars)
                .execute(conn)?;

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
