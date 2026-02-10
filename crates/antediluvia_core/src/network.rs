//! Network protocol and message types.
//! 
//! Defines all messages sent between client and server.

use serde::{Deserialize, Serialize};
use bevy::prelude::Vec3;

/// A network message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    // Player actions
    PlayerMove { position: Vec3, rotation: f32 },
    PlayerAction { action: String, target: Option<u64> },
    PlayerChat { message: String },
    
    // Combat
    CombatAction { action_type: String, target_id: u64 },
    SkillChain { first_action: String, second_action: String },
    
    // NPC interaction
    NPCInteract { npc_id: u64, query: String },
    
    // Inventory
    InventoryUpdate { items: Vec<(String, u32)> },
    
    // Server state
    WorldStateUpdate { corruption: f32, flood_phase: String },
    PlayerStateUpdate { player_id: u64, health: f32, position: Vec3 },
    
    // Connection
    Ping,
    Pong,
}

/// A player's network state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerNetworkState {
    pub player_id: u64,
    pub position: Vec3,
    pub rotation: f32,
    pub health: f32,
    pub last_update: f32, // Timestamp
}

impl PlayerNetworkState {
    /// Create a new player network state.
    pub fn new(player_id: u64, position: Vec3) -> Self {
        Self {
            player_id,
            position,
            rotation: 0.0,
            health: 100.0,
            last_update: 0.0,
        }
    }

    /// Update the player's position.
    pub fn update_position(&mut self, position: Vec3, rotation: f32) {
        self.position = position;
        self.rotation = rotation;
    }

    /// Apply damage.
    pub fn take_damage(&mut self, damage: f32) {
        self.health = (self.health - damage).max(0.0);
    }

    /// Check if the player is alive.
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

/// Network configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub server_addr: String,
    pub server_port: u16,
    pub tick_rate: u32, // Updates per second
    pub max_players: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server_addr: "127.0.0.1".to_string(),
            server_port: 5001,
            tick_rate: 60,
            max_players: 1000,
        }
    }
}

/// Rollback netcode state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RollbackState {
    pub frame: u32,
    pub player_inputs: Vec<NetworkMessage>,
    pub confirmed_frame: u32,
}

impl RollbackState {
    /// Create a new rollback state.
    pub fn new() -> Self {
        Self {
            frame: 0,
            player_inputs: Vec::new(),
            confirmed_frame: 0,
        }
    }

    /// Add an input to the rollback buffer.
    pub fn add_input(&mut self, message: NetworkMessage) {
        self.player_inputs.push(message);
    }

    /// Confirm a frame (server has validated it).
    pub fn confirm_frame(&mut self, frame: u32) {
        self.confirmed_frame = frame;
        // Remove old inputs
        self.player_inputs.retain(|_| {
            self.frame - self.confirmed_frame < 10 // Keep last 10 frames
        });
    }

    /// Advance to the next frame.
    pub fn advance_frame(&mut self) {
        self.frame += 1;
    }
}

impl Default for RollbackState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_network_state() {
        let mut state = PlayerNetworkState::new(1, Vec3::ZERO);
        state.take_damage(25.0);
        assert_eq!(state.health, 75.0);
        assert!(state.is_alive());
    }

    #[test]
    fn test_rollback_state() {
        let mut rollback = RollbackState::new();
        rollback.add_input(NetworkMessage::Ping);
        assert_eq!(rollback.player_inputs.len(), 1);
        
        rollback.advance_frame();
        assert_eq!(rollback.frame, 1);
    }
}
