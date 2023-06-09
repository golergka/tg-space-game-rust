use super::*;

fn update_galaxy_object_type(
    conn: &PgConnection,
    object_id: i32,
    object_type: GalaxyObjectType,
) -> Result<(), Error> {
    use schema::galaxy_objects::dsl::*;
    match diesel::update(galaxy_objects.filter(id.eq(object_id)))
        .set(obj_type.eq(object_type))
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

        delete_links_for_objects(conn, vec![future_id])?;

        diesel::delete(&future).execute(conn)?;

        // Change galaxy object type
        update_galaxy_object_type(conn, future_id, GalaxyObjectType::Sector)?;

        // Create new sector
        use schema::star_sectors::dsl::*;
        let sector = diesel::insert_into(star_sectors)
            .values(&NewStarSector {
                id: future_id,
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
                obj_type: GalaxyObjectType::Sector,
            })
            .get_result(conn)?;

        diesel::insert_into(star_sectors)
            .values(&NewStarSector {
                id: galaxy_object.id,
                parent_id: parent,
            })
            .get_result(conn)
    })
}

use rand::distributions::Weighted;

use std::iter::Iterator;

fn fill_star_sector(
    conn: &PgConnection,
    sector: &StarSector,
    stars: f32,
    rad: f32,
) -> Result<(), Error> {
    conn.transaction::<(), Error, _>(|| {
        // Amount of sub-sectors
        let sub_amount = 10;
        // Amount of stars in each of sub-sector
        let sub_stars = stars / (sub_amount as f32);
        // Amount of links between stars inside this sector
        let links = stars * 4f32;

        // We decide whether we create concrete stars or a sub-future
        let create_stars = sub_stars < 10.0;

        let child_amount = if create_stars {
            stars.round() as i32
        } else {
            sub_amount
        };

        // Create galaxy objects
        let star_galaxy_objects: Vec<GalaxyObject> = {
            let new_star_galaxy_objects = (0..child_amount)
                .map(|_| NewGalaxyObject {
                    obj_type: if create_stars {
                            GalaxyObjectType::System 
                        } else {
                            GalaxyObjectType::SectorFuture
                        }
                })
                .collect::<Vec<_>>();

            use schema::galaxy_objects::dsl::*;
            diesel::insert_into(galaxy_objects)
                .values(&new_star_galaxy_objects)
                .get_results(conn)?
        };

        // Create children
        let children = if create_stars {
            // Create stars themselves
            let new_stars = star_galaxy_objects
                .iter()
                .map(|g: &GalaxyObject| NewStarSystem {
                    id: g.id,
                    name: "StarName".to_string(),
                    sector_id: sector.id,
                })
                .collect::<Vec<_>>();
            use schema::star_systems::dsl::*;
            diesel::insert_into(star_systems)
                .values(&new_stars)
                .get_results::<StarSystem>(conn)?
                .iter()
                .map(GalaxyObject::from)
                .collect::<Vec<GalaxyObject>>()
        } else {
            use std::f32;
            let sub_radius = rad / (sub_amount as f32).cbrt();

            // Create sub futures themselves
            let new_futures = star_galaxy_objects
                .iter()
                .map(|g: &GalaxyObject| NewStarSectorFuture {
                    id: g.id,
                    parent_id: sector.id,
                    radius: sub_radius,
                    stars: sub_stars,
                })
                .collect::<Vec<_>>();
            use schema::star_sector_futures::dsl::*;
            diesel::insert_into(star_sector_futures)
                .values(&new_futures)
                .get_results::<StarSectorFuture>(conn)?
                .iter()
                .map(GalaxyObject::from)
                .collect::<Vec<GalaxyObject>>()
        };

        // Generate links
        let mut children_weighted = children
            .iter()
            .zip(self::tools::exp_weights(children.len()))
            .map(|pair: (&GalaxyObject, u32)| {
                let (child, weight) = pair;
                Weighted::<GalaxyObject> {
                    weight: weight,
                    item: child.clone(),
                }
            })
            .collect::<Vec<_>>();

        let new_links = generate_links(
            &mut children_weighted.as_mut_slice(),
            links as usize,
            create_stars,
            rand::thread_rng(),
        );

        use schema::star_links::dsl::*;
        diesel::insert_into(star_links)
            .values(&new_links)
            .execute(conn)?;

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

pub fn get_links_for_object_ids(conn: &PgConnection, objects: Vec<i32>
    ) -> Result<Vec<StarLink>, Error> {
    use schema::star_links::dsl::*;
    star_links
        .filter(a_id.eq_any(objects.to_vec()))
        .or_filter(b_id.eq_any(objects.to_vec()))
        .load::<StarLink>(conn)
}

pub fn get_links_for_objects(conn: &PgConnection, objects: Vec<GalaxyObject>
    ) -> Result<Vec<StarLink>, Error> {

    let ids = objects
        .iter()
        .map(|obj| obj.id.clone())
        .collect::<Vec<i32>>();

    get_links_for_object_ids(conn, ids)
}

fn delete_links_for_objects(conn: &PgConnection, objects: Vec<i32>) -> Result<usize, Error> {
    use schema::star_links::dsl::*;

    diesel::delete(
        star_links
            .filter(a_id.eq_any(objects.to_vec()))
            .or_filter(b_id.eq_any(objects.to_vec())),
    ).execute(conn)
}

fn delete_galaxy_objects(conn: &PgConnection, objects: Vec<i32>) -> Result<usize, Error> {
    use schema::galaxy_objects::dsl::*;

    diesel::delete(galaxy_objects.filter(id.eq_any(objects))).execute(conn)
}

fn delete_sector_futures(conn: &PgConnection, sector_id: i32) -> Result<usize, Error> {
    use schema::star_sector_futures::dsl::*;

    conn.transaction::<usize, Error, _>(|| {
        let ids = star_sector_futures
            .filter(parent_id.eq(sector_id))
            .select(id)
            .load(conn)?;

        delete_links_for_objects(conn, ids.to_vec())?;

        diesel::delete(star_sector_futures
                .filter(id.eq_any(ids.to_vec())))
            .execute(conn)?;

        delete_galaxy_objects(conn, ids)
    })
}

fn delete_sector_systems(conn: &PgConnection, sector: i32) -> Result<usize, Error> {
    use schema::star_systems::dsl::*;

    conn.transaction::<usize, Error, _>(|| {
        let ids = star_systems
            .filter(sector_id.eq(sector))
            .select(id)
            .load(conn)?;

        delete_links_for_objects(conn, ids.to_vec())?;

        diesel::delete(star_systems
                .filter(id.eq_any(ids.to_vec())))
            .execute(conn)?;

        delete_galaxy_objects(conn, ids)
    })
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

        delete_links_for_objects(conn, vec![sector_id])?;

        // Delete sector
        diesel::delete(star_sectors.filter(id.eq(sector_id))).execute(conn)?;

        // Delete sector's galaxy object
        delete_galaxy_objects(conn, vec![sector_id])?;

        Ok(())
    })
}