CREATE EXTENSION IF NOT EXISTS "pgcrypto";


CREATE TABLE rooms (
    id VARCHAR(5) NOT NULL UNIQUE CHECK (char_length(id) = 5),
    display_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);


CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    display_name TEXT NOT NULL CHECK (char_length(display_name) <= 32),
    connection_token UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_heartbeat TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_users_last_heartbeat ON users(last_heartbeat);


CREATE TABLE room_participants (
    room_id VARCHAR(5) NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_host BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (room_id, user_id)
);


CREATE TYPE game_status AS ENUM (
    'open',
    'in_progress',
    'finished'
);


CREATE TABLE game_states (
    room_id VARCHAR(5) PRIMARY KEY REFERENCES rooms(id) ON DELETE CASCADE,
    status game_status NOT NULL DEFAULT 'open',
    current_turn_user UUID,
    started_at TIMESTAMPTZ,
    ended_at TIMESTAMPTZ,
    solution_file INT NOT NULL,
    version INT NOT NULL DEFAULT 0,
    FOREIGN KEY (room_id, current_turn_user)
        REFERENCES room_participants(room_id, user_id)
        ON DELETE SET NULL
);
CREATE INDEX idx_game_states_current_turn ON game_states(current_turn_user);


CREATE TABLE clues (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id VARCHAR(5) NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    clue_order INT NOT NULL CHECK (clue_order >= 0),
    clue_text TEXT NOT NULL,
    clue_type TEXT,
    UNIQUE (room_id, clue_order)
);


CREATE TABLE logic_grids (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id VARCHAR(5) NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (room_id, user_id)
);


CREATE TABLE grid_cells (
    grid_id UUID NOT NULL REFERENCES logic_grids(id),
    row_index INT NOT NULL CHECK (row_index >= 0 AND row_index < 8),
    col_index INT NOT NULL CHECK (col_index >= 0 AND col_index < 8),
    value BOOLEAN,
    PRIMARY KEY (grid_id, row_index, col_index)
);


CREATE TABLE player_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id VARCHAR(5) NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    action_payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_player_actions_room ON player_actions(room_id);
CREATE INDEX idx_player_actions_user ON player_actions(user_id);

