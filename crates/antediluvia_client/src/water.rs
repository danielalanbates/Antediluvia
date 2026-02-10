//! Water rendering system with animated waves, shimmer, and shore foam.
//!
//! Quality tier scaling:
//! - Low:   Static flat plane, no animation
//! - Medium: Grid mesh with wave displacement, shimmer
//! - High:  Higher resolution waves, shore foam, reflectance
//! - Ultra: Maximum vertex density, full effects

use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use crate::graphics_settings::{GraphicsSettings, QualityTier};

/// Tags a water surface entity for animation.
#[derive(Component)]
pub struct WaterBody {
    pub wave_amplitude: f32,
    pub wave_frequency: f32,
    pub wave_speed: f32,
    pub time: f32,
}

/// Tags shore foam particles.
#[derive(Component)]
pub struct ShoreFoam {
    pub phase: f32,
}

/// Generate a subdivided water plane mesh for vertex wave animation.
pub fn generate_water_mesh(width: f32, length: f32, segments_x: u32, segments_z: u32) -> Mesh {
    let verts_x = segments_x + 1;
    let verts_z = segments_z + 1;
    let vertex_count = (verts_x * verts_z) as usize;

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);

    let half_w = width / 2.0;
    let half_l = length / 2.0;

    for z in 0..verts_z {
        for x in 0..verts_x {
            let px = -half_w + (x as f32 / segments_x as f32) * width;
            let pz = -half_l + (z as f32 / segments_z as f32) * length;
            positions.push([px, 0.0, pz]);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([x as f32 / segments_x as f32, z as f32 / segments_z as f32]);
        }
    }

    let mut indices = Vec::with_capacity((segments_x * segments_z * 6) as usize);
    for z in 0..segments_z {
        for x in 0..segments_x {
            let tl = z * verts_x + x;
            let tr = tl + 1;
            let bl = (z + 1) * verts_x + x;
            let br = bl + 1;
            indices.push(tl);
            indices.push(bl);
            indices.push(tr);
            indices.push(tr);
            indices.push(bl);
            indices.push(br);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Spawn the river with animated water mesh and shore foam.
pub fn spawn_river(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    settings: &GraphicsSettings,
) {
    let (seg_x, seg_z) = match settings.quality_tier {
        QualityTier::Low => (1, 1),      // Single quad, no animation
        QualityTier::Medium => (6, 40),
        QualityTier::High => (10, 60),
        QualityTier::Ultra => (14, 80),
    };

    let amplitude = match settings.quality_tier {
        QualityTier::Low => 0.0,
        QualityTier::Medium => 0.15,
        QualityTier::High => 0.25,
        QualityTier::Ultra => 0.3,
    };

    let water_mesh = generate_water_mesh(28.0, 700.0, seg_x, seg_z);

    let water_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.08, 0.28, 0.55, 0.78),
        metallic: 0.7,
        perceptual_roughness: 0.05,
        reflectance: match settings.quality_tier {
            QualityTier::Low => 0.3,
            QualityTier::Medium => 0.5,
            QualityTier::High | QualityTier::Ultra => 0.8,
        },
        alpha_mode: AlphaMode::Blend,
        double_sided: true,
        ..default()
    });

    let river_rot = Quat::from_rotation_y(std::f32::consts::PI / 5.0);

    commands.spawn((
        Mesh3d(meshes.add(water_mesh)),
        MeshMaterial3d(water_mat),
        Transform::from_xyz(-60.0, -1.5, 0.0).with_rotation(river_rot),
        WaterBody {
            wave_amplitude: amplitude,
            wave_frequency: 1.5,
            wave_speed: 2.0,
            time: 0.0,
        },
        Name::new("River"),
    ));

    // Shore foam (Medium+ only)
    if settings.quality_tier != QualityTier::Low {
        spawn_shore_foam(commands, meshes, materials, settings);
    }
}

/// Spawn foam patches along the river banks.
fn spawn_shore_foam(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    settings: &GraphicsSettings,
) {
    let foam_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.92, 0.95, 0.35),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        ..default()
    });

    let foam_mesh = meshes.add(Sphere::new(1.2));

    // Place foam along both banks of the river
    let river_angle: f32 = std::f32::consts::PI / 5.0;
    let cos_a = river_angle.cos();
    let sin_a = river_angle.sin();
    let river_cx = -60.0f32;

    let foam_count = match settings.quality_tier {
        QualityTier::Low => 0,
        QualityTier::Medium => 12,
        QualityTier::High => 20,
        QualityTier::Ultra => 28,
    };

    let spacing = 700.0 / foam_count.max(1) as f32;

    for i in 0..foam_count {
        let along = -350.0 + i as f32 * spacing + (i as f32 * 7.3).sin() * 8.0;

        // Both sides of river
        for side in [-1.0f32, 1.0] {
            let across = side * 15.0 + (i as f32 * 3.7).sin() * 3.0;

            // Rotate from river-local to world coordinates
            let wx = river_cx + across * cos_a - along * sin_a;
            let wz = across * sin_a + along * cos_a;

            commands.spawn((
                Mesh3d(foam_mesh.clone()),
                MeshMaterial3d(foam_mat.clone()),
                Transform::from_xyz(wx, -1.2, wz)
                    .with_scale(Vec3::new(2.5, 0.15, 1.8)),
                ShoreFoam { phase: i as f32 * 0.5 },
            ));
        }
    }
}

/// Animate water surface with multi-octave sine waves (Medium+).
pub fn water_animation_system(
    time: Res<Time>,
    settings: Res<GraphicsSettings>,
    mut water_q: Query<(&mut WaterBody, &Mesh3d, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let dt = time.delta_secs();

    for (mut water, mesh3d, transform) in water_q.iter_mut() {
        water.time += dt;
        let t = water.time;

        let Some(mesh) = meshes.get_mut(&mesh3d.0) else { continue };

        let Some(attr) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { continue };
        let positions: Vec<[f32; 3]> = attr.as_float3().unwrap().to_vec();
        let mut new_positions = positions.clone();
        let mut new_normals = Vec::with_capacity(positions.len());

        let freq = water.wave_frequency;
        let amp = water.wave_amplitude;
        let spd = water.wave_speed;

        for (i, pos) in positions.iter().enumerate() {
            // Use world-space for consistent wave pattern
            let wx = pos[0] + transform.translation.x;
            let wz = pos[2] + transform.translation.z;

            // Multi-octave waves
            let w1 = (wx * freq + t * spd).sin() * amp;
            let w2 = (wz * freq * 0.7 + t * spd * 1.3).sin() * amp * 0.5;
            let w3 = ((wx + wz) * freq * 1.5 + t * spd * 0.8).sin() * amp * 0.25;

            new_positions[i][1] = w1 + w2 + w3;

            // Analytical normal from wave derivatives
            let dx = freq * (wx * freq + t * spd).cos() * amp
                + freq * 1.5 * ((wx + wz) * freq * 1.5 + t * spd * 0.8).cos() * amp * 0.25;
            let dz = freq * 0.7 * (wz * freq * 0.7 + t * spd * 1.3).cos() * amp * 0.5
                + freq * 1.5 * ((wx + wz) * freq * 1.5 + t * spd * 0.8).cos() * amp * 0.25;

            let n = Vec3::new(-dx, 1.0, -dz).normalize();
            new_normals.push([n.x, n.y, n.z]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
    }
}

/// Animate water material for shimmer/sparkle and foam opacity pulsing.
pub fn water_shimmer_system(
    time: Res<Time>,
    settings: Res<GraphicsSettings>,
    water_q: Query<&MeshMaterial3d<StandardMaterial>, With<WaterBody>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let t = time.elapsed_secs();

    for mat_handle in water_q.iter() {
        let Some(mat) = materials.get_mut(&mat_handle.0) else { continue };

        // Subtle emissive shimmer (simulates specular highlights / sun glints)
        let shimmer = ((t * 3.0).sin() * 0.5 + 0.5) * 0.12;
        mat.emissive = LinearRgba::new(shimmer * 0.2, shimmer * 0.4, shimmer * 0.7, 1.0);

        // Gentle alpha variation
        let alpha = 0.75 + (t * 1.8).sin() * 0.04;
        mat.base_color = Color::srgba(0.08, 0.28, 0.55, alpha);
    }
}

/// Pulse shore foam opacity for a lapping-wave effect.
pub fn foam_animation_system(
    time: Res<Time>,
    settings: Res<GraphicsSettings>,
    foam_q: Query<(&ShoreFoam, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let t = time.elapsed_secs();

    for (foam, mat_handle) in foam_q.iter() {
        let Some(mat) = materials.get_mut(&mat_handle.0) else { continue };

        // Each foam patch has a different phase for variety
        let pulse = ((t * 1.2 + foam.phase).sin() * 0.5 + 0.5) * 0.4 + 0.1;
        mat.base_color = Color::srgba(0.9, 0.92, 0.95, pulse);
    }
}
