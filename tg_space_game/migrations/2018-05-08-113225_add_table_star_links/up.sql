-- Your SQL goes here
CREATE TABLE star_links (
    id SERIAL PRIMARY KEY,
    a_id INTEGER NOT NULL,
    a_obj_type galaxy_object_type NOT NULL CHECK (a_obj_type IN ('system', 'sector_future')),
    b_id INTEGER NOT NULL,
    b_obj_type galaxy_object_type NOT NULL CHECK (b_obj_type IN ('system', 'sector_future')),
    FOREIGN KEY (a_id, a_obj_type) REFERENCES galaxy_objects (id, obj_type),
    FOREIGN KEY (b_id, b_obj_type) REFERENCES galaxy_objects (id, obj_type)
);