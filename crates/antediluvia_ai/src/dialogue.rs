//! Dialogue generation system.
//! 
//! Generates NPC responses based on context, lineage, and knowledge base.

use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::knowledge_base::KnowledgeBase;

/// A dialogue exchange.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueExchange {
    pub player_input: String,
    pub npc_response: String,
    pub sentiment: Sentiment,
}

/// The sentiment of a response.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sentiment {
    Friendly,
    Neutral,
    Hostile,
    Prophetic,
    Corrupted,
}

/// Dialogue context for generating responses.
#[derive(Clone, Debug)]
pub struct DialogueContext {
    pub npc_name: String,
    pub npc_lineage: NPCLineage,
    pub player_corruption: f32,
    pub world_corruption: f32,
}

/// NPC lineage affects dialogue tone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NPCLineage {
    Seth,  // Truthful, prophetic
    Cain,  // Deceptive, ambitious
}

/// The dialogue generator.
#[derive(Clone, Debug)]
pub struct DialogueGenerator {
    knowledge_base: KnowledgeBase,
}

impl DialogueGenerator {
    /// Create a new dialogue generator.
    pub fn new() -> Self {
        Self {
            knowledge_base: KnowledgeBase::new(),
        }
    }

    /// Generate a response to a player query.
    pub fn generate_response(&self, context: &DialogueContext, query: &str) -> Result<DialogueExchange> {
        // Query the knowledge base
        if let Some(entry) = self.knowledge_base.query(query) {
            let response = match context.npc_lineage {
                NPCLineage::Seth => {
                    // Sethites tell the truth
                    format!(
                        "{}: {}",
                        context.npc_name,
                        entry.content
                    )
                }
                NPCLineage::Cain => {
                    // Cainites twist the truth
                    if entry.reliability > 0.7 {
                        format!(
                            "{}: That is ancient history. We have moved beyond such superstitions. {}",
                            context.npc_name,
                            entry.content
                        )
                    } else {
                        format!(
                            "{}: The Watchers brought us enlightenment, not corruption. {}",
                            context.npc_name,
                            entry.content
                        )
                    }
                }
            };

            let sentiment = match context.npc_lineage {
                NPCLineage::Seth => {
                    if query.to_lowercase().contains("flood") || query.to_lowercase().contains("prophecy") {
                        Sentiment::Prophetic
                    } else {
                        Sentiment::Friendly
                    }
                }
                NPCLineage::Cain => Sentiment::Corrupted,
            };

            Ok(DialogueExchange {
                player_input: query.to_string(),
                npc_response: response,
                sentiment,
            })
        } else {
            // Fallback response
            let response = match context.npc_lineage {
                NPCLineage::Seth => {
                    format!("{}: I do not know the answer to that. Perhaps the Lord will reveal it in time.", context.npc_name)
                }
                NPCLineage::Cain => {
                    format!("{}: That is not my concern. What matters is power and progress.", context.npc_name)
                }
            };

            Ok(DialogueExchange {
                player_input: query.to_string(),
                npc_response: response,
                sentiment: Sentiment::Neutral,
            })
        }
    }

    /// Generate a greeting based on player corruption.
    pub fn generate_greeting(&self, context: &DialogueContext) -> String {
        match context.npc_lineage {
            NPCLineage::Seth => {
                if context.player_corruption > 50.0 {
                    format!("{}: I sense the corruption in you. Repent, before it is too late.", context.npc_name)
                } else {
                    format!("{}: Greetings, traveler. May the Lord guide your path.", context.npc_name)
                }
            }
            NPCLineage::Cain => {
                if context.player_corruption > 50.0 {
                    format!("{}: Ah, a kindred spirit. Join us, and we shall reshape the world.", context.npc_name)
                } else {
                    format!("{}: What brings you to these lands?", context.npc_name)
                }
            }
        }
    }
}

impl Default for DialogueGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialogue_generation() {
        let gen = DialogueGenerator::new();
        let context = DialogueContext {
            npc_name: "Methuselah".to_string(),
            npc_lineage: NPCLineage::Seth,
            player_corruption: 0.0,
            world_corruption: 25.0,
        };

        let exchange = gen.generate_response(&context, "Who are the Watchers?").unwrap();
        assert!(exchange.npc_response.contains("Watchers"));
        assert_eq!(exchange.sentiment, Sentiment::Friendly);
    }

    #[test]
    fn test_cainite_dialogue() {
        let gen = DialogueGenerator::new();
        let context = DialogueContext {
            npc_name: "Tubal-Cain".to_string(),
            npc_lineage: NPCLineage::Cain,
            player_corruption: 75.0,
            world_corruption: 75.0,
        };

        let greeting = gen.generate_greeting(&context);
        assert!(greeting.contains("kindred"));
    }
}
