//! Foliage system: procedural grass patches with wind animation.
//!
//! Each grass patch is a single mesh containing many triangular blades,
//! reducing entity count while covering the terrain with vegetation.
//!
//! Quality tier scaling:
//! - Low:   No grass (performance priority)
//! - Medium: Sparse patches (radius 80, spacing 12)
//! - High:  Full coverage (radius 150, spacing 8)
//! - Ultra: Dense coverage (radius 200, spacing 6)

use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use crate::graphics_settings::{GraphicsSettings, QualityTier};
use crate::terrain_mesh;
use crate::TerrainData;

/// Tags a grass patch entity for wind animation.
#[derive(Component)]
pub struct GrassPatch {
    pub wind_phase: f32,
}

/// Generate a mesh containing multiple grass blades arranged in a cluster.
///
/// Each blade is a thin triangle (3 vertices). Blades point upward with slight
/// random tilt. Vertex colors go from dark green at base to yellow-green at tip.
fn generate_grass_patch_mesh(blade_count: u32, patch_radius: f32, blade_height: f32) -> Mesh {
    let verts_per_blade = 4; // quad per blade (2 triangles)
    let vertex_count = (blade_count * verts_per_blade) as usize;

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let mut colors = Vec::with_capacity(vertex_count);
    let mut indices = Vec::with_capacity((blade_count * 6) as usize);

    // Pseudo-random using simple hash
    let mut seed: u32 = 42;
    let next_rand = |s: &mut u32| -> f32 {
        *s = s.wrapping_mul(1103515245).wrapping_add(12345);
        ((*s >> 16) & 0x7FFF) as f32 / 32767.0
    };

    for i in 0..blade_count {
        // Random position within the patch circle
        let angle = next_rand(&mut seed) * std::f32::consts::TAU;
        let dist = next_rand(&mut seed).sqrt() * patch_radius;
        let bx = angle.cos() * dist;
        let bz = angle.sin() * dist;

        // Random blade properties
        let tilt_angle = (next_rand(&mut seed) - 0.5) * 0.4;
        let height = blade_height * (0.6 + next_rand(&mut seed) * 0.4);
        let width = 0.15 + next_rand(&mut seed) * 0.15;

        // Blade facing direction (random rotation around Y)
        let face_angle = next_rand(&mut seed) * std::f32::consts::TAU;
        let face_cos = face_angle.cos();
        let face_sin = face_angle.sin();

        let half_w = width / 2.0;

        // Base left
        let bl_x = bx - half_w * face_cos;
        let bl_z = bz - half_w * face_sin;

        // Base right
        let br_x = bx + half_w * face_cos;
        let br_z = bz + half_w * face_sin;

        // Top (narrower, offset by tilt)
        let tilt_offset = tilt_angle * height;
        let top_half_w = width * 0.15;
        let tl_x = bx - top_half_w * face_cos + tilt_offset * face_sin;
        let tl_z = bz - top_half_w * face_sin - tilt_offset * face_cos;
        let tr_x = bx + top_half_w * face_cos + tilt_offset * face_sin;
        let tr_z = bz + top_half_w * face_sin - tilt_offset * face_cos;

        let base_idx = (i * verts_per_blade) as u32;

        // Base-left vertex
        positions.push([bl_x, 0.0, bl_z]);
        normals.push([0.0, 0.5, 0.5]);
        uvs.push([0.0, 0.0]);
        colors.push([0.08, 0.28, 0.04, 1.0]); // Dark green base

        // Base-right vertex
        positions.push([br_x, 0.0, br_z]);
        normals.push([0.0, 0.5, 0.5]);
        uvs.push([1.0, 0.0]);
        colors.push([0.08, 0.28, 0.04, 1.0]);

        // Top-right vertex
        positions.push([tr_x, height, tr_z]);
        normals.push([0.0, 0.7, 0.3]);
        uvs.push([1.0, 1.0]);
        // Tip color varies: yellow-green to light green
        let tip_g = 0.45 + next_rand(&mut seed) * 0.2;
        let tip_r = 0.2 + next_rand(&mut seed) * 0.15;
        colors.push([tip_r, tip_g, 0.05, 1.0]);

        // Top-left vertex
        positions.push([tl_x, height, tl_z]);
        normals.push([0.0, 0.7, 0.3]);
        uvs.push([0.0, 1.0]);
        colors.push([tip_r, tip_g, 0.05, 1.0]);

        // Two triangles per blade
        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);
        indices.push(base_idx);
        indices.push(base_idx + 2);
        indices.push(base_idx + 3);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Spawn grass patches across the terrain surface.
pub fn spawn_grass(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    settings: &GraphicsSettings,
    terrain_data: &TerrainData,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let (radius, spacing, blades_per_patch, blade_height) = match settings.quality_tier {
        QualityTier::Low => return,
        QualityTier::Medium => (80.0f32, 14.0f32, 16u32, 1.5f32),
        QualityTier::High => (150.0, 10.0, 24, 1.8),
        QualityTier::Ultra => (200.0, 7.0, 32, 2.0),
    };

    let grass_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE, // Using vertex colors
        double_sided: true,
        cull_mode: None,
        alpha_mode: AlphaMode::Opaque,
        perceptual_roughness: 0.9,
        ..default()
    });

    // Pre-generate a few patch mesh variants for visual variety
    let patch_meshes: Vec<Handle<Mesh>> = (0..4)
        .map(|_| meshes.add(generate_grass_patch_mesh(blades_per_patch, 4.0, blade_height)))
        .collect();

    let mut patch_count = 0u32;
    let half_radius = radius;
    let step = spacing;

    let mut x = -half_radius;
    while x <= half_radius {
        let mut z = -half_radius;
        while z <= half_radius {
            // Skip patches too far from center (circular distribution)
            let dist = (x * x + z * z).sqrt();
            if dist > radius {
                z += step;
                continue;
            }

            // Skip patches in the river channel area
            let river_cx = -60.0f32;
            let river_angle: f32 = std::f32::consts::PI / 5.0;
            let dx = x - river_cx;
            let dz = z;
            let rx = dx * river_angle.cos() + dz * river_angle.sin();
            let rz = -dx * river_angle.sin() + dz * river_angle.cos();
            if rx.abs() < 25.0 && rz.abs() < 370.0 {
                z += step;
                continue;
            }

            let ground_y = terrain_mesh::get_terrain_height(
                &terrain_data.generator, x, z, terrain_data.base_offset,
            );

            // Skip underwater areas and very steep terrain
            if ground_y < -1.0 {
                z += step;
                continue;
            }

            let mesh_idx = (patch_count as usize) % patch_meshes.len();

            commands.spawn((
                Mesh3d(patch_meshes[mesh_idx].clone()),
                MeshMaterial3d(grass_mat.clone()),
                Transform::from_xyz(x, ground_y, z),
                GrassPatch {
                    wind_phase: x * 0.1 + z * 0.07,
                },
            ));

            patch_count += 1;
            z += step;
        }
        x += step;
    }

    println!("Foliage: spawned {} grass patches ({} quality)", patch_count, settings.quality_tier);
}

/// Animate grass patches with wind sway.
pub fn wind_animation_system(
    time: Res<Time>,
    settings: Res<GraphicsSettings>,
    mut grass_q: Query<(&GrassPatch, &mut Transform)>,
) {
    if settings.quality_tier == QualityTier::Low {
        return;
    }

    let t = time.elapsed_secs();

    // Wind direction slowly shifts over time
    let wind_dir = (t * 0.05).sin() * 0.5;

    for (patch, mut transform) in grass_q.iter_mut() {
        // Multi-frequency sway for natural look
        let sway1 = (t * 1.2 + patch.wind_phase).sin() * 0.04;
        let sway2 = (t * 2.7 + patch.wind_phase * 1.5).sin() * 0.015;
        let gust = ((t * 0.3 + patch.wind_phase * 0.2).sin().max(0.0)).powf(3.0) * 0.06;

        let total_sway = sway1 + sway2 + gust;

        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            total_sway,
            0.0,
            total_sway * 0.5 + wind_dir * 0.02,
        );
    }
}
