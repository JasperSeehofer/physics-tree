-- Add has_phases column to nodes table (per D-10 / UI-04)
ALTER TABLE nodes ADD COLUMN has_phases BOOLEAN NOT NULL DEFAULT FALSE;

-- Set has_phases=true for nodes that have entries in node_phases
UPDATE nodes SET has_phases = TRUE
WHERE id IN (SELECT DISTINCT node_id FROM node_phases);

-- Phase progress table (per D-24)
CREATE TABLE user_phase_progress (
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id      UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number SMALLINT NOT NULL,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    format_pref  TEXT NOT NULL DEFAULT 'reading',
    PRIMARY KEY (user_id, node_id, phase_number)
);

CREATE INDEX idx_user_phase_progress_user_node
    ON user_phase_progress(user_id, node_id);
