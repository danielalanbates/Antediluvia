//! Gathering nodes for resource collection.

use bevy::prelude::*;
use crate::player::PlayerCamera;
use crate::inventory::{Satchel, InventoryItem};

#[derive(Component)]
pub struct GatheringNode {
    pub resource_name: String,
    pub quantity_per_gather: u32,
    pub weight_per_unit: f32,
    pub remaining: u32,
    pub max_remaining: u32,
    pub respawn_timer: f32,
}

pub fn spawn_gathering_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Wood piles near trees
    let wood_mesh = meshes.add(Cylinder::new(2.0, 4.0));
    let wood_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.3, 0.1),
        ..default()
    });

    for (x, z) in &[(105, 95), (-195, 75), (175, 55), (-45, 185), (25, -165)] {
        commands.spawn((
            Mesh3d(wood_mesh.clone()),
            MeshMaterial3d(wood_mat.clone()),
            Transform::from_xyz(*x as f32, 2.0, *z as f32),
            GatheringNode {
                resource_name: "Gopher Wood".to_string(),
                quantity_per_gather: 2,
                weight_per_unit: 10.0,
                remaining: 5,
                max_remaining: 5,
                respawn_timer: 60.0,
            },
            Name::new("Wood Pile"),
        ));
    }

    // Ore veins near rocks
    let ore_mesh = meshes.add(Sphere::new(3.0));
    let bronze_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.4, 0.2),
        metallic: 0.6,
        ..default()
    });
    let iron_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.35, 0.4),
        metallic: 0.8,
        ..default()
    });

    for (x, z) in &[(55, -195), (-145, 125)] {
        commands.spawn((
            Mesh3d(ore_mesh.clone()),
            MeshMaterial3d(bronze_mat.clone()),
            Transform::from_xyz(*x as f32, 2.0, *z as f32),
            GatheringNode {
                resource_name: "Bronze Ingot".to_string(),
                quantity_per_gather: 1,
                weight_per_unit: 4.0,
                remaining: 3,
                max_remaining: 3,
                respawn_timer: 90.0,
            },
            Name::new("Bronze Ore"),
        ));
    }

    for (x, z) in &[(205, -95), (-75, -175)] {
        commands.spawn((
            Mesh3d(ore_mesh.clone()),
            MeshMaterial3d(iron_mat.clone()),
            Transform::from_xyz(*x as f32, 2.0, *z as f32),
            GatheringNode {
                resource_name: "Iron Ingot".to_string(),
                quantity_per_gather: 1,
                weight_per_unit: 5.0,
                remaining: 3,
                max_remaining: 3,
                respawn_timer: 90.0,
            },
            Name::new("Iron Ore"),
        ));
    }

    // Herb patches
    let herb_mesh = meshes.add(Sphere::new(1.5));
    let herb_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.2),
        ..default()
    });

    for (x, z) in &[(40, 70), (-70, 50), (120, -40), (-30, -80)] {
        commands.spawn((
            Mesh3d(herb_mesh.clone()),
            MeshMaterial3d(herb_mat.clone()),
            Transform::from_xyz(*x as f32, 1.0, *z as f32),
            GatheringNode {
                resource_name: "Healing Herb".to_string(),
                quantity_per_gather: 3,
                weight_per_unit: 0.3,
                remaining: 6,
                max_remaining: 6,
                respawn_timer: 45.0,
            },
            Name::new("Herb Patch"),
        ));
    }

    // Linen plants
    let linen_mesh = meshes.add(Cylinder::new(1.0, 5.0));
    let linen_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.75, 0.5),
        ..default()
    });

    for (x, z) in &[(80, 30), (-90, -40), (20, 120), (-20, -110)] {
        commands.spawn((
            Mesh3d(linen_mesh.clone()),
            MeshMaterial3d(linen_mat.clone()),
            Transform::from_xyz(*x as f32, 2.5, *z as f32),
            GatheringNode {
                resource_name: "Linen Cloth".to_string(),
                quantity_per_gather: 2,
                weight_per_unit: 0.5,
                remaining: 4,
                max_remaining: 4,
                respawn_timer: 50.0,
            },
            Name::new("Linen Plant"),
        ));
    }

    // Thread (spider silk)
    for (x, z) in &[(60, 90), (-110, 70)] {
        commands.spawn((
            Mesh3d(herb_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.7),
                ..default()
            })),
            Transform::from_xyz(*x as f32, 1.0, *z as f32),
            GatheringNode {
                resource_name: "Thread".to_string(),
                quantity_per_gather: 2,
                weight_per_unit: 0.2,
                remaining: 4,
                max_remaining: 4,
                respawn_timer: 50.0,
            },
            Name::new("Spider Silk"),
        ));
    }

    println!("Spawned gathering nodes. Press F near nodes to gather resources.");
}

pub fn gathering_system(
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<PlayerCamera>>,
    mut node_q: Query<(Entity, &mut GatheringNode, &Transform)>,
    mut satchel_q: Query<&mut Satchel>,
) {
    if !keys.just_pressed(KeyCode::KeyF) { return; }

    let player_pos = match player_q.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let mut nearest: Option<(Entity, f32)> = None;
    for (entity, node, transform) in node_q.iter() {
        if node.remaining == 0 { continue; }
        let dist = player_pos.distance(transform.translation);
        if dist < 20.0 {
            if nearest.is_none() || dist < nearest.unwrap().1 {
                nearest = Some((entity, dist));
            }
        }
    }

    if let Some((entity, _)) = nearest {
        if let Ok((_, mut node, _)) = node_q.get_mut(entity) {
            let gather_qty = node.quantity_per_gather.min(node.remaining);
            node.remaining -= gather_qty;

            if let Ok(mut satchel) = satchel_q.single_mut() {
                if satchel.add_item(InventoryItem {
                    name: node.resource_name.clone(),
                    quantity: gather_qty,
                    weight: node.weight_per_unit * gather_qty as f32,
                }) {
                    println!("Gathered {} x{}", node.resource_name, gather_qty);
                } else {
                    node.remaining += gather_qty;
                    println!("Satchel too heavy! Cannot gather.");
                }
            }

            if node.remaining == 0 {
                println!("{} depleted. It will regenerate.", node.resource_name);
            }
        }
    }
}

pub fn node_respawn_system(
    mut node_q: Query<&mut GatheringNode>,
    time: Res<Time>,
) {
    for mut node in node_q.iter_mut() {
        if node.remaining == 0 {
            node.respawn_timer -= time.delta_secs();
            if node.respawn_timer <= 0.0 {
                node.remaining = node.max_remaining;
                node.respawn_timer = 60.0;
                println!("{} node has regenerated!", node.resource_name);
            }
        }
    }
}
