-- Pedagogical node types — branch-agnostic by design
CREATE TYPE node_type AS ENUM (
    'concept',
    'formula',
    'theorem',
    'application',
    'consequence'
);

-- Typed edges — enables different visual treatments per relationship
CREATE TYPE edge_type AS ENUM (
    'prerequisite',
    'derives_from',
    'applies_to',
    'mathematical_foundation'
);

-- Content review pipeline status
CREATE TYPE review_status AS ENUM (
    'draft',
    'under_review',
    'approved'
);

-- Physics knowledge graph nodes
CREATE TABLE nodes (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug        TEXT UNIQUE NOT NULL,
    title       TEXT NOT NULL,
    node_type   node_type NOT NULL,
    branch      TEXT NOT NULL,
    depth_tier  TEXT NOT NULL,
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Physics knowledge graph edges
CREATE TABLE edges (
    from_node   UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    to_node     UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    edge_type   edge_type NOT NULL,
    weight      REAL NOT NULL DEFAULT 1.0,
    PRIMARY KEY (from_node, to_node, edge_type)
);

-- Content metadata (content body lives in files on disk)
CREATE TABLE content_metadata (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    file_path       TEXT NOT NULL,
    review_status   review_status NOT NULL DEFAULT 'draft',
    reviewer        TEXT,
    approved_at     TIMESTAMPTZ,
    content_hash    TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users (schema only — auth implementation in Phase 4)
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           TEXT UNIQUE NOT NULL,
    password_hash   TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Progress tracking (schema only — logic in Phase 4)
CREATE TABLE progress (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    mastery_level   INTEGER NOT NULL DEFAULT 0,
    xp_earned       INTEGER NOT NULL DEFAULT 0,
    last_reviewed   TIMESTAMPTZ,
    next_review     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, node_id)
);

-- Indexes for graph traversal performance
CREATE INDEX idx_edges_from_node ON edges(from_node);
CREATE INDEX idx_edges_to_node ON edges(to_node);
CREATE INDEX idx_nodes_branch ON nodes(branch);
CREATE INDEX idx_progress_user_id ON progress(user_id);
