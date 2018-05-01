pub mod types;

table! {
    use diesel::sql_types::*;
    use schema::types::*;
    galaxy_objects (galaxy_object_id) {
        galaxy_object_id -> Int4,
        galaxy_object_type -> GalaxyObjectTypeSql,
    }
}

table! {
    use diesel::sql_types::*;
    use schema::types::*;
    star_sectors (galaxy_object_id) {
        galaxy_object_id -> Int4,
        galaxy_object_type -> GalaxyObjectTypeSql,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use schema::types::*;
    star_sector_futures (galaxy_object_id) {
        radius -> Float4,
        stars -> Float4,
        galaxy_object_id -> Int4,
        galaxy_object_type -> GalaxyObjectTypeSql,
        parent_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use schema::types::*;
    star_systems (galaxy_object_id) {
        name -> Varchar,
        galaxy_object_id -> Int4,
        galaxy_object_type -> GalaxyObjectTypeSql,
        sector_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    galaxy_objects,
    star_sector_futures,
    star_sectors,
    star_systems,
);
