-- M14 — glossary terms, branch conventions, pins and peek events
-- (content-spec v1.5).
--
-- Five tables in two groups, and the split is the whole design:
--
--   content-derived  glossary_terms · glossary_term_tags · branch_conventions
--   learner-derived  user_glossary_pins · glossary_peek_events
--
-- Ingest is the only writer of the first group and the server the only reader,
-- exactly as node_phases and node_probes already work. The server never reads
-- `content/` at request time, so an in-memory index built at startup is not an
-- option and these tables are not a cache.
--
-- Naming, nullability and index style follow user_phase_progress — composite
-- primary keys, no surrogate UUIDs where the natural key is complete, cascade
-- delete, one timestamp.

-- 1. Term records. Owned by the node that first defines the term, which is why
--    `node_id` is here and there is no `defined_by` field anywhere in content:
--    "defined by" is structural. Keys are branch-scoped on purpose —
--    `metric-signature` genuinely differs between the quantum-field-theory and
--    general-relativity branches, and they are distinct terms.
CREATE TABLE glossary_terms (
    branch         TEXT NOT NULL,
    term_key       TEXT NOT NULL,
    node_id        UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    term           TEXT NOT NULL,
    symbol         TEXT,           -- KaTeX source, never rendered HTML
    units          TEXT,           -- '—' for dimensionless
    definition     TEXT NOT NULL,  -- spoiler surface: never served locked
    caveat         TEXT,           -- amber slot; where convention traps live
    teaser         TEXT,           -- the non-spoiling one-liner
    convention_row TEXT,           -- joins the card to branch_conventions.row_key
    PRIMARY KEY (branch, term_key)
);

CREATE INDEX idx_glossary_terms_node ON glossary_terms(node_id);

-- 2. The tag index — every `::term[key]` occurrence, as (branch, key, node,
--    phase). This is what makes unlock derived rather than authored: a term is
--    unlocked once the learner has completed at least one (node, phase) in
--    which it is tagged, so "you were taught it where you met it" is literally
--    true and no `first_taught` field can go stale. A book has one linear
--    position; a knowledge graph does not.
CREATE TABLE glossary_term_tags (
    branch       TEXT NOT NULL,
    term_key     TEXT NOT NULL,
    node_id      UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number SMALLINT NOT NULL CHECK (phase_number BETWEEN 0 AND 6),
    PRIMARY KEY (branch, term_key, node_id, phase_number)
);

CREATE INDEX idx_glossary_term_tags_node_phase
    ON glossary_term_tags(node_id, phase_number);

-- 3. The branch conventions table. Authored as a *branch* object because rows
--    are opened by one node and closed by another: node 1 opens
--    `state-normalization` and leaves it deliberately unfixed; node 5 closes it.
--    A per-node block cannot express that without a merge pass.
--
--    Both the slug and the resolved id are stored. The id carries the join; the
--    slug carries the display and survives a row that names a node not yet
--    ingested — which is the normal state of a branch under construction, and
--    the reason the id columns are nullable.
CREATE TABLE branch_conventions (
    branch         TEXT NOT NULL,
    row_key        TEXT NOT NULL,
    sort_order     INTEGER NOT NULL,
    object         TEXT NOT NULL,
    this_branch    TEXT NOT NULL,
    also_common    TEXT,
    status         TEXT NOT NULL,   -- free | forced | not_independent | convention_independent | open
    status_note    TEXT,
    opened_by_slug TEXT NOT NULL,
    closed_by_slug TEXT NOT NULL,
    opened_by      UUID REFERENCES nodes(id) ON DELETE SET NULL,
    closed_by      UUID REFERENCES nodes(id) ON DELETE SET NULL,
    PRIMARY KEY (branch, row_key)
);

-- 4. Pins. Follows user_phase_progress exactly: composite PK, cascade delete,
--    one timestamp, idempotent insert.
--
--    `term_key` is TEXT and deliberately not a foreign key. Terms live in
--    content files and content gets re-ingested; a pin to a renamed key is
--    tolerated and filtered on read, rather than being a delete that silently
--    loses the learner's own annotation.
CREATE TABLE user_glossary_pins (
    user_id   UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    branch    TEXT        NOT NULL,
    term_key  TEXT        NOT NULL,
    pinned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, branch, term_key)
);

CREATE INDEX idx_user_glossary_pins_user ON user_glossary_pins(user_id);

-- 5. Peek events — Gate-9 decision D-G9c, peek-with-logging over a hard lock.
--
--    Every panel open and every card view in a closed-book context (phase 5, and
--    the phase-0 calibration-probe section) writes a row. The argument is not
--    that a lock is unenforceable — it is that **which term the learner reached
--    for is a direct read on which production is missing**, at exactly the
--    granularity the node's misconception ledger consumes, and a lock measures
--    nothing. The real closed-book instrument is the paper sitting; an in-app
--    lock buys little against a textbook on the desk, and the log buys a signal
--    nothing else produces.
--
--    `term_key` NULL distinguishes a panel open from a card view. Append-only:
--    nothing here is ever updated, and the row is evidence, not state.
--
--    This is a new category of personal data. The physics-tree vault page's
--    Transparency → Data Processing table needs a row for it (M14a §7 risk 7).
CREATE TABLE glossary_peek_events (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id      UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number SMALLINT NOT NULL CHECK (phase_number BETWEEN 0 AND 6),
    term_key     TEXT,             -- NULL = the panel was opened, no card viewed
    occurred_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_glossary_peek_events_user_node_phase
    ON glossary_peek_events(user_id, node_id, phase_number, occurred_at DESC);
