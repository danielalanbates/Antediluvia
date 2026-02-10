//! Player character data structures.

use serde::{Deserialize, Serialize};
use super::{Entity, EntityId, JobMastery};
use glam::Vec3;

/// A player character.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub entity: Entity,
    pub name: String,
    pub lineage: Lineage,
    pub corruption: f32, // 0.0 (Pure) to 100.0 (Fallen)
    pub job_mastery: JobMastery,
}

/// The two houses of humanity.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Lineage {
    /// The House of Seth. Faith, Discipline, Integrity.
    Seth,
    
    /// The House of Cain. Technology, Ambition, Corruption.
    Cain,
}

impl Player {
    /// Create a new player character.
    pub fn new(id: EntityId, name: String, position: Vec3) -> Self {
        Self {
            entity: Entity::new(id, position, 100.0),
            name,
            lineage: Lineage::Seth,
            corruption: 0.0,
            job_mastery: JobMastery::default(),
        }
    }

    /// Check if the player is corrupted (Cain-aligned).
    pub fn is_corrupted(&self) -> bool {
        self.corruption > 50.0
    }

    /// Increase the player's corruption.
    pub fn corrupt(&mut self, amount: f32) {
        self.corruption = (self.corruption + amount).min(100.0);
        if self.corruption > 50.0 {
            self.lineage = Lineage::Cain;
        }
    }

    /// Decrease the player's corruption (Redemption).
    pub fn redeem(&mut self, amount: f32) {
        self.corruption = (self.corruption - amount).max(0.0);
        if self.corruption < 50.0 {
            self.lineage = Lineage::Seth;
        }
    }
}
