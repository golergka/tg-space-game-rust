use super::schema::*;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StarSector, foreign_key = "sector")]
pub struct StarSystem {
    pub id: i32,
    pub name: String,
    pub sector: i32,
}

#[derive(Insertable)]
#[table_name = "star_systems"]
pub struct NewStarSystem {
    pub name: String,
    pub sector: i32
}

#[derive(Identifiable, Queryable, PartialEq, Associations)]
#[belongs_to(StarSector, foreign_key = "parent")]
pub struct StarSector {
    pub id: i32,
    pub parent: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "star_sectors"]
pub struct NewStarSector {
    pub parent: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StarSector, foreign_key = "parent")]
pub struct StarSectorFuture {
    pub id: i32,
    pub parent: i32,
    pub radius: f32,
    pub stars: f32,
}

#[derive(Insertable)]
#[table_name = "star_sector_futures"]
pub struct NewStarSectorFuture {
    pub parent: i32,
    pub radius: f32,
    pub stars: f32,
}
