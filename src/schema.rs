table! {
    star_sector_futures (id) {
        id -> Int4,
        parent -> Int4,
        radius -> Float4,
        stars -> Float4,
    }
}

table! {
    star_sectors (id) {
        id -> Int4,
        parent -> Nullable<Int4>,
    }
}

table! {
    star_systems (id) {
        id -> Int4,
        name -> Varchar,
        sector -> Int4,
    }
}

joinable!(star_sector_futures -> star_sectors (parent));
joinable!(star_systems -> star_sectors (sector));

allow_tables_to_appear_in_same_query!(
    star_sector_futures,
    star_sectors,
    star_systems,
);
