//! Particle effects for campfire, ambient atmosphere, and visual feedback.
//!
//! Uses simple entity-based particles with lifetime, velocity, and fade-out.
//! Quality tier controls particle count and effect complexity.

use bevy::prelude::*;
use crate::graphics_settings::{GraphicsSettings, QualityTier};

/// A particle with lifetime and movement.
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub fade_out: bool,
}

/// Tags the campfire emitter location.
#[derive(Component)]
pub struct CampfireEmitter {
    pub timer: f32,
    pub spawn_interval: f32,
}

/// Tags ambient dust mote particles.
#[derive(Component)]
pub struct DustMote;

/// Spawn the campfire particle emitter at the campfire location.
pub fn spawn_campfire_emitter(commands: &mut Commands, position: Vec3, settings: &GraphicsSettings) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let interval = match settings.quality_tier {
        QualityTier::Low => return,
        QualityTier::Medium => 0.15,
        QualityTier::High => 0.08,
        QualityTier::Ultra => 0.04,
    };

    commands.spawn((
        Transform::from_translation(position),
        CampfireEmitter {
            timer: 0.0,
            spawn_interval: interval,
        },
    ));
}

/// Spawn ambient dust motes in the area around the player.
pub fn spawn_dust_motes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    settings: &GraphicsSettings,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let count = match settings.quality_tier {
        QualityTier::Low => return,
        QualityTier::Medium => 15,
        QualityTier::High => 30,
        QualityTier::Ultra => 50,
    };

    let mote_mesh = meshes.add(Sphere::new(0.08));
    let mote_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.98, 0.9, 0.3),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let mut seed: u32 = 777;
    let next_rand = |s: &mut u32| -> f32 {
        *s = s.wrapping_mul(1103515245).wrapping_add(12345);
        ((*s >> 16) & 0x7FFF) as f32 / 32767.0
    };

    for _ in 0..count {
        let x = (next_rand(&mut seed) - 0.5) * 100.0;
        let y = 2.0 + next_rand(&mut seed) * 20.0;
        let z = (next_rand(&mut seed) - 0.5) * 100.0;

        let vx = (next_rand(&mut seed) - 0.5) * 0.3;
        let vy = (next_rand(&mut seed) - 0.5) * 0.15;
        let vz = (next_rand(&mut seed) - 0.5) * 0.3;

        let lifetime = 8.0 + next_rand(&mut seed) * 12.0;

        commands.spawn((
            Mesh3d(mote_mesh.clone()),
            MeshMaterial3d(mote_mat.clone()),
            Transform::from_xyz(x, y, z),
            Particle {
                velocity: Vec3::new(vx, vy, vz),
                lifetime,
                max_lifetime: lifetime,
                fade_out: true,
            },
            DustMote,
        ));
    }
}

/// Campfire emitter system: spawns fire spark and smoke particles.
pub fn campfire_emitter_system(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<GraphicsSettings>,
    mut emitter_q: Query<(&Transform, &mut CampfireEmitter)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let dt = time.delta_secs();
    let t = time.elapsed_secs();

    let spark_mesh = meshes.add(Sphere::new(0.12));
    let smoke_mesh = meshes.add(Sphere::new(0.4));

    for (transform, mut emitter) in emitter_q.iter_mut() {
        emitter.timer += dt;

        if emitter.timer < emitter.spawn_interval {
            continue;
        }
        emitter.timer -= emitter.spawn_interval;

        let base = transform.translation;

        // Pseudo-random offset
        let rx = (t * 17.3).sin() * 0.8;
        let rz = (t * 23.7).cos() * 0.8;

        // Fire spark (small, bright, fast upward)
        let spark_mat = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.7, 0.1, 0.9),
            emissive: LinearRgba::new(2.0, 1.0, 0.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        commands.spawn((
            Mesh3d(spark_mesh.clone()),
            MeshMaterial3d(spark_mat),
            Transform::from_xyz(base.x + rx, base.y + 1.5, base.z + rz),
            Particle {
                velocity: Vec3::new(rx * 0.3, 2.5 + (t * 5.0).sin().abs() * 1.0, rz * 0.3),
                lifetime: 1.2,
                max_lifetime: 1.2,
                fade_out: true,
            },
        ));

        // Smoke (larger, slower, rises and spreads)
        if (t * 3.0).sin() > 0.0 {
            let smoke_mat = materials.add(StandardMaterial {
                base_color: Color::srgba(0.3, 0.3, 0.32, 0.2),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            });

            commands.spawn((
                Mesh3d(smoke_mesh.clone()),
                MeshMaterial3d(smoke_mat),
                Transform::from_xyz(base.x + rx * 0.5, base.y + 3.0, base.z + rz * 0.5),
                Particle {
                    velocity: Vec3::new(rx * 0.1 + 0.2, 1.0, rz * 0.1),
                    lifetime: 3.0,
                    max_lifetime: 3.0,
                    fade_out: true,
                },
            ));
        }
    }
}

/// Update all particles: move, age, fade, despawn.
pub fn particle_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_q: Query<(Entity, &mut Particle, &mut Transform, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_secs();

    for (entity, mut particle, mut transform, mat_handle) in particle_q.iter_mut() {
        // Age
        particle.lifetime -= dt;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Move
        transform.translation += particle.velocity * dt;

        // Fade out alpha based on remaining lifetime
        if particle.fade_out {
            let alpha = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                let c = mat.base_color.to_srgba();
                mat.base_color = Color::srgba(c.red, c.green, c.blue, alpha * 0.9);
            }
        }

        // Smoke particles grow as they rise
        if particle.velocity.y > 0.5 && particle.velocity.y < 1.5 {
            let age_ratio = 1.0 - particle.lifetime / particle.max_lifetime;
            let scale = 1.0 + age_ratio * 2.0;
            transform.scale = Vec3::splat(scale);
        }
    }
}

/// Dust motes drift gently and respawn when expired.
pub fn dust_mote_system(
    time: Res<Time>,
    settings: Res<GraphicsSettings>,
    mut mote_q: Query<(&mut Particle, &mut Transform), With<DustMote>>,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let t = time.elapsed_secs();

    for (mut particle, mut transform) in mote_q.iter_mut() {
        // Gentle drifting with slight sine wave
        let drift_x = (t * 0.3 + transform.translation.z * 0.1).sin() * 0.02;
        let drift_y = (t * 0.2 + transform.translation.x * 0.1).cos() * 0.01;

        transform.translation += particle.velocity * time.delta_secs();
        transform.translation.x += drift_x;
        transform.translation.y += drift_y;

        // Respawn when expired (loop instead of despawn)
        particle.lifetime -= time.delta_secs();
        if particle.lifetime <= 0.0 {
            particle.lifetime = particle.max_lifetime;
            // Reset position near origin
            transform.translation.x = (t * 17.3 + transform.translation.z).sin() * 50.0;
            transform.translation.y = 2.0 + (t * 7.1).sin().abs() * 18.0;
            transform.translation.z = (t * 23.7 + transform.translation.x).cos() * 50.0;
        }
    }
}
