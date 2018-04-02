use super::schema::*;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StarSector, foreign_key = "secor")]
pub struct StarSystem {
    pub id: i32,
    pub name: String,
    pub sector: i32
}

#[derive(Identifiable, Queryable)]
pub struct StarSector {
    pub id: i32,
    pub parent: i32
}