//! Entity definitions and components.

pub mod player;
pub mod npc;
pub mod job;

pub use player::*;
pub use npc::*;
pub use job::*;

use serde::{Deserialize, Serialize};
use glam::Vec3;

/// A unique identifier for any entity in the world.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

/// Base entity data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub position: Vec3,
    pub rotation: f32,
    pub health: f32,
    pub max_health: f32,
}

impl Entity {
    /// Create a new entity at a position.
    pub fn new(id: EntityId, position: Vec3, max_health: f32) -> Self {
        Self {
            id,
            position,
            rotation: 0.0,
            health: max_health,
            max_health,
        }
    }

    /// Check if the entity is alive.
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    /// Apply damage to the entity.
    pub fn take_damage(&mut self, damage: f32) {
        self.health = (self.health - damage).max(0.0);
    }

    /// Heal the entity.
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }
}
