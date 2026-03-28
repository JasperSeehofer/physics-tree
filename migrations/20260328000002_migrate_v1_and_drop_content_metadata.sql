-- Phase 9: Migrate v1.0 content_metadata rows to node_phases, then drop content_metadata

-- Insert one node_phases row per v1.0 module (phase 0, flat content marker).
-- content_body stores the file_path as a reference since v1.0 content is served from disk.
-- New 7-phase nodes will store actual Markdown in content_body.
INSERT INTO node_phases (node_id, phase_number, phase_type, content_body)
SELECT
    cm.node_id,
    0::SMALLINT,
    'schema_activation',
    cm.file_path
FROM content_metadata cm;

-- Drop the now-replaced table
DROP TABLE content_metadata;
