-- Add discovery card info to discoveries table
CREATE TYPE discovery_card_type AS ENUM ('wild', 'same', 'different');

ALTER TABLE discoveries 
    ADD COLUMN card_type discovery_card_type NOT NULL DEFAULT 'wild',
    ADD COLUMN category_1 category,
    ADD COLUMN category_2 category,
    ALTER COLUMN clue_id DROP NOT NULL,
    ADD CONSTRAINT check_discovery_categories 
        CHECK (
            (card_type = 'wild' AND category_1 IS NULL AND category_2 IS NULL) OR
            (card_type = 'same' AND category_1 IS NOT NULL AND category_2 IS NULL) OR
            (card_type = 'different' AND category_1 IS NOT NULL AND category_2 IS NOT NULL)
        );
