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

fn create_star_sector_future(
    conn: &PgConnection,
    parent: i32,
    stars_i: f32,
    radius_i: f32,
) -> Result<StarSectorFuture, Error> {
    conn.transaction::<StarSectorFuture, Error, _>(|| {
        use schema::galaxy_objects::dsl::*;
        use schema::star_sector_futures::dsl::*;

        let galaxy_object: GalaxyObject = diesel::insert_into(galaxy_objects)
            .values(&NewGalaxyObject {
                galaxy_object_type: GalaxyObjectType::SectorFuture,
            })
            .get_result(conn)?;

        diesel::insert_into(star_sector_futures)
            .values(&NewStarSectorFuture {
                galaxy_object_id: galaxy_object.id,
                parent_id: parent,
                radius: radius_i,
                stars: stars_i,
            })
            .get_result::<StarSectorFuture>(conn)
    })
}

fn update_galaxy_object_type(
    conn: &PgConnection,
    object_id: i32,
    object_type: GalaxyObjectType,
) -> Result<(), Error> {
    use schema::galaxy_objects::dsl::*;
    match diesel::update(galaxy_objects.filter(id.eq(object_id)))
        .set(galaxy_object_type.eq(object_type))
        .execute(conn)
    {
        Ok(1) => Ok(()),
        _ => Err(Error::NotFound),
    }
}

pub fn fulfill_star_sector_future(
    conn: &PgConnection,
    future_id: i32,
) -> Result<StarSector, Error> {
    conn.transaction::<StarSector, Error, _>(|| {
        use schema::star_sector_futures::dsl::*;

        // Find and delete old future
        let future = star_sector_futures
            .for_update()
            .find(future_id)
            .get_result::<StarSectorFuture>(conn)?;

        diesel::delete(&future).execute(conn)?;

        // Change galaxy object type
        update_galaxy_object_type(conn, future_id, GalaxyObjectType::Sector)?;

        // Create new sector
        use schema::star_sectors::dsl::*;
        let sector = diesel::insert_into(star_sectors)
            .values(&NewStarSector {
                galaxy_object_id: future_id,
                parent_id: Some(future.parent_id),
            })
            .get_result(conn)?;

        // Fill this new sector
        fill_star_sector(conn, &sector, future.stars, future.radius)?;

        Ok(sector)
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

fn fill_star_sector(
    conn: &PgConnection,
    sector: &StarSector,
    stars: f32,
    radius: f32,
) -> Result<(), Error> {
    conn.transaction::<(), Error, _>(|| {
        let sub_amount = 10;
        let sub_stars = stars / (sub_amount as f32);

        if sub_stars < 10.0 {
            let stars_amount = stars.round() as i32;

            // Create galaxy objects for stars
            let new_star_galaxy_objects = (0..stars_amount)
                .map(|_| NewGalaxyObject {
                    galaxy_object_type: GalaxyObjectType::System,
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
                    sector_id: sector.id,
                })
                .collect::<Vec<_>>();
            use schema::star_systems::dsl::*;
            diesel::insert_into(star_systems)
                .values(&new_stars)
                .execute(conn)?;

            return Ok(());
        }

        use std::f32;
        let sub_radius = radius / (sub_amount as f32).cbrt();

        for _ in 0..sub_amount {
            create_star_sector_future(conn, sector.id, sub_stars, sub_radius)?;
        }

        Ok(())
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
        fill_star_sector(conn, &result, stars, radius)?;
        Ok(result)
    })
}

pub fn get_star_sector_children_futures(
    conn: &PgConnection,
    sector: &StarSector,
) -> Result<Vec<StarSectorFuture>, Error> {
    StarSectorFuture::belonging_to(sector).load(conn)
}

fn delete_galaxy_objects(conn: &PgConnection, objects: Vec<i32>) -> Result<usize, Error> {
    use schema::galaxy_objects::dsl::*;

    diesel::delete(galaxy_objects.filter(id.eq_any(objects))).execute(conn)
}

fn delete_sector_futures(conn: &PgConnection, sector_id: i32) -> Result<usize, Error> {
    use schema::star_sector_futures::dsl::*;

    let deleted_ids = diesel::delete(star_sector_futures.filter(parent_id.eq(sector_id)))
        .returning(galaxy_object_id)
        .get_results(conn)?;

    delete_galaxy_objects(conn, deleted_ids)
}

fn delete_sector_systems(conn: &PgConnection, sector: i32) -> Result<usize, Error> {
    use schema::star_systems::dsl::*;

    let deleted_ids = diesel::delete(star_systems.filter(sector_id.eq(sector)))
        .returning(galaxy_object_id)
        .get_results(conn)?;

    delete_galaxy_objects(conn, deleted_ids)
}

pub fn delete_sector(conn: &PgConnection, sector_id: i32) -> Result<(), Error> {
    conn.transaction::<_, Error, _>(|| {
        use schema::star_sectors::dsl::*;

        // Delete child futures and systems
        delete_sector_futures(conn, sector_id)?;
        delete_sector_systems(conn, sector_id)?;

        // Find child sectors
        let child_sectors: Vec<StarSector> = star_sectors
            .filter(parent_id.eq(sector_id))
            .for_update()
            .load(conn)?;

        // Recursively delete child sectors;
        for c in child_sectors {
            try!(delete_sector(conn, c.id));
        }

        // Delete sector
        diesel::delete(star_sectors.filter(galaxy_object_id.eq(sector_id))).execute(conn)?;

        // Delete sector's galaxy object
        delete_galaxy_objects(conn, vec![sector_id])?;

        Ok(())
    })
}
