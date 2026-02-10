//! NPC (Non-Player Character) definitions.

use serde::{Deserialize, Serialize};
use super::{Entity, EntityId};
use glam::Vec3;

/// An NPC in the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NPC {
    pub entity: Entity,
    pub name: String,
    pub npc_type: NPCType,
    pub dialogue_state: String,
}

/// The type of NPC.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NPCType {
    /// Noah, the Ark builder. Non-interactive.
    Noah,
    
    /// An elder who shares lore.
    Elder,
    
    /// A class trainer.
    Trainer,
    
    /// A merchant.
    Merchant,
    
    /// A generic villager.
    Villager,
    
    /// A hostile creature.
    Creature,
}

impl NPC {
    /// Create a new NPC.
    pub fn new(id: EntityId, name: String, npc_type: NPCType, position: Vec3) -> Self {
        let max_health = match npc_type {
            NPCType::Noah => 500.0,
            NPCType::Elder => 100.0,
            NPCType::Trainer => 150.0,
            NPCType::Merchant => 80.0,
            NPCType::Villager => 50.0,
            NPCType::Creature => 200.0,
        };

        Self {
            entity: Entity::new(id, position, max_health),
            name,
            npc_type,
            dialogue_state: String::new(),
        }
    }

    /// Check if this NPC is interactive (can be talked to).
    pub fn is_interactive(&self) -> bool {
        matches!(
            self.npc_type,
            NPCType::Elder | NPCType::Trainer | NPCType::Merchant | NPCType::Villager
        )
    }

    /// Check if this NPC is hostile.
    pub fn is_hostile(&self) -> bool {
        matches!(self.npc_type, NPCType::Creature)
    }
}
