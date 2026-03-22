---
concept_id: vectors
title: "Vectors and Vector Calculus"
prerequisites: []
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

Force, velocity, acceleration, momentum, electric fields — all of these are vector quantities. They have not just a magnitude but a direction. A force of 10 N to the right is fundamentally different from 10 N upward. Without vectors, we cannot properly describe motion in two or three dimensions, and almost all real physical problems are at least two-dimensional.

Vectors are the natural language of physics. Once you express force and acceleration as vectors, Newton's second law becomes $\vec{F} = m\vec{a}$ — a single equation that encodes three equations (one for each direction) simultaneously. Operations like dot product and cross product encode the physics of work (how much force is in the direction of motion) and torque (how much force creates rotation around an axis).

Understanding vectors means you can decompose any physical situation into independent components, solve each component separately, and reconstruct the full answer. This decomposition strategy — breaking 2D and 3D problems into 1D pieces — is one of the most powerful problem-solving tools in classical mechanics.

## Derivation {#derivation}

<div data-derivation-step="1">

**Vectors defined**

A vector $\vec{A}$ in 3D has three components $(A_x, A_y, A_z)$ in Cartesian coordinates:
$$\vec{A} = A_x\hat{i} + A_y\hat{j} + A_z\hat{k}$$

Unit vectors $\hat{i}$, $\hat{j}$, $\hat{k}$ point along the $x$, $y$, $z$ axes. The magnitude (length) of $\vec{A}$:
$$|\vec{A}| = A = \sqrt{A_x^2 + A_y^2 + A_z^2}$$

</div>

<div data-derivation-step="2">

**Vector addition and scalar multiplication**

$$\vec{A} + \vec{B} = (A_x+B_x)\hat{i} + (A_y+B_y)\hat{j} + (A_z+B_z)\hat{k}$$
$$c\vec{A} = cA_x\hat{i} + cA_y\hat{j} + cA_z\hat{k}$$

Geometrically: vector addition is tip-to-tail placement. The resultant points from the tail of the first vector to the tip of the last.

</div>

<div data-derivation-step="3">

**Dot product**

$$\vec{A} \cdot \vec{B} = A_x B_x + A_y B_y + A_z B_z = AB\cos\theta$$

where $\theta$ is the angle between the vectors. The dot product is a scalar. It measures how much $\vec{A}$ projects onto $\vec{B}$ (or vice versa). Physical application: work is $W = \vec{F} \cdot \vec{d}$ — only the force component along the displacement does work.

</div>

<div data-derivation-step="4">

**Cross product**

$$\vec{A} \times \vec{B} = (A_yB_z - A_zB_y)\hat{i} - (A_xB_z - A_zB_x)\hat{j} + (A_xB_y - A_yB_x)\hat{k}$$

Magnitude: $|\vec{A} \times \vec{B}| = AB\sin\theta$. Direction: perpendicular to both $\vec{A}$ and $\vec{B}$ (right-hand rule). Physical application: torque is $\vec{\tau} = \vec{r} \times \vec{F}$ — the cross product encodes how a force at lever arm $\vec{r}$ produces rotation.

</div>

<div data-derivation-step="5">

**Components and angle decomposition**

A 2D vector of magnitude $A$ at angle $\theta$ from the $x$-axis:
$$A_x = A\cos\theta, \qquad A_y = A\sin\theta$$

This decomposition lets you handle any direction by projecting onto coordinate axes, solve each axis independently using scalar equations, then reconstruct the magnitude and direction.

</div>

## Intuition {#intuition}

A vector is an arrow: length encodes magnitude, orientation encodes direction. Adding two vectors tip-to-tail gives the combined effect — the net displacement from two displacements, or the net force from two forces.

The dot product answers "how aligned are these two vectors?" It is maximum when they point in the same direction (cos 0° = 1), zero when perpendicular (cos 90° = 0), and negative when opposing (cos 180° = -1). Work is zero when you push perpendicular to motion (carrying a heavy box horizontally) — the force vector is vertical, the displacement is horizontal, and their dot product is zero.

The cross product answers "how perpendicular are these two vectors?" and produces a new vector perpendicular to both. The right-hand rule gives its direction: curl your fingers from $\vec{A}$ to $\vec{B}$ and your thumb points in the direction of $\vec{A} \times \vec{B}$. This direction convention is not arbitrary — it encodes a physical choice of orientation (handedness) that is consistent throughout physics.

## Examples {#examples}

**Example 1: Resolving a force into components**

A force of magnitude $F = 50\,\text{N}$ is applied at $37°$ above horizontal to push a box. Find horizontal and vertical components.

$$F_x = 50\cos 37° = 50 \times 0.799 = 39.95\,\text{N}$$
$$F_y = 50\sin 37° = 50 \times 0.602 = 30.1\,\text{N}$$

**Example 2: Work as dot product**

Force $\vec{F} = 10\hat{i} + 5\hat{j}\,\text{N}$, displacement $\vec{d} = 3\hat{i} + 0\hat{j}\,\text{m}$.

$$W = \vec{F}\cdot\vec{d} = 10 \times 3 + 5 \times 0 = 30\,\text{J}$$

The vertical component of force does no work because displacement is purely horizontal.

**Example 3: Adding two displacement vectors**

Walk $\vec{A} = 4\hat{i}\,\text{m}$ (east), then $\vec{B} = 3\hat{j}\,\text{m}$ (north). Net displacement:

$$\vec{A} + \vec{B} = 4\hat{i} + 3\hat{j}\,\text{m}$$

Magnitude: $\sqrt{16 + 9} = 5\,\text{m}$. Direction: $\arctan(3/4) = 36.9°$ north of east.

## Misconceptions {#misconceptions}

::misconception[You can add vectors by adding their magnitudes]{reveal=Vector magnitudes only add when the vectors point in the same direction. In general, you must add components separately: $(A_x + B_x)$ and $(A_y + B_y)$. A 3 N force east and a 4 N force north give a net force of 5 N (northeast), not 7 N. Always decompose into components before adding.}

::misconception[The dot product of two vectors is always positive]{reveal=The dot product $\vec{A}\cdot\vec{B} = AB\cos\theta$ is negative when $\theta > 90°$ (the vectors have an opposing component). Work is negative when force and displacement are in opposite directions (friction does negative work). The dot product can also be zero (perpendicular vectors) or negative — it's not a magnitude.}

::misconception[Vectors in physics always have three components]{reveal=Vectors can have any number of dimensions depending on context. In 1D kinematics, velocity is just a signed number (positive = forward, negative = backward). In 2D problems, vectors have two components. In 3D, three. In quantum mechanics and other advanced physics, vectors can have many more dimensions (state vectors in Hilbert space). The choice depends on the physical system.}

::misconception[A unit vector is always (1, 0, 0) or similar]{reveal=A unit vector is any vector with magnitude 1. You can create a unit vector in any direction by dividing a vector by its magnitude: $\hat{A} = \vec{A}/|\vec{A}|$. $\hat{i}$, $\hat{j}$, $\hat{k}$ are the standard basis unit vectors, but a unit vector in the direction 37° above horizontal would be $\hat{A} = \cos 37°\,\hat{i} + \sin 37°\,\hat{j}$, which has magnitude 1 but is not aligned with any axis.}

## Summary {#summary}

- **Vectors** have magnitude and direction: $\vec{A} = A_x\hat{i} + A_y\hat{j} + A_z\hat{k}$, $|\vec{A}| = \sqrt{A_x^2 + A_y^2 + A_z^2}$
- **Component decomposition**: $A_x = A\cos\theta$, $A_y = A\sin\theta$ — allows independent treatment of perpendicular directions.
- **Dot product**: $\vec{A}\cdot\vec{B} = AB\cos\theta$ — scalar, measures alignment; used for work.
- **Cross product**: $|\vec{A}\times\vec{B}| = AB\sin\theta$ — vector perpendicular to both; used for torque and angular momentum.
- **Vector addition**: add components separately; magnitudes add only when parallel.
- Vectors are the essential mathematical language of forces, velocities, and fields in physics.
