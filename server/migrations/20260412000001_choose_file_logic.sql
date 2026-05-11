-- Update game_states and discoveries for ChooseFile logic
ALTER TABLE game_states ADD COLUMN files_data JSONB;

CREATE TABLE discovery_clues (
    discovery_id UUID NOT NULL REFERENCES discoveries(id) ON DELETE CASCADE,
    clue_id UUID NOT NULL REFERENCES clues(id) ON DELETE CASCADE,
    file_number INT NOT NULL,
    PRIMARY KEY (discovery_id, clue_id)
);

-- clue_id in discoveries is now redundant if we use discovery_clues
ALTER TABLE discoveries DROP COLUMN clue_id;
