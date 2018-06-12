use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "galaxy_object_type")]
pub struct GalaxyObjectTypeSql;

#[derive(Debug, PartialEq, Eq, Hash, FromSqlRow, AsExpression, Copy, Clone)]
#[sql_type = "GalaxyObjectTypeSql"]
pub enum GalaxyObjectType {
    System,
    Sector,
    SectorFuture,
}

impl ToSql<GalaxyObjectTypeSql, Pg> for GalaxyObjectType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            GalaxyObjectType::System => out.write_all(b"system")?,
            GalaxyObjectType::Sector => out.write_all(b"sector")?,
            GalaxyObjectType::SectorFuture => out.write_all(b"sector_future")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<GalaxyObjectTypeSql, Pg> for GalaxyObjectType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"system" => Ok(GalaxyObjectType::System),
            b"sector" => Ok(GalaxyObjectType::Sector),
            b"sector_future" => Ok(GalaxyObjectType::SectorFuture),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
