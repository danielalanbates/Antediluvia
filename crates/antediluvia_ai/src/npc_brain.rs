//! NPC AI brain (state machine).
//! 
//! Manages NPC behavior, dialogue, and decision-making.

use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::dialogue::{DialogueGenerator, DialogueContext, NPCLineage};

/// NPC behavior state.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NPCState {
    Idle,
    Talking,
    Working,
    Fleeing,
    Attacking,
}

/// An NPC's brain (AI logic).
#[derive(Clone, Debug)]
pub struct NPCBrain {
    pub name: String,
    pub lineage: NPCLineage,
    pub state: NPCState,
    pub dialogue_gen: DialogueGenerator,
    pub memory: Vec<String>, // Recent interactions
}

impl NPCBrain {
    /// Create a new NPC brain.
    pub fn new(name: String, lineage: NPCLineage) -> Self {
        Self {
            name,
            lineage,
            state: NPCState::Idle,
            dialogue_gen: DialogueGenerator::new(),
            memory: Vec::new(),
        }
    }

    /// Process a player interaction.
    pub fn interact(&mut self, player_input: &str, player_corruption: f32, world_corruption: f32) -> Result<String> {
        self.state = NPCState::Talking;

        let context = DialogueContext {
            npc_name: self.name.clone(),
            npc_lineage: self.lineage,
            player_corruption,
            world_corruption,
        };

        let exchange = self.dialogue_gen.generate_response(&context, player_input)?;
        
        // Store in memory
        self.memory.push(format!("{}: {}", self.name, exchange.npc_response));
        if self.memory.len() > 10 {
            self.memory.remove(0);
        }

        Ok(exchange.npc_response)
    }

    /// Get a greeting.
    pub fn greet(&self, player_corruption: f32, world_corruption: f32) -> String {
        let context = DialogueContext {
            npc_name: self.name.clone(),
            npc_lineage: self.lineage,
            player_corruption,
            world_corruption,
        };

        self.dialogue_gen.generate_greeting(&context)
    }

    /// Update NPC state based on world conditions.
    pub fn update(&mut self, world_corruption: f32) {
        // If world corruption is high, Sethites flee or pray
        if world_corruption > 80.0 && self.lineage == NPCLineage::Seth {
            self.state = NPCState::Fleeing;
        }
        // If world corruption is high, Cainites attack or work
        else if world_corruption > 80.0 && self.lineage == NPCLineage::Cain {
            self.state = NPCState::Attacking;
        }
        // Otherwise, idle or work
        else if self.state != NPCState::Talking {
            self.state = NPCState::Idle;
        }
    }

    /// Get the NPC's opinion on a topic.
    pub fn get_opinion(&self, topic: &str) -> String {
        match self.lineage {
            NPCLineage::Seth => {
                format!("{} believes {} is a matter of faith and discipline.", self.name, topic)
            }
            NPCLineage::Cain => {
                format!("{} believes {} is a tool for power and progress.", self.name, topic)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_brain_creation() {
        let brain = NPCBrain::new("Methuselah".to_string(), NPCLineage::Seth);
        assert_eq!(brain.name, "Methuselah");
        assert_eq!(brain.state, NPCState::Idle);
    }

    #[test]
    fn test_npc_interaction() {
        let mut brain = NPCBrain::new("Methuselah".to_string(), NPCLineage::Seth);
        let response = brain.interact("Who are the Watchers?", 0.0, 25.0).unwrap();
        assert!(!response.is_empty());
        assert_eq!(brain.state, NPCState::Talking);
    }

    #[test]
    fn test_npc_state_update() {
        let mut brain = NPCBrain::new("Methuselah".to_string(), NPCLineage::Seth);
        brain.update(85.0);
        assert_eq!(brain.state, NPCState::Fleeing);
    }
}
