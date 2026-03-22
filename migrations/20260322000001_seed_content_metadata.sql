-- Seed content_metadata for all 15 classical mechanics concepts.
-- Sets review_status to 'approved' so content API serves them immediately.
-- File paths match the content/classical-mechanics/ directory structure.

-- Add unique constraint if not present (content_metadata may not have it yet)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'content_metadata_node_id_key') THEN
        ALTER TABLE content_metadata ADD CONSTRAINT content_metadata_node_id_key UNIQUE (node_id);
    END IF;
END $$;

-- Seed content_metadata for all 15 classical mechanics concepts
INSERT INTO content_metadata (node_id, file_path, review_status, approved_at)
SELECT n.id, 'content/classical-mechanics/' || n.slug || '.md', 'approved', NOW()
FROM nodes n
WHERE n.slug IN (
    'newtons-first-law', 'newtons-second-law', 'newtons-third-law',
    'kinematics', 'projectile-motion', 'circular-motion',
    'work-energy-theorem', 'conservation-of-energy', 'conservation-of-momentum',
    'simple-harmonic-motion', 'mass', 'space-and-time',
    'vectors', 'calculus', 'differential-equations'
)
ON CONFLICT (node_id) DO UPDATE
SET file_path = EXCLUDED.file_path,
    review_status = EXCLUDED.review_status,
    approved_at = EXCLUDED.approved_at;
