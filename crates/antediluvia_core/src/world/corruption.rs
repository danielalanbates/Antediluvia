//! Global Corruption Meter and End Game Protocol.
//! 
//! Tracks the moral state of the world. As corruption rises, the Flood approaches.

use serde::{Deserialize, Serialize};

/// Corruption event types.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorruptionEvent {
    /// A player killed another player (PVP murder).
    PlayerKill,
    
    /// A player crafted forbidden tech (Iron, Sorcery).
    ForbiddenTech,
    
    /// A player worships a Nephilim idol.
    IdolWorship,
    
    /// A player built part of the Ark.
    ArkConstruction,
    
    /// A player made a sacrifice (Burnt Offering).
    Sacrifice,
    
    /// A player destroyed an idol.
    IdolDestruction,
}

impl CorruptionEvent {
    /// Get the corruption delta for this event.
    /// Positive = increases corruption. Negative = decreases corruption.
    pub fn delta(&self) -> f32 {
        match self {
            CorruptionEvent::PlayerKill => 2.0,
            CorruptionEvent::ForbiddenTech => 1.5,
            CorruptionEvent::IdolWorship => 1.0,
            CorruptionEvent::ArkConstruction => -2.0,
            CorruptionEvent::Sacrifice => -1.5,
            CorruptionEvent::IdolDestruction => -1.0,
        }
    }
}

/// The End Game Protocol stages.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FloodStage {
    /// 0-50% Corruption. Age of Innocence.
    Innocence,
    
    /// 51-80% Corruption. Age of Violence.
    Violence,
    
    /// 81-99% Corruption. Age of Judgment.
    Judgment,
    
    /// 100% Corruption. The Deluge begins.
    TheFlood,
}

impl FloodStage {
    /// Get the flood stage for a given corruption level.
    pub fn from_corruption(corruption: f32) -> Self {
        match corruption {
            c if c < 50.0 => FloodStage::Innocence,
            c if c < 80.0 => FloodStage::Violence,
            c if c < 100.0 => FloodStage::Judgment,
            _ => FloodStage::TheFlood,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corruption_deltas() {
        assert!(CorruptionEvent::PlayerKill.delta() > 0.0);
        assert!(CorruptionEvent::ArkConstruction.delta() < 0.0);
    }

    #[test]
    fn test_flood_stages() {
        assert_eq!(FloodStage::from_corruption(25.0), FloodStage::Innocence);
        assert_eq!(FloodStage::from_corruption(65.0), FloodStage::Violence);
        assert_eq!(FloodStage::from_corruption(90.0), FloodStage::Judgment);
        assert_eq!(FloodStage::from_corruption(100.0), FloodStage::TheFlood);
    }
}
