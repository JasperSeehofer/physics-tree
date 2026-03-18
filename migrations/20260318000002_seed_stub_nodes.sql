-- Classical mechanics stub (validates the primary branch)
INSERT INTO nodes (slug, title, node_type, branch, depth_tier)
VALUES ('newtons-second-law', 'Newton''s Second Law', 'formula', 'classical-mechanics', 'trunk');

-- Non-mechanics stubs — validates schema is branch-agnostic BEFORE locking
INSERT INTO nodes (slug, title, node_type, branch, depth_tier) VALUES
('maxwells-equations', 'Maxwell''s Equations', 'theorem', 'electromagnetism', 'root'),
('schrodinger-equation', 'Schrödinger Equation', 'theorem', 'quantum-mechanics', 'root'),
('first-law-thermodynamics', 'First Law of Thermodynamics', 'theorem', 'thermodynamics', 'root'),
('entropy', 'Entropy', 'concept', 'thermodynamics', 'trunk');

-- Edge between thermo stubs — validates edge creation across branches
INSERT INTO edges (from_node, to_node, edge_type)
SELECT
    (SELECT id FROM nodes WHERE slug = 'first-law-thermodynamics'),
    (SELECT id FROM nodes WHERE slug = 'entropy'),
    'prerequisite';
