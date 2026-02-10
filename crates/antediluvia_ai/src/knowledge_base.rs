//! Knowledge base for NPC dialogue.
//! 
//! Contains biblical lore, world facts, and NPC personalities.

use serde::{Deserialize, Serialize};

/// A knowledge base entry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub topic: String,
    pub content: String,
    pub source: String, // "Genesis", "1 Enoch", "Lore", etc.
    pub reliability: f32, // 0.0 (lie) to 1.0 (truth)
}

/// The knowledge base for the world.
#[derive(Clone, Debug, Default)]
pub struct KnowledgeBase {
    entries: Vec<KnowledgeEntry>,
}

impl KnowledgeBase {
    /// Create a new knowledge base.
    pub fn new() -> Self {
        Self {
            entries: Self::default_entries(),
        }
    }

    /// Get default biblical entries.
    fn default_entries() -> Vec<KnowledgeEntry> {
        vec![
            KnowledgeEntry {
                topic: "The Watchers".to_string(),
                content: "They are the two hundred who descended on Mount Hermon. They took human wives and taught forbidden knowledge.".to_string(),
                source: "1 Enoch".to_string(),
                reliability: 1.0,
            },
            KnowledgeEntry {
                topic: "The Nephilim".to_string(),
                content: "Offspring of the Watchers and humans. They are mighty men of old, consuming all resources.".to_string(),
                source: "Genesis".to_string(),
                reliability: 1.0,
            },
            KnowledgeEntry {
                topic: "Noah".to_string(),
                content: "A preacher of righteousness. He builds the Ark to preserve life from the coming flood.".to_string(),
                source: "Genesis".to_string(),
                reliability: 1.0,
            },
            KnowledgeEntry {
                topic: "The Flood".to_string(),
                content: "The fountains of the deep will break forth. The waters will cover all the earth.".to_string(),
                source: "Genesis".to_string(),
                reliability: 1.0,
            },
            KnowledgeEntry {
                topic: "Tubal-Cain".to_string(),
                content: "Master of the forge. He teaches the art of metallurgy and warfare.".to_string(),
                source: "Genesis".to_string(),
                reliability: 0.5, // Sethites view him as fallen; Cainites view him as enlightened
            },
            KnowledgeEntry {
                topic: "The Prophecy".to_string(),
                content: "A deliverer will come. He shall bruise the serpent's head.".to_string(),
                source: "Genesis".to_string(),
                reliability: 1.0,
            },
        ]
    }

    /// Query the knowledge base.
    pub fn query(&self, topic: &str) -> Option<KnowledgeEntry> {
        self.entries
            .iter()
            .find(|e| e.topic.to_lowercase().contains(&topic.to_lowercase()))
            .cloned()
    }

    /// Get all entries about a topic.
    pub fn query_all(&self, topic: &str) -> Vec<KnowledgeEntry> {
        self.entries
            .iter()
            .filter(|e| e.topic.to_lowercase().contains(&topic.to_lowercase()))
            .cloned()
            .collect()
    }

    /// Add a new entry.
    pub fn add_entry(&mut self, entry: KnowledgeEntry) {
        self.entries.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_base_query() {
        let kb = KnowledgeBase::new();
        let entry = kb.query("Watchers");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().reliability, 1.0);
    }
}
