use super::*;

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