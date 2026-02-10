//! NPC spawning, management, and interaction.

use bevy::prelude::*;
use antediluvia_core::entity::{NPC, NPCType, EntityId};
use crate::player::PlayerCamera;

/// Component for NPCs in the world.
#[derive(Component)]
pub struct NPCEntity {
    pub npc: NPC,
}

/// Resource tracking active NPC dialogue.
#[derive(Resource, Default)]
pub struct NPCInteraction {
    pub npc_name: String,
    pub dialogue_lines: Vec<String>,
    pub current_line: usize,
    pub is_open: bool,
}

/// Spawn Noah near the Eden Pillar.
pub fn spawn_noah(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noah = NPC::new(
        EntityId(1),
        "Noah".to_string(),
        NPCType::Noah,
        Vec3::new(30.0, 5.0, 40.0),
    );

    let noah_skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.56, 0.42),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(1.8, 3.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.85, 0.9),
            ..default()
        })),
        Transform::from_translation(noah.entity.position),
        NPCEntity { npc: noah },
        Name::new("Noah"),
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(1.1))),
            MeshMaterial3d(noah_skin),
            Transform::from_xyz(0.0, 3.8, 0.0),
        ));
    });

    println!("Spawned NPC: Noah near Eden Pillar");
}

/// Spawn Methuselah the Elder.
pub fn spawn_elder(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let elder = NPC::new(
        EntityId(2),
        "Methuselah".to_string(),
        NPCType::Elder,
        Vec3::new(-40.0, 5.0, 30.0),
    );

    let elder_skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.68, 0.52, 0.38),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(1.5, 3.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.4, 0.2),
            ..default()
        })),
        Transform::from_translation(elder.entity.position),
        NPCEntity { npc: elder },
        Name::new("Methuselah"),
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(1.0))),
            MeshMaterial3d(elder_skin),
            Transform::from_xyz(0.0, 3.3, 0.0),
        ));
    });

    println!("Spawned NPC: Methuselah the Elder");
}

/// Spawn Jubal the Merchant.
pub fn spawn_merchant(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let merchant = NPC::new(
        EntityId(3),
        "Jubal".to_string(),
        NPCType::Merchant,
        Vec3::new(10.0, 5.0, -30.0),
    );

    let merchant_skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.56, 0.42),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(1.6, 3.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.5, 0.1),
            ..default()
        })),
        Transform::from_translation(merchant.entity.position),
        NPCEntity { npc: merchant },
        Name::new("Jubal"),
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(1.0))),
            MeshMaterial3d(merchant_skin),
            Transform::from_xyz(0.0, 3.4, 0.0),
        ));
    });

    println!("Spawned NPC: Jubal the Merchant");
}

/// System to handle E key NPC interaction.
pub fn npc_interaction_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut interaction: ResMut<NPCInteraction>,
    player_q: Query<&Transform, With<PlayerCamera>>,
    npc_q: Query<(&NPCEntity, &Transform)>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if interaction.is_open {
            interaction.is_open = false;
            return;
        }

        let player_pos = match player_q.single() {
            Ok(t) => t.translation,
            Err(_) => return,
        };

        let mut nearest: Option<(&NPCEntity, f32)> = None;
        for (npc_entity, npc_transform) in npc_q.iter() {
            let dist = player_pos.distance(npc_transform.translation);
            if dist < 30.0 {
                if nearest.is_none() || dist < nearest.unwrap().1 {
                    nearest = Some((npc_entity, dist));
                }
            }
        }

        if let Some((npc_comp, _)) = nearest {
            interaction.npc_name = npc_comp.npc.name.clone();
            interaction.dialogue_lines = get_npc_dialogue(&npc_comp.npc);
            interaction.current_line = 0;
            interaction.is_open = true;
            println!("Speaking with {}...", npc_comp.npc.name);
        }
    }

    // Space advances dialogue
    if interaction.is_open && keys.just_pressed(KeyCode::Space) {
        interaction.current_line += 1;
        if interaction.current_line >= interaction.dialogue_lines.len() {
            interaction.is_open = false;
        }
    }
}

fn get_npc_dialogue(npc: &NPC) -> Vec<String> {
    match npc.npc_type {
        NPCType::Noah => vec![
            format!("{}: The Lord has shown me what is to come.", npc.name),
            format!("{}: I am building an ark of gopher wood. Three hundred cubits long.", npc.name),
            format!("{}: The world grows more corrupt each day. The Watchers taught mankind forbidden arts.", npc.name),
            format!("{}: Repent, for the fountains of the deep shall break forth.", npc.name),
            format!("{}: Will you help me gather gopher wood for the Ark?", npc.name),
        ],
        NPCType::Elder => vec![
            format!("{}: I have walked this earth for many long years...", npc.name),
            format!("{}: I remember when the Watchers descended upon Mount Hermon.", npc.name),
            format!("{}: Their offspring, the Nephilim, are giants of renown and wickedness.", npc.name),
            format!("{}: The House of Cain has embraced forbidden knowledge. Iron, sorcery...", npc.name),
            format!("{}: But the House of Seth still calls upon the name of the Lord.", npc.name),
            format!("{}: Seek Noah. He is building something... extraordinary.", npc.name),
        ],
        NPCType::Merchant => vec![
            format!("{}: Welcome, traveler! I have wares if you have coin.", npc.name),
            format!("{}: Bronze tools, linen garments, herbs for healing...", npc.name),
            format!("{}: Some whisper of iron weapons from Tubal-Cain's forges.", npc.name),
            format!("{}: But such things carry a cost beyond gold.", npc.name),
        ],
        NPCType::Trainer => vec![
            format!("{}: You wish to learn? Strength comes through perseverance.", npc.name),
        ],
        NPCType::Villager => vec![
            format!("{}: Times are dark. The Nephilim grow bolder each day.", npc.name),
        ],
        NPCType::Creature => vec![
            "*growls*".to_string(),
        ],
    }
}
