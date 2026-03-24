-- Add FSRS memory model columns to progress table
-- last_reviewed and next_review already exist from initial schema
ALTER TABLE progress
    ADD COLUMN fsrs_stability      DOUBLE PRECISION,
    ADD COLUMN fsrs_difficulty      DOUBLE PRECISION,
    ADD COLUMN fsrs_elapsed_days    INTEGER,
    ADD COLUMN fsrs_scheduled_days  INTEGER,
    ADD COLUMN fsrs_reps            INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN fsrs_lapses          INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN fsrs_state           TEXT NOT NULL DEFAULT 'New';

-- Distinguish review XP events from initial quiz events (for diminishing returns D-08, idempotency Pitfall 5)
ALTER TABLE xp_events
    ADD COLUMN is_review BOOLEAN NOT NULL DEFAULT FALSE;

-- Index for review queue query: find due concepts quickly
CREATE INDEX idx_progress_next_review ON progress(user_id, next_review)
    WHERE next_review IS NOT NULL;
