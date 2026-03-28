---
phase: 0
type: schema_activation
estimated_minutes: 6
---

## Recall Prompt

Think about watching a car drive past. What quantities would you need to fully describe how the car is moving at any moment?

List every physical quantity you can think of that relates to how an object moves. For each one, write down whether it has a direction or just a magnitude.

Now think back to your experience with algebra and basic calculus: if you know how position changes with time, how could you figure out how fast the position is changing?

## Linkage Map

This node builds directly on:

- **Vectors** (`vectors`): Velocity and acceleration are vector quantities — they have both magnitude and direction. Kinematics in 2D and 3D requires component-wise treatment.
- **Calculus** (`calculus`): Velocity is the derivative of position ($v = dx/dt$); acceleration is the derivative of velocity ($a = dv/dt$). Integration reverses this: given constant acceleration, you integrate to get velocity and position.

After completing this node, you will use kinematics in:

- `newtons-second-law` (forces cause the acceleration that kinematics describes)
- `projectile-motion` (2D kinematics with constant downward acceleration)
- `circular-motion` (velocity direction changes even at constant speed)

## Wonder Hook

A car brakes to a stop. A ball is thrown upward and falls back down. A satellite orbits Earth. These three situations look completely different — yet all of them are fully described by the same handful of equations derived from two simple definitions.

How can three such different physical situations share the same mathematical description? And here is the surprising part: the equations work regardless of *why* the object moves the way it does — they describe motion without any reference to forces, masses, or causes.

What minimal information do you need to predict exactly where an object will be at any future time, given that acceleration is constant? By the end of this node you will be able to derive the answer from first principles.
