-- Phase 9: Create node_phases table and add metadata columns to nodes

-- Per D-01: one row per (node_id, phase_number) with UNIQUE constraint
CREATE TABLE node_phases (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number    SMALLINT NOT NULL,
    phase_type      TEXT NOT NULL,
    content_body    TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(node_id, phase_number)
);

CREATE INDEX idx_node_phases_node_id ON node_phases(node_id);

-- Per D-03: node-level metadata from node.yaml stored on nodes table
-- All columns nullable because existing v1.0 nodes won't have this metadata
ALTER TABLE nodes
    ADD COLUMN eqf_level SMALLINT,
    ADD COLUMN bloom_minimum TEXT,
    ADD COLUMN estimated_minutes SMALLINT,
    ADD COLUMN derivation_required BOOLEAN,
    ADD COLUMN misconceptions TEXT[],
    ADD COLUMN domain_of_applicability TEXT[],
    ADD COLUMN esco_tags TEXT[];
