use super::schema::*;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StarSector, foreign_key = "secor")]
pub struct StarSystem {
    pub id: i32,
    pub name: String,
    pub sector: i32,
}

#[derive(Identifiable, Queryable)]
pub struct StarSector {
    pub id: i32,
    pub parent: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "star_sectors"]
pub struct NewStarSector {
    pub parent: Option<i32>,
}

#[derive(Identifiable, Queryable)]
pub struct StarSectorFuture {
    pub id: i32,
    pub parent: Option<i32>,
    pub radius: f32,
    pub stars: f32,
}

#[derive(Insertable)]
#[table_name = "star_sector_futures"]
pub struct NewStarSectorFuture {
    pub parent: Option<i32>,
    pub radius: f32,
    pub stars: f32,
}
