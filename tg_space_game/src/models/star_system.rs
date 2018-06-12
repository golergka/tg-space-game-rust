use super::*;

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