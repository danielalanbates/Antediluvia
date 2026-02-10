//! Authoritative game state and tick loop helpers.

use antediluvia_core::{WorldState, EventManager, FloodEvent, FloodPhase, NetworkMessage};
use tracing::info;
use crate::net::NetServer;

/// Authoritative game state container.
pub struct GameState {
    pub world: WorldState,
    pub events: EventManager,
    pub flood: FloodEvent,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = WorldState::new();
        world.update_weather();

        Self {
            world,
            events: EventManager::with_defaults(),
            flood: FloodEvent::new(),
        }
    }

    /// Advance the world by `delta_seconds`.
    pub fn tick(&mut self, delta_seconds: f32, net: &mut NetServer) {
        // Process incoming network messages
        let messages = net.receive_messages();
        for (client_id, msg) in messages {
            match msg {
                NetworkMessage::PlayerMove { position, rotation } => {
                    // Update local player state cache
                    if let Some(state) = net.player_states.get_mut(&client_id) {
                        state.update_position(position, rotation);
                    } else {
                        // New player? Register them
                        net.player_states.insert(client_id, antediluvia_core::PlayerNetworkState::new(client_id, position));
                    }
                    
                    // Broadcast movement to others
                    let _ = net.broadcast(&NetworkMessage::PlayerMove { position, rotation });
                }
                NetworkMessage::PlayerChat { message } => {
                    info!("[Chat] {}: {}", client_id, message);
                    let _ = net.broadcast(&NetworkMessage::PlayerChat { message: format!("{}: {}", client_id, message) });
                }
                _ => {}
            }
        }

        // Update weather based on corruption
        self.world.update_weather();

        // Update events and handle triggers
        let triggered = self.events.update(delta_seconds);
        for evt in triggered {
            info!("Event triggered: {:?}", evt);
            // Broadcast event start (placeholder)
            // let _ = net.broadcast(&NetworkMessage::WorldEvent { ... });
        }

        // Update flood state if corruption is maxed
        if self.world.corruption_level >= 99.9 && self.flood.phase == FloodPhase::PreFlood {
            self.flood.begin_flood();
            info!("Flood initiated (corruption 100%)");
        }
        if self.flood.phase != FloodPhase::PreFlood {
            self.flood.update(delta_seconds);
        }
    }
}
