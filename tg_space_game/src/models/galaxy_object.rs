use super::*;

#[derive(Queryable, Hash, PartialEq, Eq, Clone)]
pub struct GalaxyObject {
    pub id: i32,
    pub obj_type: GalaxyObjectType,
}

pub trait ToGalaxyObject {
    fn to_galaxy_object(&self) -> GalaxyObject;
}

impl ToGalaxyObject for GalaxyObject {
    fn to_galaxy_object(&self) -> GalaxyObject {
        self.clone()
    }
}

impl<'a> From<&'a StarSector> for GalaxyObject {
    fn from(sector: &StarSector) -> Self {
        GalaxyObject {
            id: sector.id,
            obj_type: GalaxyObjectType::Sector
        }
    }
}

impl ToGalaxyObject for StarSector {
    fn to_galaxy_object(&self) -> GalaxyObject {
        GalaxyObject::from(self)
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

impl ToGalaxyObject for StarSectorFuture {
    fn to_galaxy_object(&self) -> GalaxyObject {
        GalaxyObject::from(self)
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

impl ToGalaxyObject for StarSystem {
    fn to_galaxy_object(&self) -> GalaxyObject {
        GalaxyObject::from(self)
    }
}

#[derive(Insertable)]
#[table_name = "galaxy_objects"]
pub struct NewGalaxyObject {
    pub obj_type: GalaxyObjectType,
}
