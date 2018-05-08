pub mod types;

table! {
    use diesel::sql_types::*;
    use schema::types::*;
    galaxy_objects (id) {
        id -> Int4,
        obj_type -> GalaxyObjectTypeSql,
    }
}

table! {
    use diesel::sql_types::*;
    star_sectors (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    star_sector_futures (id) {
        id -> Int4,
        parent_id -> Int4,
        radius -> Float4,
        stars -> Float4,
    }
}

table! {
    use diesel::sql_types::*;
    star_systems (id) {
        id -> Int4,
        name -> Varchar,
        sector_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    galaxy_objects,
    star_sector_futures,
    star_sectors,
    star_systems,
);
