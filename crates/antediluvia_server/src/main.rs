//! Antediluvia Server
//! Authoritative game loop stub. Provides entrypoint for networking and world state.

use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;
use antediluvia_core::NetworkConfig;
mod net;
mod game;
mod db;
mod auth;
use game::GameState;
use auth::AuthService;
use std::sync::Arc;
use db::{WorldRecord, PlayerRecord};
use bevy::prelude::Vec3;

#[tokio::main]
async fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting Antediluvia server (authoritative stub)...");

    // Start auth HTTP server (fire-and-forget)
    let auth_service = AuthService::new();
    tokio::spawn(auth::start_auth_server(auth_service.clone(), 8081));

    // Load config (placeholder)
    let net_config = NetworkConfig::default();
    info!("Listening on {}:{}", net_config.server_addr, net_config.server_port);

    // Start networking (placeholder)
    let mut net_server = net::NetServer::new();
    if let Err(err) = net_server.start_server(&net_config) {
        return Err(err);
    }


    // Connect to database (stub)
    let db_pool = match db::DbPool::connect().await {
        Ok(pool) => {
            let _ = pool.init_schema().await;
            Some(pool)
        },
        Err(e) => {
            info!("DB connect failed (continuing without DB): {}", e);
            None
        }
    };
    let db_pool = db_pool.map(Arc::new);

    // Initialize authoritative game state
    let mut state = GameState::new();

    // Load world state from DB if available
    if let Some(db) = db_pool.as_ref() {
        match db.load_world().await {
            Ok(Some(record)) => {
                info!("Loaded world state from DB: ID {}", record.id);
                state.world.corruption_level = record.corruption;

                // Parse flood phase
                state.flood.phase = match record.flood_phase.as_str() {
                    "PreFlood" => antediluvia_core::FloodPhase::PreFlood,
                    "FloodBegins" => antediluvia_core::FloodPhase::FloodBegins,
                    "Rising" => antediluvia_core::FloodPhase::Rising,
                    "Deluge" => antediluvia_core::FloodPhase::Deluge,
                    "DoorClosed" => antediluvia_core::FloodPhase::DoorClosed,
                    _ => {
                        info!("Unknown flood phase '{}', defaulting to PreFlood", record.flood_phase);
                        antediluvia_core::FloodPhase::PreFlood
                    }
                };
            }
            Ok(None) => {
                info!("No world state found in DB, starting fresh.");
            }
            Err(e) => {
                info!("Failed to load world state: {}", e);
            }
        }
    }

    info!("World initialized. Corruption: {:.1}%, Phase: {:?}", state.world.corruption_level, state.flood.phase);

    // Main loop placeholder
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(16));
    let mut save_accumulator: f32 = 0.0;
    loop {
        interval.tick().await;
        let dt = 0.016;

        // Update network transport
        let _ = net_server.update(std::time::Duration::from_millis(16));

        // Poll for connection events
        for event in net_server.poll_events() {
            match event {
                bevy_renet::renet::ServerEvent::ClientConnected { client_id } => {
                    let id = client_id;
                    info!("Client connected: {}", id);

                    // Validate Token
                    let mut is_valid = false;
                    if let Some(handles) = &net_server.handles {
                        if let Some(user_data) = handles.transport.user_data(client_id) {
                             let len = user_data.iter().position(|&c| c == 0).unwrap_or(256);
                             if let Ok(token) = std::str::from_utf8(&user_data[..len]) {
                                 if auth_service.validate(token) {
                                     info!("Token validated for client {}: {}", id, token);
                                     is_valid = true;
                                 } else {
                                     info!("Invalid token for client {}: {}", id, token);
                                 }
                             }
                        }
                    }

                    if !is_valid {
                         info!("Disconnecting client {} due to invalid token", id);
                         net_server.disconnect(id);
                         continue;
                    }

                    // Load player state from DB
                    if let Some(db) = db_pool.as_ref() {
                        match db.load_player(id).await {
                            Ok(Some(record)) => {
                                info!("Loaded player {} from DB", id);
                                let state = antediluvia_core::PlayerNetworkState {
                                    player_id: id,
                                    position: Vec3::new(record.position_x, record.position_y, record.position_z),
                                    rotation: 0.0,
                                    health: 100.0,
                                    last_update: 0.0,
                                };
                                net_server.player_states.insert(id, state);
                            }
                            Ok(None) => {
                                info!("New player {} connected (no DB record)", id);
                            }
                            Err(e) => {
                                info!("Failed to load player {}: {}", id, e);
                            }
                        }
                    }
                }
                bevy_renet::renet::ServerEvent::ClientDisconnected { client_id, reason } => {
                    let id = client_id;
                    info!("Client disconnected: {} ({:?})", id, reason);
                    // Save player state and remove from memory
                    if let Some(state) = net_server.player_states.remove(&id) {
                        if let Some(db) = db_pool.as_ref() {
                            let record = PlayerRecord {
                                id: state.player_id as i64,
                                name: "Player".to_string(),
                                lineage: "Seth".to_string(),
                                corruption: 0.0,
                                position_x: state.position.x,
                                position_y: state.position.y,
                                position_z: state.position.z,
                                inventory_json: "[]".to_string(),
                            };
                            if let Err(e) = db.save_player(&record).await {
                                info!("Failed to save player {}: {}", id, e);
                            } else {
                                info!("Saved player {} to DB", id);
                            }
                        }
                    }
                }
            }
        }

        // Tick game logic (process messages, events)
        state.tick(dt, &mut net_server);

        // Send packets after processing
        net_server.send_packets();

        save_accumulator += dt;

        // Periodic world save (every 10s) if DB available
        if save_accumulator >= 10.0 {
            if let Some(db) = db_pool.as_ref() {
                let _ = db.save_world(&WorldRecord {
                    id: 1,
                    corruption: state.world.corruption_level,
                    flood_phase: format!("{:?}", state.flood.phase),
                    server_time_days: 0,
                }).await;
            }
            save_accumulator = 0.0;
        }
    }
}
