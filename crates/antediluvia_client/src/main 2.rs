mod map;
mod player;
mod npc;
mod inventory;
mod login;
mod gui;
mod combat;
mod physics;
mod mob_ai;
mod gathering;
mod terrain_mesh;
mod water;
pub mod graphics_settings;
pub mod rendering;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use antediluvia_core::world::PangeaGenerator;
use antediluvia_core::world::FloodStage;
use antediluvia_core::crafting::CraftingSystem;
use antediluvia_core::entity::Job;
use map::{map_input_system, map_render_system};
use player::{player_movement_system, player_look_system, cursor_grab_system, camera_follow_system, PlayerCamera, FollowCamera};
use npc::{spawn_noah, spawn_elder, spawn_merchant, npc_interaction_system, NPCInteraction};
use inventory::{inventory_input_system, Satchel, InventoryItem};
use login::{spawn_login_ui, despawn_login_ui, login_input_system, LoginStatus};
use gui::GuiPlugin;
use combat::{
    PlayerCombat, ChainNotification, update_cooldowns, combat_input_system, spawn_mobs,
    update_mob_health_display, update_damage_numbers, player_respawn_system,
};
use mob_ai::{mob_ai_system, mob_attack_system, mob_death_system, death_effect_system, MobBrain};
use gathering::{spawn_gathering_nodes, gathering_system, node_respawn_system};
use physics::{physics_system, collision_system, collision_response_system, CollisionEvent};
use graphics_settings::{GraphicsSettingsPlugin, GraphicsSettings, QualityTier};
use rendering::RenderingPlugin;

use bevy_renet::RenetClientPlugin;
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeClientTransport, ClientAuthentication};
use bevy_renet::RenetClient;
use bevy_renet::renet::ConnectionConfig;
use bevy::light::GlobalAmbientLight;
use std::net::UdpSocket;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

// ─── Resources ──────────────────────────────────────────

/// Holds the terrain generator and base offset for terrain height queries.
#[derive(Resource)]
pub struct TerrainData {
    pub generator: PangeaGenerator,
    pub base_offset: f32,
}

#[derive(Resource)]
pub struct CraftingRes(pub CraftingSystem);

#[derive(Resource)]
pub struct WorldState {
    pub corruption: f32,
    pub flood_stage: FloodStage,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            corruption: 15.0,
            flood_stage: FloodStage::Innocence,
        }
    }
}

#[derive(Resource, Default)]
pub struct Equipment {
    pub weapon: Option<String>,
    pub armor: Option<String>,
}

impl Equipment {
    pub fn weapon_damage_bonus(&self) -> f32 {
        match self.weapon.as_deref() {
            Some("Bronze Sword") => 10.0,
            Some("Iron Sword") => 20.0,
            _ => 0.0,
        }
    }

    pub fn armor_hp_bonus(&self) -> f32 {
        match self.armor.as_deref() {
            Some("Linen Tunic") => 25.0,
            _ => 0.0,
        }
    }
}

#[derive(Resource)]
pub struct DayNightCycle {
    pub time_of_day: f32,
    pub day_speed: f32,
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            time_of_day: 8.0,
            day_speed: 0.1,
        }
    }
}

// ─── App State ──────────────────────────────────────────

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    player_id: u64,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InWorld,
    Login,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .init_resource::<LoginStatus>()
        .init_resource::<NPCInteraction>()
        .init_resource::<WorldState>()
        .init_resource::<Equipment>()
        .init_resource::<DayNightCycle>()
        .init_resource::<ChainNotification>()
        .insert_resource(CraftingRes(CraftingSystem::new()))
        .add_message::<CollisionEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Antediluvia: The Tenth Generation".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(GraphicsSettingsPlugin)
        .add_plugins(RenderingPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)))
        // Login flow
        .add_systems(OnEnter(AppState::Login), spawn_login_ui)
        .add_systems(OnExit(AppState::Login), despawn_login_ui)
        .add_systems(Update, login_input_system.run_if(in_state(AppState::Login)))
        // World enter
        .add_systems(
            OnEnter(AppState::InWorld),
            (setup, connect_to_server, spawn_noah, spawn_elder, spawn_merchant, spawn_player_satchel, spawn_mobs, spawn_gathering_nodes),
        )
        // Update systems - split into groups (Bevy tuple limit = 12)
        .add_systems(
            Update,
            (
                player_movement_system,
                player_look_system,
                cursor_grab_system,
                camera_follow_system,
                combat_input_system,
                update_cooldowns,
                player_respawn_system,
                update_damage_numbers,
                mob_ai_system,
                mob_attack_system,
                mob_death_system,
                death_effect_system,
            )
            .run_if(in_state(AppState::InWorld)),
        )
        .add_systems(
            Update,
            (
                update_mob_health_display,
                npc_interaction_system,
                corruption_tick_system,
                sky_system,
                day_night_system,
                gathering_system,
                node_respawn_system,
                map_input_system,
                map_render_system,
                inventory_input_system,
                physics_system,
                collision_system,
            )
            .run_if(in_state(AppState::InWorld)),
        )
        .add_systems(
            Update,
            (
                collision_response_system,
                terrain_snap_system,
                water::water_animation_system,
                water::water_shimmer_system,
                water::foam_animation_system,
            )
            .run_if(in_state(AppState::InWorld)),
        )
        .run();
}

// ─── World Systems ──────────────────────────────────────

fn corruption_tick_system(mut world_state: ResMut<WorldState>, time: Res<Time>) {
    world_state.corruption += 0.02 * time.delta_secs();
    world_state.corruption = world_state.corruption.min(100.0);
    world_state.flood_stage = FloodStage::from_corruption(world_state.corruption);
}

fn sky_system(
    world_state: Res<WorldState>,
    cycle: Res<DayNightCycle>,
    mut clear_color: ResMut<ClearColor>,
) {
    let c = world_state.corruption / 100.0;
    let hour = cycle.time_of_day;

    // Day/night blend: 1 = day, 0 = night
    let blend = if hour >= 6.0 && hour < 7.0 {
        hour - 6.0
    } else if hour >= 7.0 && hour < 17.0 {
        1.0
    } else if hour >= 17.0 && hour < 18.0 {
        1.0 - (hour - 17.0)
    } else {
        0.0
    };

    // Base sky = lerp between night and day
    let r = 0.05 + blend * 0.48;
    let g = 0.05 + blend * 0.76;
    let b = 0.15 + blend * 0.77;

    // Apply corruption tint
    let r = (r + c * 0.3).min(0.85);
    let g = (g - c * 0.5).max(0.05);
    let b = (b - c * 0.6).max(0.05);

    clear_color.0 = Color::srgb(r, g, b);
}

fn day_night_system(
    mut cycle: ResMut<DayNightCycle>,
    mut sun_q: Query<(&mut DirectionalLight, &mut Transform)>,
    mut ambient: ResMut<GlobalAmbientLight>,
    time: Res<Time>,
) {
    cycle.time_of_day += cycle.day_speed * time.delta_secs();
    if cycle.time_of_day >= 24.0 {
        cycle.time_of_day -= 24.0;
    }

    let hour = cycle.time_of_day;
    let is_day = hour >= 6.0 && hour < 18.0;
    let sun_progress = ((hour - 6.0) / 12.0).clamp(0.0, 1.0);
    let sun_elevation = (sun_progress * std::f32::consts::PI).sin();

    for (mut light, mut transform) in sun_q.iter_mut() {
        if is_day {
            light.illuminance = 10000.0 + sun_elevation * 20000.0;
            transform.rotation = Quat::from_euler(
                EulerRot::XYZ,
                -sun_elevation.max(0.1) * std::f32::consts::FRAC_PI_2,
                std::f32::consts::FRAC_PI_6,
                0.0,
            );
        } else {
            light.illuminance = 2000.0;
            transform.rotation = Quat::from_euler(
                EulerRot::XYZ,
                -std::f32::consts::FRAC_PI_4,
                std::f32::consts::FRAC_PI_3,
                0.0,
            );
        }
    }

    ambient.brightness = if is_day { 0.3 + sun_elevation * 0.4 } else { 0.15 };
}

// ─── Networking ─────────────────────────────────────────

fn connect_to_server(mut commands: Commands) {
    println!("Initiating Dev Connection...");
    let player_id = 12345u64;
    let token = String::new();

    if let Ok((client, transport)) = setup_network_connection(player_id, &token) {
        commands.insert_resource(client);
        commands.insert_resource(transport);
        println!("Renet Client connected (online mode)");
    } else {
        println!("No server found. Starting in offline mode.");
        if let Ok((client, transport)) = setup_offline_connection(999999) {
            commands.insert_resource(client);
            commands.insert_resource(transport);
            println!("Running in offline mode");
        } else {
            println!("WARNING: Network init failed entirely. Running without networking.");
        }
    }
}

fn setup_network_connection(player_id: u64, token: &str) -> Result<(RenetClient, NetcodeClientTransport), String> {
    let connection_config = ConnectionConfig::default();
    let client = RenetClient::new(connection_config);

    let server_addr_str = std::env::var("CLIENT_SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("CLIENT_SERVER_PORT")
        .unwrap_or_else(|_| "5001".to_string());

    let server_addr = format!("{}:{}", server_addr_str, server_port)
        .parse()
        .map_err(|e| format!("Failed to parse server address: {}", e))?;

    let socket = UdpSocket::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get system time: {}", e))?;

    let authentication = ClientAuthentication::Unsecure {
        client_id: player_id,
        protocol_id: 7,
        server_addr,
        user_data: Some({
            let mut data = [0u8; 256];
            let bytes = token.as_bytes();
            if bytes.len() <= 256 {
                data[..bytes.len()].copy_from_slice(bytes);
            }
            data
        }),
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .map_err(|e| format!("Failed to create network transport: {}", e))?;

    Ok((client, transport))
}

fn setup_offline_connection(client_id: u64) -> Result<(RenetClient, NetcodeClientTransport), String> {
    let connection_config = ConnectionConfig::default();
    let client = RenetClient::new(connection_config);

    let server_addr = "127.0.0.1:5001"
        .parse()
        .map_err(|e| format!("Failed to parse fallback address: {}", e))?;

    let socket = UdpSocket::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to bind fallback socket: {}", e))?;

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| format!("Failed to get system time: {}", e))?;

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 7,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .map_err(|e| format!("Failed to create fallback transport: {}", e))?;

    Ok((client, transport))
}

// ─── Terrain Following ──────────────────────────────────

/// Keeps ground entities on the terrain surface each frame.
fn terrain_snap_system(
    terrain_data: Option<Res<TerrainData>>,
    mut player_q: Query<&mut Transform, With<PlayerCamera>>,
    mut mob_q: Query<&mut Transform, (With<MobBrain>, Without<PlayerCamera>)>,
    mut npc_q: Query<&mut Transform, (With<npc::NPCEntity>, Without<PlayerCamera>, Without<MobBrain>)>,
    mut node_q: Query<&mut Transform, (With<gathering::GatheringNode>, Without<PlayerCamera>, Without<MobBrain>, Without<npc::NPCEntity>)>,
) {
    let Some(data) = terrain_data else { return; };

    // Player - snap to terrain (minimum height constraint)
    for mut tf in player_q.iter_mut() {
        let ground = terrain_mesh::get_terrain_height(
            &data.generator, tf.translation.x, tf.translation.z, data.base_offset,
        );
        let min_y = ground + 3.5;
        if tf.translation.y < min_y {
            tf.translation.y = min_y;
        }
    }

    // Mobs - follow terrain
    for mut tf in mob_q.iter_mut() {
        let ground = terrain_mesh::get_terrain_height(
            &data.generator, tf.translation.x, tf.translation.z, data.base_offset,
        );
        tf.translation.y = ground + 5.0;
    }

    // NPCs - static on terrain
    for mut tf in npc_q.iter_mut() {
        let ground = terrain_mesh::get_terrain_height(
            &data.generator, tf.translation.x, tf.translation.z, data.base_offset,
        );
        tf.translation.y = ground + 5.0;
    }

    // Gathering nodes - static on terrain
    for mut tf in node_q.iter_mut() {
        let ground = terrain_mesh::get_terrain_height(
            &data.generator, tf.translation.x, tf.translation.z, data.base_offset,
        );
        tf.translation.y = ground + 2.0;
    }
}

// ─── World Setup ────────────────────────────────────────

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    gfx_settings: Res<GraphicsSettings>,
) {
    let player_combat = PlayerCombat::new(Job::Hunter);

    // ── Player body (visible third-person character) ──

    let tunic_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.25, 0.55),
        ..default()
    });
    let skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.56, 0.42),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(1.2, 2.5))),
        MeshMaterial3d(tunic_mat),
        Transform::from_xyz(0.0, 5.0, 100.0),
        PlayerCamera,
        player_combat,
        Name::new("Player"),
    )).with_children(|parent| {
        // Head
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.9))),
            MeshMaterial3d(skin_mat.clone()),
            Transform::from_xyz(0.0, 2.8, 0.0),
        ));
        // Arms
        for side in [-1.0f32, 1.0] {
            parent.spawn((
                Mesh3d(meshes.add(Capsule3d::new(0.3, 1.2))),
                MeshMaterial3d(skin_mat.clone()),
                Transform::from_xyz(side * 1.6, 0.3, 0.0),
            ));
        }
    });

    // ── 3D Camera (follows player in third-person) ──

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 13.0, 114.0)
            .looking_at(Vec3::new(0.0, 5.0, 100.0), Vec3::Y),
        FollowCamera::default(),
        DistanceFog {
            color: Color::srgba(0.55, 0.65, 0.75, 1.0),
            falloff: FogFalloff::Linear {
                start: 250.0,
                end: 600.0,
            },
            ..default()
        },
    ));

    // 2D overlay camera
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));

    let generator = PangeaGenerator::new();
    let base_offset = terrain_mesh::compute_base_offset(&generator, 0.0, 100.0);

    commands.insert_resource(TerrainData {
        generator: PangeaGenerator::new(),
        base_offset,
    });

    // ── Heightmap terrain ──

    let terrain = terrain_mesh::generate_terrain_mesh(&generator, 800.0, 128, base_offset);
    let terrain_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.0,
        perceptual_roughness: 0.95,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(terrain_mat),
        Transform::default(),
        terrain_mesh::TerrainChunk,
    ));

    // ── River (animated water in carved channel) ──

    water::spawn_river(&mut commands, &mut meshes, &mut materials, &gfx_settings);

    // ── Sun (directional light) ──

    commands.spawn((
        DirectionalLight {
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::PI / 3.0,
            std::f32::consts::PI / 6.0,
            0.0,
        )),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
        ..default()
    });

    // Sun disc (visible in sky)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(18.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.7),
            emissive: LinearRgba::new(2.0, 1.8, 1.0, 1.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(200.0, 350.0, -300.0),
    ));

    // ── Clouds ──

    let cloud_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.35),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for (x, y, z, sx, sz) in &[
        (80.0f32, 280.0f32, -100.0f32, 120.0f32, 50.0f32),
        (-120.0, 300.0, 50.0, 90.0, 55.0),
        (200.0, 260.0, -200.0, 140.0, 40.0),
        (-50.0, 290.0, 180.0, 100.0, 45.0),
        (150.0, 310.0, 100.0, 80.0, 35.0),
        (-200.0, 275.0, -150.0, 110.0, 48.0),
        (300.0, 295.0, 50.0, 95.0, 42.0),
    ] {
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(*sx, *sz))),
            MeshMaterial3d(cloud_mat.clone()),
            Transform::from_xyz(*x, *y, *z),
        ));
    }

    // ── Eden Pillar (stone monument, not giant pole) ──

    let pillar_y = terrain_mesh::get_terrain_height(&generator, 0.0, 0.0, base_offset);

    let pillar_stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.72, 0.65),
        metallic: 0.1,
        perceptual_roughness: 0.7,
        ..default()
    });

    // Base platform
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(14.0, 3.0, 14.0))),
        MeshMaterial3d(pillar_stone.clone()),
        Transform::from_xyz(0.0, pillar_y + 1.5, 0.0),
    ));

    // Steps
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(18.0, 1.0, 18.0))),
        MeshMaterial3d(pillar_stone.clone()),
        Transform::from_xyz(0.0, pillar_y + 0.5, 0.0),
    ));

    // Pillar shaft
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(2.5, 22.0))),
        MeshMaterial3d(pillar_stone),
        Transform::from_xyz(0.0, pillar_y + 14.0, 0.0),
    ));

    // Golden cap with glow
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(3.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.85, 0.2),
            emissive: LinearRgba::new(1.0, 0.8, 0.0, 1.0),
            metallic: 0.6,
            ..default()
        })),
        Transform::from_xyz(0.0, pillar_y + 27.0, 0.0),
    ));

    // Glow aura
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(6.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.9, 0.3, 0.15),
            alpha_mode: AlphaMode::Blend,
            emissive: LinearRgba::new(0.5, 0.4, 0.0, 1.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, pillar_y + 27.0, 0.0),
    ));

    // ── Trees (taller trunks, ellipsoid canopies) ──

    let tree_mesh = meshes.add(Cylinder::new(1.5, 18.0));
    let canopy_mesh = meshes.add(Sphere::new(7.0));
    let tree_trunk_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.22, 0.05),
        perceptual_roughness: 0.9,
        ..default()
    });
    let canopy_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.5, 0.1),
        ..default()
    });
    let dark_canopy_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.06, 0.38, 0.06),
        ..default()
    });

    for (i, (x, z)) in [(100, 100), (150, -150), (-200, 80), (-100, -120), (180, 50),
                     (70, -60), (-50, 180), (220, 120), (-180, -50), (30, -170),
                     (-130, 150), (90, -200), (-220, -100), (170, -80), (-70, 100),
                     (250, -50), (-160, 200), (60, 240), (-240, -160), (280, 150)]
                     .iter().enumerate() {
        let xf = *x as f32;
        let zf = *z as f32;
        let ground_y = terrain_mesh::get_terrain_height(&generator, xf, zf, base_offset);
        commands.spawn((
            Mesh3d(tree_mesh.clone()),
            MeshMaterial3d(tree_trunk_mat.clone()),
            Transform::from_xyz(xf, ground_y + 9.0, zf),
        ));
        let mat = if i % 3 == 0 { dark_canopy_mat.clone() } else { canopy_mat.clone() };
        commands.spawn((
            Mesh3d(canopy_mesh.clone()),
            MeshMaterial3d(mat),
            Transform::from_xyz(xf, ground_y + 22.0, zf)
                .with_scale(Vec3::new(1.3, 1.8, 1.3)),
        ));
    }

    // ── Bushes ──

    let bush_mesh = meshes.add(Sphere::new(2.5));
    let bush_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.48, 0.12),
        ..default()
    });

    for (x, z) in &[(95, 105), (155, -145), (-195, 85), (75, -55), (-45, 175),
                     (110, 90), (-110, -115), (185, 55), (-175, -45), (35, -165),
                     (45, 50), (-25, 60), (130, 30), (-80, -30)] {
        let xf = *x as f32;
        let zf = *z as f32;
        let ground_y = terrain_mesh::get_terrain_height(&generator, xf, zf, base_offset);
        commands.spawn((
            Mesh3d(bush_mesh.clone()),
            MeshMaterial3d(bush_mat.clone()),
            Transform::from_xyz(xf, ground_y + 1.2, zf)
                .with_scale(Vec3::new(1.4, 0.9, 1.4)),
        ));
    }

    // ── Rock formations ──

    let rock_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.48, 0.45),
        perceptual_roughness: 0.85,
        ..default()
    });

    for (x, z, scale) in &[
        (50i32, -200i32, 1.0f32), (-150, 120, 1.2), (200, -100, 0.8),
        (-80, -180, 1.1), (250, 30, 0.7), (-230, 60, 1.3),
        (40, 230, 0.9), (-160, -170, 1.0),
    ] {
        let xf = *x as f32;
        let zf = *z as f32;
        let ground_y = terrain_mesh::get_terrain_height(&generator, xf, zf, base_offset);
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(6.0))),
            MeshMaterial3d(rock_mat.clone()),
            Transform::from_xyz(xf, ground_y + 3.0, zf)
                .with_scale(Vec3::new(1.3 * scale, 0.7 * scale, 1.1 * scale)),
        ));
    }

    // ── Campfire near NPCs ──

    let campfire_y = terrain_mesh::get_terrain_height(&generator, 5.0, 15.0, base_offset);

    // Fire pit stones
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(1.5, 2.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.35, 0.3),
            ..default()
        })),
        Transform::from_xyz(5.0, campfire_y + 0.5, 15.0),
    ));

    // Fire glow
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.5, 0.1, 0.6),
            emissive: LinearRgba::new(2.0, 1.0, 0.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(5.0, campfire_y + 2.0, 15.0),
        Name::new("Campfire"),
    ));

    // Fire light
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.6, 0.2),
            intensity: 50000.0,
            range: 50.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(5.0, campfire_y + 3.0, 15.0),
    ));

    println!("=== Antediluvia: The Tenth Generation ===");
    println!("Havilah zone loaded. Welcome, Hunter.");
    println!("Controls:");
    println!("  WASD: Move | Shift: Sprint | Mouse: Look");
    println!("  1-4: Abilities | 3: Heal");
    println!("  E: Talk to NPC | I: Inventory | C: Crafting | M: Map");
    println!("  Tab: Equipment | F: Gather | F3: Debug panel");
    println!("Click to capture mouse. ESC to release.");

    // Controls overlay
    commands.spawn((
        Text::new(
            "WASD: Move | Shift: Sprint | 1-4: Abilities\nE: Talk | I: Inventory | C: Craft | M: Map | Tab: Gear\nF: Gather | F3: Debug | Click mouse to lock | ESC unlock",
        ),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn spawn_player_satchel(mut commands: Commands) {
    let mut satchel = Satchel::new(100.0);

    satchel.add_item(InventoryItem {
        name: "Gopher Wood".to_string(),
        quantity: 5,
        weight: 10.0,
    });

    satchel.add_item(InventoryItem {
        name: "Bread".to_string(),
        quantity: 10,
        weight: 0.5,
    });

    satchel.add_item(InventoryItem {
        name: "Waterskin".to_string(),
        quantity: 1,
        weight: 2.0,
    });

    // Starting crafting materials
    satchel.add_item(InventoryItem {
        name: "Bronze Ingot".to_string(),
        quantity: 3,
        weight: 4.0,
    });

    satchel.add_item(InventoryItem {
        name: "Leather Grip".to_string(),
        quantity: 2,
        weight: 1.0,
    });

    commands.spawn(satchel);
    println!("Player satchel initialized with starting items + crafting materials.");
}
