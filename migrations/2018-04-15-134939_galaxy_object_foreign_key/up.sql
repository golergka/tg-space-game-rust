-- Create galaxy_objects
CREATE TYPE galaxy_object_type AS enum ('system', 'sector', 'sector_future');
CREATE TABLE galaxy_objects (
    galaxy_object_id SERIAL PRIMARY KEY,
    galaxy_object_type galaxy_object_type NOT NULL,
    unique (galaxy_object_id, galaxy_object_type)
);

-- Add columns to star_sectors
ALTER TABLE star_sectors ADD COLUMN galaxy_object_id integer;
ALTER TABLE star_sectors ADD COLUMN galaxy_object_type galaxy_object_type;
ALTER TABLE star_sectors
    ADD CONSTRAINT galaxy_object_fkey
    FOREIGN KEY (galaxy_object_id, galaxy_object_type)
    REFERENCES galaxy_objects (galaxy_object_id, galaxy_object_type);
ALTER TABLE star_sectors
    ADD CONSTRAINT galaxy_object_type
    CHECK (galaxy_object_type = 'sector');
ALTER TABLE star_sectors
    ADD CONSTRAINT galaxy_object_id_unique
    UNIQUE (galaxy_object_id);

-- Create entries for star_sectors
ALTER TABLE galaxy_objects ADD COLUMN temp_sector_id integer;
INSERT INTO galaxy_objects (galaxy_object_type, temp_sector_id) 
    SELECT 'sector', id 
    FROM star_sectors;
UPDATE star_sectors
    SET galaxy_object_id = galaxy_objects.galaxy_object_id, 
        galaxy_object_type = 'sector'
    FROM galaxy_objects
    WHERE star_sectors.id = galaxy_objects.temp_sector_id;

-- Add NOT NULL constraint for star_sectors
ALTER TABLE star_sectors ALTER COLUMN galaxy_object_id SET NOT NULL;
ALTER TABLE star_sectors ALTER COLUMN galaxy_object_type SET NOT NULL;

-- Add columns to star_systems
ALTER TABLE star_systems ADD COLUMN galaxy_object_id integer;
ALTER TABLE star_systems ADD COLUMN galaxy_object_type galaxy_object_type;
ALTER TABLE star_systems
    ADD CONSTRAINT galaxy_object_fkey
    FOREIGN KEY (galaxy_object_id, galaxy_object_type)
    REFERENCES galaxy_objects (galaxy_object_id, galaxy_object_type);
ALTER TABLE star_systems
    ADD CONSTRAINT galaxy_object_type
    CHECK (galaxy_object_type = 'system');

-- Create entries for star_systems
ALTER TABLE galaxy_objects ADD COLUMN temp_system_id integer;
INSERT INTO galaxy_objects (galaxy_object_type, temp_system_id)
    SELECT 'system', id
    FROM star_systems;
UPDATE star_systems
    SET galaxy_object_id = galaxy_objects.galaxy_object_id,
        galaxy_object_type = 'system'
    FROM galaxy_objects
    WHERE star_systems.id = galaxy_objects.temp_system_id;

-- Add NOT NULL constraint for star_systems
ALTER TABLE star_systems ALTER COLUMN galaxy_object_id SET NOT NULL;
ALTER TABLE star_systems ALTER COLUMN galaxy_object_type SET NOT NULL;

-- Add columns to star_sector_futures
ALTER TABLE star_sector_futures ADD COLUMN galaxy_object_id integer;
ALTER TABLE star_sector_futures ADD COLUMN galaxy_object_type galaxy_object_type;
ALTER TABLE star_sector_futures
    ADD CONSTRAINT galaxy_object_fkey
    FOREIGN KEY (galaxy_object_id, galaxy_object_type)
    REFERENCES galaxy_objects (galaxy_object_id, galaxy_object_type);
ALTER TABLE star_sector_futures
    ADD CONSTRAINT galaxy_object_type
    CHECK (galaxy_object_type = 'sector_future');

-- Create entries for star_sector_futures
ALTER TABLE galaxy_objects ADD COLUMN temp_future_id integer;
INSERT INTO galaxy_objects (galaxy_object_type, temp_future_id)
    SELECT 'sector_future', id
    FROM star_sector_futures;
UPDATE star_sector_futures
    SET galaxy_object_id = galaxy_objects.galaxy_object_id,
        galaxy_object_type = 'sector_future'
    FROM galaxy_objects
    WHERE star_sector_futures.id = galaxy_objects.temp_future_id;

-- Add NOT NULL constraint for star_sector_futures
ALTER TABLE star_sector_futures ALTER COLUMN galaxy_object_id SET NOT NULL;
ALTER TABLE star_sector_futures ALTER COLUMN galaxy_object_type SET NOT NULL;

-- Remove old star_sector_futures.id
ALTER TABLE star_sector_futures DROP CONSTRAINT star_sector_futures_pkey;
ALTER TABLE star_sector_futures ADD PRIMARY KEY (galaxy_object_id);
ALTER TABLE star_sector_futures DROP COLUMN id;

-- Remove old star_systems.id
ALTER TABLE star_systems DROP CONSTRAINT star_systems_pkey;
ALTER TABLE star_systems ADD PRIMARY KEY (galaxy_object_id);
ALTER TABLE star_systems DROP COLUMN id;

-- star_sector_futures new relationship with parent
ALTER TABLE star_sector_futures ADD COLUMN parent_id integer;
ALTER TABLE star_sector_futures DROP CONSTRAINT star_sector_futures_parent_fkey;
ALTER TABLE star_sector_futures
    ADD CONSTRAINT star_sector_futures_parent_fkey
    FOREIGN KEY (parent_id)
    REFERENCES star_sectors (galaxy_object_id);
UPDATE star_sector_futures
    SET parent_id = galaxy_objects.galaxy_object_id
    FROM galaxy_objects
    WHERE star_sector_futures.parent = galaxy_objects.temp_sector_id;
ALTER TABLE star_sector_futures ALTER COLUMN parent_id SET NOT NULL;
ALTER TABLE star_sector_futures DROP COLUMN parent;

-- star_system new relationsip with parent
ALTER TABLE star_systems ADD COLUMN sector_id integer;
ALTER TABLE star_systems DROP CONSTRAINT star_systems_sector_fkey;
ALTER TABLE star_systems
    ADD CONSTRAINT star_systems_sector_fkey
    FOREIGN KEY (sector_id)
    REFERENCES star_sectors (galaxy_object_id);
UPDATE star_systems
    SET sector_id = galaxy_objects.galaxy_object_id
    FROM galaxy_objects
    WHERE star_systems.sector = galaxy_objects.temp_sector_id;
ALTER TABLE star_systems ALTER COLUMN sector_id SET NOT NULL;
ALTER TABLE star_systems DROP COLUMN sector;

-- star_sector relationship to itself
ALTER TABLE star_sectors ADD COLUMN parent_id integer;
ALTER TABLE star_sectors DROP CONSTRAINT star_sectors_parent_fkey;
ALTER TABLE star_sectors
    ADD CONSTRAINT star_sectors_parent_fkey
    FOREIGN KEY (parent_id)
    REFERENCES star_sectors (galaxy_object_id);
UPDATE star_sectors
    SET parent_id = galaxy_objects.galaxy_object_id
    FROM galaxy_objects
    WHERE star_sectors.parent = galaxy_objects.temp_sector_id;
ALTER TABLE star_sectors DROP COLUMN parent;

-- Remove old star_sectors.id
ALTER TABLE star_sectors DROP CONSTRAINT star_sectors_pkey;
ALTER TABLE star_sectors ADD PRIMARY KEY (galaxy_object_id);
ALTER TABLE star_sectors DROP COLUMN id;

-- Drop temp columns from galaxy_objects
ALTER TABLE galaxy_objects DROP COLUMN temp_system_id;
ALTER TABLE galaxy_objects DROP COLUMN temp_future_id;
ALTER TABLE galaxy_objects DROP COLUMN temp_sector_id;