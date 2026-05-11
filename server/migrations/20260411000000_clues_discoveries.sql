-- Add migration script here

-- Cleanup old grid/clue structures
DROP TABLE IF EXISTS grid_cells;
DROP TABLE IF EXISTS logic_grids;
DROP TABLE IF EXISTS clues;

-- Define new types/tables for Relational Discovery
CREATE TYPE category AS ENUM ('suspect', 'weapon', 'location');

CREATE TABLE clues (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id VARCHAR(5) NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    x_category category NOT NULL,
    x_idx INTEGER NOT NULL,
    y_category category NOT NULL,
    y_idx INTEGER NOT NULL,
    is_true BOOLEAN NOT NULL,
    UNIQUE(room_id, x_category, x_idx, y_category, y_idx)
);

CREATE TABLE discoveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES users(id),
    room_id VARCHAR(5) REFERENCES rooms(id)
);
