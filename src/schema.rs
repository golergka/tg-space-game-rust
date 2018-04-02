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

joinable!(star_systems -> star_sectors (sector));

allow_tables_to_appear_in_same_query!(
    star_sectors,
    star_systems,
);