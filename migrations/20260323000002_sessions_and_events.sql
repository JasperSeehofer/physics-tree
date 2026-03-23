-- Pre-create tower_sessions schema so tower-sessions migrate() succeeds
-- (tower-sessions will create its tables inside this schema)
CREATE SCHEMA IF NOT EXISTS tower_sessions;

-- Add display_name to users (auto-generated from email prefix, changeable later)
ALTER TABLE users ADD COLUMN display_name TEXT;

-- Add email_verified for future optional email verification (D-03)
-- Account is usable immediately; this column is future-proofed for later use
ALTER TABLE users ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT FALSE;

-- Engagement event kinds (D-10)
CREATE TYPE event_kind AS ENUM (
    'quiz_checkpoint_passed',
    'content_module_opened',
    'simulation_interacted',
    'module_completed'
);

-- Engagement events table (D-10, D-11)
CREATE TABLE engagement_events (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id     UUID REFERENCES nodes(id) ON DELETE SET NULL,
    event_kind  event_kind NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_engagement_events_user_id ON engagement_events(user_id);
CREATE INDEX idx_engagement_events_node_id ON engagement_events(node_id);
