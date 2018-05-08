use super::schema::types::*;
use super::schema::*;

#[derive(Queryable)]
pub struct GalaxyObject {
    pub id: i32,
    pub obj_type: GalaxyObjectType,
}

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
    pub galaxy_object_id: i32,
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
