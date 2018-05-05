-- Your SQL goes here
CREATE TABLE star_sector_futures (
    id SERIAL PRIMARY KEY,
    parent integer REFERENCES star_sectors (id),
    radius real NOT NULL,
    stars real NOT NULL
)