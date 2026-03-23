-- Seed content_metadata for gravitational-orbits concept (gap closure).
-- Adds the gravity module that was omitted from the original 15-slug seed in 20260322000001.

INSERT INTO content_metadata (node_id, file_path, review_status, approved_at)
SELECT n.id, 'content/classical-mechanics/' || n.slug || '.md', 'approved', NOW()
FROM nodes n
WHERE n.slug = 'gravitational-orbits'
ON CONFLICT (node_id) DO UPDATE
SET file_path = EXCLUDED.file_path,
    review_status = EXCLUDED.review_status,
    approved_at = EXCLUDED.approved_at;
