-- Expand seed graph: 30+ physics nodes across 4+ branches with prerequisite trees.
-- NOTE: Does NOT duplicate slugs from 20260318000002_seed_stub_nodes.sql:
--   newtons-second-law, maxwells-equations, schrodinger-equation,
--   first-law-thermodynamics, entropy

-- ============================================================
-- MATHEMATICS (branch: mathematics)
-- ============================================================
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('calculus', 'Calculus', 'concept', 'mathematics', 'root',
 'The mathematical study of continuous change. Provides differentiation and integration, the core language of classical and modern physics.'),
('differential-equations', 'Differential Equations', 'concept', 'mathematics', 'trunk',
 'Equations relating a function to its derivatives. Govern the time evolution of nearly every physical system.'),
('linear-algebra', 'Linear Algebra', 'concept', 'mathematics', 'root',
 'Study of vectors, matrices, and linear transformations. Essential for quantum mechanics and special relativity.'),
('vectors', 'Vectors and Vector Calculus', 'concept', 'mathematics', 'trunk',
 'Mathematical objects with magnitude and direction. The natural language for forces, fields, and physical quantities in 3D space.');

-- ============================================================
-- CLASSICAL MECHANICS (branch: classical-mechanics)
-- ============================================================

-- Root tier
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('space-and-time', 'Space and Time', 'concept', 'classical-mechanics', 'root',
 'The fundamental arena of physical phenomena. Classical mechanics treats space and time as absolute and independent.'),
('mass', 'Mass', 'concept', 'classical-mechanics', 'root',
 'A scalar measure of the amount of matter in an object and its resistance to acceleration. Central to both gravitational and inertial phenomena.');

-- Trunk tier
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('newtons-first-law', 'Newton''s First Law (Inertia)', 'theorem', 'classical-mechanics', 'trunk',
 'An object at rest stays at rest and an object in motion stays in motion with the same speed and direction unless acted upon by an unbalanced force.'),
('newtons-third-law', 'Newton''s Third Law (Action-Reaction)', 'theorem', 'classical-mechanics', 'trunk',
 'For every action there is an equal and opposite reaction. Forces always come in pairs acting on different objects.'),
('kinematics', 'Kinematics', 'concept', 'classical-mechanics', 'trunk',
 'The mathematical description of motion — position, velocity, and acceleration — without reference to the forces causing it.'),
('work-energy-theorem', 'Work-Energy Theorem', 'theorem', 'classical-mechanics', 'trunk',
 'The net work done on an object equals its change in kinetic energy. Bridges force-based and energy-based descriptions of motion.');

-- Branch tier
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('projectile-motion', 'Projectile Motion', 'application', 'classical-mechanics', 'branch',
 'Motion of an object launched into the air under gravity alone. A direct application of kinematics and Newton''s laws in two dimensions.'),
('circular-motion', 'Circular Motion', 'concept', 'classical-mechanics', 'branch',
 'Motion along a circular path. Requires a centripetal force directed toward the centre and introduces angular velocity and acceleration.'),
('simple-harmonic-motion', 'Simple Harmonic Motion', 'concept', 'classical-mechanics', 'branch',
 'Oscillatory motion where the restoring force is proportional to displacement. The simplest model for vibration in springs, pendula, and molecules.'),
('conservation-of-energy', 'Conservation of Energy', 'theorem', 'classical-mechanics', 'branch',
 'The total mechanical energy of an isolated system remains constant. Kinetic and potential energy interconvert without loss in the absence of friction.'),
('conservation-of-momentum', 'Conservation of Momentum', 'theorem', 'classical-mechanics', 'branch',
 'The total momentum of an isolated system is constant. Directly derived from Newton''s third law and the absence of external forces.'),
('friction', 'Friction', 'concept', 'classical-mechanics', 'branch',
 'A contact force opposing relative motion between surfaces. Characterised by a dimensionless coefficient that depends on the materials involved.'),
('torque', 'Torque', 'concept', 'classical-mechanics', 'branch',
 'The rotational analog of force — the tendency of a force to rotate an object about an axis. Equals the cross product of the moment arm and force vectors.'),
('angular-momentum', 'Angular Momentum', 'concept', 'classical-mechanics', 'branch',
 'The rotational analog of linear momentum. Conserved in the absence of external torques; governs planetary orbits and spinning tops.');

-- Leaf tier
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('keplers-laws', 'Kepler''s Laws of Planetary Motion', 'theorem', 'classical-mechanics', 'leaf',
 'Three empirical laws describing elliptical orbits, equal-area sweep rates, and the period-radius relationship. Derived analytically from Newton''s law of gravitation.'),
('gravitational-orbits', 'Gravitational Orbits', 'application', 'classical-mechanics', 'leaf',
 'Conic-section trajectories (ellipse, parabola, hyperbola) produced by the inverse-square gravitational force. Unifies terrestrial and celestial mechanics.'),
('damped-oscillations', 'Damped Oscillations', 'concept', 'classical-mechanics', 'leaf',
 'Oscillatory motion with energy dissipation due to friction or drag. The amplitude decays exponentially; damping ratio determines underdamped, critically damped, or overdamped behaviour.'),
('coupled-oscillators', 'Coupled Oscillators', 'concept', 'classical-mechanics', 'leaf',
 'Two or more oscillating systems connected so that energy can transfer between them. Exhibits normal modes and is the classical analog of molecular vibrations and phonons.');

-- ============================================================
-- ELECTROMAGNETISM stubs (branch: electromagnetism)
-- NOTE: maxwells-equations is already seeded in 20260318000002
-- ============================================================
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('coulombs-law', 'Coulomb''s Law', 'theorem', 'electromagnetism', 'root',
 'The electrostatic force between two point charges is proportional to the product of their charges and inversely proportional to the square of the distance between them.'),
('electric-fields', 'Electric Fields', 'concept', 'electromagnetism', 'trunk',
 'A vector field describing the force per unit charge at every point in space. Created by stationary charges; superposition of field contributions is linear.'),
('gausss-law', 'Gauss''s Law', 'theorem', 'electromagnetism', 'branch',
 'The total electric flux through a closed surface equals the enclosed free charge divided by the permittivity of free space. One of Maxwell''s four equations.');

-- ============================================================
-- THERMODYNAMICS additions (branch: thermodynamics)
-- NOTE: first-law-thermodynamics and entropy already seeded in 20260318000002
-- ============================================================
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('heat-capacity', 'Heat Capacity', 'concept', 'thermodynamics', 'trunk',
 'The amount of heat required to raise the temperature of a substance by one degree. Distinguishes constant-volume (Cv) from constant-pressure (Cp) processes.'),
('carnot-cycle', 'Carnot Cycle', 'application', 'thermodynamics', 'branch',
 'The idealised thermodynamic cycle operating between two heat reservoirs at maximum theoretical efficiency. Sets the upper bound for all real heat engines.');

-- ============================================================
-- QUANTUM MECHANICS additions (branch: quantum-mechanics)
-- NOTE: schrodinger-equation already seeded in 20260318000002
-- ============================================================
INSERT INTO nodes (slug, title, node_type, branch, depth_tier, description) VALUES
('wave-particle-duality', 'Wave-Particle Duality', 'concept', 'quantum-mechanics', 'trunk',
 'Every quantum object exhibits both wave-like and particle-like behaviour depending on the type of measurement. The double-slit experiment is the canonical demonstration.'),
('heisenberg-uncertainty', 'Heisenberg Uncertainty Principle', 'theorem', 'quantum-mechanics', 'branch',
 'The product of the uncertainties in position and momentum for any quantum state is at least ℏ/2. A fundamental limit arising from the wave nature of matter, not measurement error.');

-- ============================================================
-- EDGES — prerequisite relationships
-- ============================================================

-- Mathematics internal prerequisites
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'calculus'),
    (SELECT id FROM nodes WHERE slug = 'differential-equations'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'vectors'),
    (SELECT id FROM nodes WHERE slug = 'differential-equations'),
    'prerequisite', 0.8;

-- Classical mechanics root -> trunk
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'space-and-time'),
    (SELECT id FROM nodes WHERE slug = 'kinematics'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'mass'),
    (SELECT id FROM nodes WHERE slug = 'newtons-first-law'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'kinematics'),
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-first-law'),
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    (SELECT id FROM nodes WHERE slug = 'work-energy-theorem'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    (SELECT id FROM nodes WHERE slug = 'newtons-third-law'),
    'prerequisite', 0.8;

-- Classical mechanics trunk -> branch
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'kinematics'),
    (SELECT id FROM nodes WHERE slug = 'projectile-motion'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    (SELECT id FROM nodes WHERE slug = 'circular-motion'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    (SELECT id FROM nodes WHERE slug = 'simple-harmonic-motion'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'work-energy-theorem'),
    (SELECT id FROM nodes WHERE slug = 'conservation-of-energy'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-third-law'),
    (SELECT id FROM nodes WHERE slug = 'conservation-of-momentum'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    (SELECT id FROM nodes WHERE slug = 'friction'),
    'prerequisite', 0.9;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    (SELECT id FROM nodes WHERE slug = 'torque'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'torque'),
    (SELECT id FROM nodes WHERE slug = 'angular-momentum'),
    'prerequisite', 1.0;

-- Classical mechanics branch -> leaf
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'circular-motion'),
    (SELECT id FROM nodes WHERE slug = 'keplers-laws'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'conservation-of-energy'),
    (SELECT id FROM nodes WHERE slug = 'gravitational-orbits'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'angular-momentum'),
    (SELECT id FROM nodes WHERE slug = 'gravitational-orbits'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'simple-harmonic-motion'),
    (SELECT id FROM nodes WHERE slug = 'damped-oscillations'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'simple-harmonic-motion'),
    (SELECT id FROM nodes WHERE slug = 'coupled-oscillators'),
    'prerequisite', 1.0;

-- Electromagnetism prerequisites
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'coulombs-law'),
    (SELECT id FROM nodes WHERE slug = 'electric-fields'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'electric-fields'),
    (SELECT id FROM nodes WHERE slug = 'gausss-law'),
    'prerequisite', 1.0;

-- Thermodynamics prerequisites
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'heat-capacity'),
    (SELECT id FROM nodes WHERE slug = 'carnot-cycle'),
    'prerequisite', 0.9;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'first-law-thermodynamics'),
    (SELECT id FROM nodes WHERE slug = 'carnot-cycle'),
    'prerequisite', 1.0;

-- Quantum mechanics prerequisites
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'wave-particle-duality'),
    (SELECT id FROM nodes WHERE slug = 'heisenberg-uncertainty'),
    'prerequisite', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'heisenberg-uncertainty'),
    (SELECT id FROM nodes WHERE slug = 'schrodinger-equation'),
    'prerequisite', 1.0;

-- ============================================================
-- EDGES — mathematical_foundation relationships
-- ============================================================

-- Calculus underpins many physics derivations
INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'calculus'),
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    'mathematical_foundation', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'calculus'),
    (SELECT id FROM nodes WHERE slug = 'work-energy-theorem'),
    'mathematical_foundation', 0.9;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'differential-equations'),
    (SELECT id FROM nodes WHERE slug = 'simple-harmonic-motion'),
    'mathematical_foundation', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'differential-equations'),
    (SELECT id FROM nodes WHERE slug = 'damped-oscillations'),
    'mathematical_foundation', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'vectors'),
    (SELECT id FROM nodes WHERE slug = 'newtons-second-law'),
    'mathematical_foundation', 0.9;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'vectors'),
    (SELECT id FROM nodes WHERE slug = 'electric-fields'),
    'mathematical_foundation', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'linear-algebra'),
    (SELECT id FROM nodes WHERE slug = 'schrodinger-equation'),
    'mathematical_foundation', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'calculus'),
    (SELECT id FROM nodes WHERE slug = 'gausss-law'),
    'mathematical_foundation', 0.9;

-- ============================================================
-- EDGES — derives_from relationships
-- ============================================================

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'conservation-of-energy'),
    (SELECT id FROM nodes WHERE slug = 'work-energy-theorem'),
    'derives_from', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'keplers-laws'),
    (SELECT id FROM nodes WHERE slug = 'gravitational-orbits'),
    'derives_from', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'damped-oscillations'),
    (SELECT id FROM nodes WHERE slug = 'simple-harmonic-motion'),
    'derives_from', 1.0;

INSERT INTO edges (from_node, to_node, edge_type, weight)
SELECT
    (SELECT id FROM nodes WHERE slug = 'gausss-law'),
    (SELECT id FROM nodes WHERE slug = 'coulombs-law'),
    'derives_from', 0.9;
