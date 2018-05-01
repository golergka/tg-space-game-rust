-- This file should undo anything in `up.sql`

-- Bring back star_sectors.id
ALTER TABLE star_sectors ADD COLUMN id SERIAL;
ALTER TABLE star_sectors DROP CONSTRAINT star_sectors_pkey;
ALTER TABLE star_sectors ADD PRIMARY KEY (id);

-- star_sector relationship to itself
ALTER TABLE star_sectors ADD COLUMN parent integer;
WITH star_sectors_ AS (SELECT galaxy_object_id,id FROM star_sectors)
UPDATE star_sectors
    SET parent = star_sectors_.id
    FROM star_sectors_
    WHERE star_sectors_.galaxy_object_id = star_sectors.parent_id;
ALTER TABLE star_sectors DROP CONSTRAINT star_sectors_parent_fkey;
ALTER TABLE star_sectors
    ADD CONSTRAINT star_sectors_parent_fkey
    FOREIGN KEY (parent)
    REFERENCES star_sectors (id);
ALTER TABLE star_sectors DROP COLUMN parent_id;

-- star_systems restore relationship with parent
ALTER TABLE star_systems ADD COLUMN sector integer;
UPDATE star_systems
    SET sector = star_sectors.id
    FROM star_sectors
    WHERE star_sectors.galaxy_object_id = star_systems.sector_id;
ALTER TABLE star_systems DROP CONSTRAINT star_systems_sector_fkey;
ALTER TABLE star_systems
    ADD CONSTRAINT star_systems_sector_fkey
    FOREIGN KEY (sector)
    REFERENCES star_sectors (id);
ALTER TABLE star_systems DROP COLUMN sector_id;

-- star_sector_futures restore relationship with parent
ALTER TABLE star_sector_futures ADD COLUMN parent integer;
UPDATE star_sector_futures
    SET parent = star_sectors.id
    FROM star_sectors
    WHERE star_sectors.galaxy_object_id = star_sector_futures.parent_id;
ALTER TABLE star_sector_futures DROP CONSTRAINT star_sector_futures_parent_fkey;
ALTER TABLE star_sector_futures
    ADD CONSTRAINT star_sector_futures_parent_fkey
    FOREIGN KEY (parent)
    REFERENCES star_sectors (id);
ALTER TABLE star_sector_futures DROP COLUMN parent_id;

-- Bring back star_systems.id
ALTER TABLE star_systems ADD COLUMN id SERIAL;
ALTER TABLE star_systems DROP CONSTRAINT star_systems_pkey;
ALTER TABLE star_systems ADD PRIMARY KEY (id);

-- Bring back star_sector_futures.id
ALTER TABLE star_sector_futures ADD COLUMN id SERIAL;
ALTER TABLE star_sector_futures DROP CONSTRAINT star_sector_futures_pkey;
ALTER TABLE star_sector_futures ADD PRIMARY KEY (id);

-- Delete columns for star_sector_futures
ALTER TABLE star_sector_futures DROP CONSTRAINT galaxy_object_type;
ALTER TABLE star_sector_futures DROP CONSTRAINT galaxy_object_fkey;
ALTER TABLE star_sector_futures DROP COLUMN galaxy_object_type;
ALTER TABLE star_sector_futures DROP COLUMN galaxy_object_id;

-- Delete columns for star_systems
ALTER TABLE star_systems DROP CONSTRAINT galaxy_object_type;
ALTER TABLE star_systems DROP CONSTRAINT galaxy_object_fkey;
ALTER TABLE star_systems DROP COLUMN galaxy_object_type;
ALTER TABLE star_systems DROP COLUMN galaxy_object_id;

-- Delete columns for star_sectors
ALTER TABLE star_sectors DROP CONSTRAINT galaxy_object_id_unique;
ALTER TABLE star_sectors DROP CONSTRAINT galaxy_object_type;
ALTER TABLE star_sectors DROP CONSTRAINT galaxy_object_fkey;
ALTER TABLE star_sectors DROP COLUMN galaxy_object_type;
ALTER TABLE star_sectors DROP COLUMN galaxy_object_id;

-- Delete galaxy_objects
DROP TABLE galaxy_objects;
DROP TYPE galaxy_object_type;