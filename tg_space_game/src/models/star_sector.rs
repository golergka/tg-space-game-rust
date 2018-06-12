use super::*;

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
