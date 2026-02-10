//! Simple physics and collision detection system.

use bevy::prelude::*;
use crate::TerrainData;
use crate::terrain_mesh;

/// A simple 3D collider component
#[derive(Component, Clone, Copy, Debug)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    /// Check if this collider overlaps with another
    pub fn overlaps(&self, pos1: Vec3, other: &Collider, pos2: Vec3) -> bool {
        let distance = pos1.distance(pos2);
        distance < (self.radius + other.radius)
    }
}

/// A physics component that handles movement and gravity
#[derive(Component, Debug)]
pub struct RigidBody {
    pub velocity: Vec3,
    pub use_gravity: bool,
    pub mass: f32,
}

impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            use_gravity: true,
            mass,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.velocity += force / self.mass;
    }
}

const GRAVITY: f32 = -9.81;
const GROUND_OFFSET: f32 = 1.0; // Entity hover above terrain surface

/// Update physics each frame
pub fn physics_system(
    mut query: Query<(&mut Transform, &mut RigidBody, Option<&Collider>)>,
    time: Res<Time>,
    terrain_data: Option<Res<TerrainData>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut body, _collider) in query.iter_mut() {
        // Apply gravity
        if body.use_gravity {
            body.velocity.y += GRAVITY * dt;
        }

        // Update position based on velocity
        transform.translation += body.velocity * dt;

        // Ground collision - use terrain height if available, else flat
        let ground_level = if let Some(ref data) = terrain_data {
            terrain_mesh::get_terrain_height(
                &data.generator,
                transform.translation.x,
                transform.translation.z,
                data.base_offset,
            ) + GROUND_OFFSET
        } else {
            GROUND_OFFSET
        };

        if transform.translation.y < ground_level {
            transform.translation.y = ground_level;
            body.velocity.y = 0.0;
        }

        // Simple air resistance
        body.velocity *= 0.99;
    }
}

/// Detect collisions between entities
pub fn collision_system(
    mut query: Query<(&Transform, &Collider, Entity)>,
    mut collision_events: MessageWriter<CollisionEvent>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(transform1, collider1, entity1), (transform2, collider2, entity2)]) =
        combinations.fetch_next()
    {
        if collider1.overlaps(transform1.translation, collider2, transform2.translation) {
            collision_events.write(CollisionEvent {
                entity1,
                entity2,
            });
        }
    }
}

/// Message fired when two entities collide
#[derive(Message, Clone, Copy)]
pub struct CollisionEvent {
    pub entity1: Entity,
    pub entity2: Entity,
}

/// Prevent entities from walking through each other
pub fn collision_response_system(
    mut collision_events: MessageReader<CollisionEvent>,
    mut query: Query<(&Transform, &Collider, &mut RigidBody)>,
) {
    for event in collision_events.read() {
        if let Ok([(transform1, collider1, mut body1), (transform2, collider2, mut body2)]) =
            query.get_many_mut([event.entity1, event.entity2])
        {
            // Simple push apart
            let diff = transform2.translation - transform1.translation;
            let distance = diff.length();
            let min_distance = collider1.radius + collider2.radius;

            if distance < min_distance && distance > 0.0 {
                let push = (min_distance - distance) / 2.0;
                let direction = diff.normalize();

                // Push both bodies apart
                body1.velocity -= direction * push * 0.5;
                body2.velocity += direction * push * 0.5;
            }
        }
    }
}
