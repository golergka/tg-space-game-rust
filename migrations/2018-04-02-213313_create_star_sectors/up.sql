-- Your SQL goes here
CREATE TABLE star_sectors (
    id SERIAL PRIMARY KEY,
    parent integer REFERENCES star_sectors (id)
)