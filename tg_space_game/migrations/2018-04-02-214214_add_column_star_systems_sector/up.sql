-- Your SQL goes here
ALTER TABLE star_systems ADD COLUMN sector integer REFERENCES star_sectors (id) NOT NULL