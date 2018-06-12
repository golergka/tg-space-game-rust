use super::*;

#[derive(Queryable, Hash, PartialEq, Eq, Clone)]
pub struct GalaxyObject {
    pub id: i32,
    pub obj_type: GalaxyObjectType,
}

impl<'a> From<&'a StarSector> for GalaxyObject {
    fn from(sector: &StarSector) -> Self {
        GalaxyObject {
            id: sector.id,
            obj_type: GalaxyObjectType::Sector
        }
    }
}

impl<'a> From<&'a StarSectorFuture> for GalaxyObject {
    fn from(sector: &StarSectorFuture) -> Self {
        GalaxyObject {
            id: sector.id,
            obj_type: GalaxyObjectType::SectorFuture
        }
    }
}

impl<'a> From<&'a StarSystem> for GalaxyObject {
    fn from(sector: &StarSystem) -> Self {
        GalaxyObject {
            id: sector.id,
            obj_type: GalaxyObjectType::System
        }
    }
}

#[derive(Insertable)]
#[table_name = "galaxy_objects"]
pub struct NewGalaxyObject {
    pub obj_type: GalaxyObjectType,
}
