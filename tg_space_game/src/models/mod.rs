use super::schema::types::*;
use super::schema::*;

pub use self::galaxy_object::GalaxyObject;

mod galaxy_object;

#[derive(Insertable)]
#[table_name = "galaxy_objects"]
pub struct NewGalaxyObject {
    pub obj_type: GalaxyObjectType,
}

#[derive(Identifiable, Queryable, PartialEq, Associations)]
#[belongs_to(StarSector, foreign_key = "parent_id")]
pub struct StarSector {
    pub id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "star_sectors"]
pub struct NewStarSector {
    pub id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StarSector, foreign_key = "sector_id")]
pub struct StarSystem {
    pub id: i32,
    pub name: String,
    pub sector_id: i32,
}

#[derive(Insertable)]
#[table_name = "star_systems"]
pub struct NewStarSystem {
    pub id: i32,
    pub name: String,
    pub sector_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StarSector, foreign_key = "parent_id")]
pub struct StarSectorFuture {
    pub id: i32,
    pub parent_id: i32,
    pub radius: f32,
    pub stars: f32,
}

#[derive(Insertable)]
#[table_name = "star_sector_futures"]
pub struct NewStarSectorFuture {
    pub id: i32,
    pub parent_id: i32,
    pub radius: f32,
    pub stars: f32,
}

#[derive(Identifiable, Queryable)]
pub struct StarLink {
    pub id: i32,
    pub a_id: i32,
    pub a_obj_type: GalaxyObjectType,
    pub b_id: i32,
    pub b_obj_type: GalaxyObjectType,
}

#[derive(Insertable)]
#[table_name = "star_links"]
pub struct NewStarLink {
    pub a_id: i32,
    pub a_obj_type: GalaxyObjectType,
    pub b_id: i32,
    pub b_obj_type: GalaxyObjectType,
}

use std::collections::hash_map::DefaultHasher;

impl NewStarLink {
    pub fn new(a: &GalaxyObject, b: &GalaxyObject) -> NewStarLink {
        NewStarLink {
            a_id: a.id,
            a_obj_type: a.obj_type,
            b_id: b.id,
            b_obj_type: b.obj_type
        }
    }

    // TODO make these methods a trait and implement this trait for both NewStarLink and StarLink
    pub fn side_a(&self) -> GalaxyObject {
        GalaxyObject {
            id: self.a_id,
            obj_type: self.a_obj_type
        }
    }

    pub fn side_b(&self) -> GalaxyObject {
        GalaxyObject {
            id: self.b_id,
            obj_type: self.b_obj_type
        }
    }

}

use std::cmp;

impl PartialEq for NewStarLink {
    fn eq(&self, other: &NewStarLink) -> bool {
        (
            self.side_a() == other.side_a() &&
            self.side_b() == other.side_b()
        ) ||
        (
            self.side_a() == other.side_b() &&
            self.side_b() == other.side_a()
        )
    }
}

use std::hash::{Hash, Hasher};

impl Hash for NewStarLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Create side hashers
        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        // Hash sides
        self.side_a().hash(&mut hasher_a);
        self.side_b().hash(&mut hasher_b);
        // Finish side hashers
        let hash_a = hasher_a.finish();
        let hash_b = hasher_b.finish();
        // Order side hashes
        let hash_max = cmp::max(hash_a, hash_b);
        let hash_min = cmp::min(hash_a, hash_b);
        // Hash in order
        hash_max.hash(state);
        hash_min.hash(state);
    }
}

impl Eq for NewStarLink {}