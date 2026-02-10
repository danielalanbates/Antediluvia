//! Client-side terrain mesh generation from PangeaGenerator heightmap data.
//!
//! Generates a heightmap mesh covering the playable area. The starting zone
//! is mapped to the Havilah region (distance 500-2000 from world origin)
//! of the PangeaGenerator, giving gentle rolling hills (10-30m variation).

use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use antediluvia_core::world::PangeaGenerator;

/// Offset applied to game coordinates to place starting area within Havilah.
/// Havilah spans distances 500-2000 from world origin in PangeaGenerator.
const WORLD_OFFSET_X: f64 = 1000.0;
const WORLD_OFFSET_Z: f64 = 0.0;

/// Component to tag terrain mesh entities.
#[derive(Component)]
pub struct TerrainChunk;

/// Get terrain height at a local game coordinate.
pub fn get_terrain_height(generator: &PangeaGenerator, local_x: f32, local_z: f32, base_offset: f32) -> f32 {
    let world_x = local_x as f64 + WORLD_OFFSET_X;
    let world_z = local_z as f64 + WORLD_OFFSET_Z;
    generator.get_height(world_x, world_z) - base_offset
}

/// Compute height offset at the player spawn point so terrain is near y=0 there.
pub fn compute_base_offset(generator: &PangeaGenerator, spawn_x: f32, spawn_z: f32) -> f32 {
    let world_x = spawn_x as f64 + WORLD_OFFSET_X;
    let world_z = spawn_z as f64 + WORLD_OFFSET_Z;
    generator.get_height(world_x, world_z)
}

/// Generate a terrain mesh from the PangeaGenerator.
///
/// - `size`: Total terrain extent in game units (e.g. 800.0 for 800x800)
/// - `resolution`: Vertices per side (e.g. 128 for 128x128 grid)
/// - `base_offset`: Height offset so player spawn is near y=0
pub fn generate_terrain_mesh(
    generator: &PangeaGenerator,
    size: f32,
    resolution: u32,
    base_offset: f32,
) -> Mesh {
    let vertex_count = (resolution * resolution) as usize;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertex_count);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(vertex_count);

    let step = size / (resolution - 1) as f32;
    let half_size = size / 2.0;

    // --- Pass 1: compute raw heights ---
    let mut heights: Vec<f32> = Vec::with_capacity(vertex_count);

    for z_idx in 0..resolution {
        for x_idx in 0..resolution {
            let local_x = -half_size + x_idx as f32 * step;
            let local_z = -half_size + z_idx as f32 * step;

            let world_x = local_x as f64 + WORLD_OFFSET_X;
            let world_z = local_z as f64 + WORLD_OFFSET_Z;

            let height = generator.get_height(world_x, world_z) - base_offset;
            heights.push(height);
        }
    }

    // --- Pass 1b: carve river channel ---
    let river_cx: f32 = -60.0;
    let river_cz: f32 = 0.0;
    let river_angle: f32 = std::f32::consts::PI / 5.0;
    let river_half_w: f32 = 20.0;
    let river_half_l: f32 = 360.0;
    let cos_a = river_angle.cos();
    let sin_a = river_angle.sin();

    for z_idx in 0..resolution {
        for x_idx in 0..resolution {
            let idx = (z_idx * resolution + x_idx) as usize;
            let local_x = -half_size + x_idx as f32 * step;
            let local_z = -half_size + z_idx as f32 * step;

            // Transform to river-local coordinates (rotate back by -angle)
            let dx = local_x - river_cx;
            let dz = local_z - river_cz;
            let rx = dx * cos_a + dz * sin_a;
            let rz = -dx * sin_a + dz * cos_a;

            if rx.abs() < river_half_w && rz.abs() < river_half_l {
                // Smooth river bed profile: deeper in center, slopes at edges
                let edge_factor = 1.0 - (rx.abs() / river_half_w).powf(2.0);
                let depth = edge_factor * 3.5;
                let river_bed = -1.5 - depth;
                heights[idx] = heights[idx].min(river_bed);
            }
        }
    }

    // --- Pass 2: build vertex data with positions, normals, colors ---
    for z_idx in 0..resolution {
        for x_idx in 0..resolution {
            let idx = (z_idx * resolution + x_idx) as usize;
            let local_x = -half_size + x_idx as f32 * step;
            let local_z = -half_size + z_idx as f32 * step;
            let h = heights[idx];

            positions.push([local_x, h, local_z]);
            uvs.push([
                x_idx as f32 / (resolution - 1) as f32,
                z_idx as f32 / (resolution - 1) as f32,
            ]);

            // Central-difference normals
            let h_left = if x_idx > 0 { heights[idx - 1] } else { h };
            let h_right = if x_idx < resolution - 1 { heights[idx + 1] } else { h };
            let h_down = if z_idx > 0 { heights[idx - resolution as usize] } else { h };
            let h_up = if z_idx < resolution - 1 { heights[idx + resolution as usize] } else { h };

            let normal = Vec3::new(h_left - h_right, 2.0 * step, h_down - h_up).normalize();
            normals.push([normal.x, normal.y, normal.z]);

            // Vertex color based on height and slope
            let slope = 1.0 - normal.y;
            let color = if h < -3.0 {
                // River bed / deep low area - dark mud
                [0.35, 0.28, 0.18, 1.0]
            } else if h < -1.0 {
                // Low near water - sandy/muddy
                [0.55, 0.45, 0.28, 1.0]
            } else if slope > 0.5 {
                // Very steep - exposed rock
                [0.5, 0.48, 0.44, 1.0]
            } else if slope > 0.3 {
                // Moderate slope - rocky grass
                [0.32, 0.46, 0.28, 1.0]
            } else if h > 8.0 {
                // Higher elevation - dark lush forest
                [0.08, 0.38, 0.06, 1.0]
            } else if h > 3.0 {
                // Mid elevation - grass
                [0.18, 0.55, 0.14, 1.0]
            } else {
                // Low flat areas - light grass
                [0.24, 0.62, 0.2, 1.0]
            };
            colors.push(color);
        }
    }

    // --- Pass 3: triangle indices ---
    let quad_count = ((resolution - 1) * (resolution - 1)) as usize;
    let mut indices: Vec<u32> = Vec::with_capacity(quad_count * 6);

    for z_idx in 0..(resolution - 1) {
        for x_idx in 0..(resolution - 1) {
            let tl = z_idx * resolution + x_idx;
            let tr = tl + 1;
            let bl = (z_idx + 1) * resolution + x_idx;
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
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
