-- M13 — probe capture and phase time telemetry (content-spec v1.4).
--
-- Four new tables, one new column. No existing table's semantics change.
-- Naming, nullability and index style follow user_phase_progress and xp_events.
--
-- `engagement_events` is deliberately left in place, unused: it has never held a
-- row, dropping it buys nothing, and a DROP TABLE is the kind of thing that
-- should be ratified rather than tucked into a feature migration. The dead
-- POST /api/progress/event route that wrote to it is retired in this mission.

-- 1. The parsed probe spec, stored the way node_phases stores phase markdown:
--    ingest is the only writer, the server is the only reader.
CREATE TABLE node_probes (
    node_id      UUID PRIMARY KEY REFERENCES nodes(id) ON DELETE CASCADE,
    spec_version TEXT NOT NULL,
    spec         JSONB NOT NULL,          -- the whole ProbeSpec, serde round-trip
    spec_digest  TEXT NOT NULL,           -- sha256 of probe.yaml; pins a sitting to a revision
    -- The node's effective `relaxation` at ingest time. The routing engine needs
    -- it to apply the narrowing invariant, and node.yaml is not otherwise
    -- reachable from the server at request time.
    relaxation   TEXT NOT NULL DEFAULT 'on',
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. One row per paper sitting of a node's probe. Re-sittings are allowed and
--    ordered by entered_at; "current verdict" means the latest row.
--
--    The verdict is frozen here rather than recomputed on read: routing rules
--    are content and content gets edited, so a verdict recomputed six months
--    later against a revised probe.yaml is not the verdict the learner acted on.
--    spec_digest + verdict_engine make that drift visible without silently
--    rewriting history — the same instinct as the is_review flag on xp_events.
CREATE TABLE probe_sittings (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id        UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    sat_on         DATE NOT NULL,               -- the paper sitting's date
    paper_minutes  SMALLINT,                    -- from "write your start and stop times"
    spec_digest    TEXT NOT NULL,               -- which revision was sat
    verdict        JSONB NOT NULL,              -- computed ProbeVerdict, frozen at entry
    verdict_engine SMALLINT NOT NULL,           -- engine version; lets a re-eval be detected
    note           TEXT,
    entered_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_probe_sittings_user_node
    ON probe_sittings(user_id, node_id, entered_at DESC);

-- 3. Per-item scores. score NULL = item left blank (node 3's item 3 expects
--    this, and blank is not zero). correct NULL = the item is not
--    correctness-gated, or correctness was not judged.
CREATE TABLE probe_item_scores (
    sitting_id UUID NOT NULL REFERENCES probe_sittings(id) ON DELETE CASCADE,
    item_id    TEXT NOT NULL,
    score      SMALLINT CHECK (score BETWEEN 0 AND 3),
    correct    BOOLEAN,
    PRIMARY KEY (sitting_id, item_id)
);

-- 4. Phase-level time telemetry. One row per contiguous working session; a phase
--    accumulates many. active_seconds excludes idle and hidden-tab time.
CREATE TYPE phase_session_source AS ENUM ('timer', 'manual');

CREATE TABLE phase_sessions (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id        UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number   SMALLINT NOT NULL CHECK (phase_number BETWEEN 0 AND 6),
    started_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_beat_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at      TIMESTAMPTZ,                 -- NULL = open (or lost)
    active_seconds INTEGER NOT NULL DEFAULT 0,
    source         phase_session_source NOT NULL DEFAULT 'timer',
    note           TEXT
);

CREATE INDEX idx_phase_sessions_user_node
    ON phase_sessions(user_id, node_id, phase_number);

-- 5. Per-phase estimates: parsed and validated since v1.1 (check 14), dropped at
--    ingest ever since. Without this the pace dashboard can only compare at node
--    granularity, and the interesting question is which phase overruns.
ALTER TABLE node_phases ADD COLUMN estimated_minutes SMALLINT;
