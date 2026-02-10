//! World events and spiritual visions.
//! Implements Nephilim raids and Jacob's Ladder vision triggers.

use serde::{Deserialize, Serialize};
use glam::Vec3;

/// Types of world events.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorldEventType {
    /// Nephilim raid on a settlement or the Ark site.
    NephilimRaid,
    /// A prophetic vision (Jacob's Ladder).
    VisionJacobLadder,
}

/// A scheduled world event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub event_type: WorldEventType,
    pub location: Vec3,
    pub trigger_time: f32,   // seconds since server start
    pub duration: f32,       // seconds
    pub active: bool,
}

impl WorldEvent {
    pub fn new(event_type: WorldEventType, location: Vec3, trigger_time: f32, duration: f32) -> Self {
        Self {
            event_type,
            location,
            trigger_time,
            duration,
            active: false,
        }
    }
}

/// Vision payload shown to players.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vision {
    pub title: String,
    pub description: String,
    pub duration: f32,
}

/// Vision templates.
pub fn jacobs_ladder_vision() -> Vision {
    Vision {
        title: "Jacob's Ladder".to_string(),
        description: "You see a stairway to heaven, angels ascending and descending. A voice whispers of a coming Deliverer.".to_string(),
        duration: 10.0,
    }
}

/// Event scheduler and runtime.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EventManager {
    pub events: Vec<WorldEvent>,
    pub time_seconds: f32,
}

impl EventManager {
    /// Create with default rotation of events.
    pub fn with_defaults() -> Self {
        let mut manager = EventManager::default();
        // Schedule a Jacob's Ladder vision at t = 300s near Bethel
        manager.events.push(WorldEvent::new(
            WorldEventType::VisionJacobLadder,
            Vec3::new(10000.0, 500.0, 10000.0),
            300.0,
            20.0,
        ));
        // Schedule a Nephilim raid at t = 600s near Ark site
        manager.events.push(WorldEvent::new(
            WorldEventType::NephilimRaid,
            Vec3::new(5000.0, 50.0, 5000.0),
            600.0,
            120.0,
        ));
        manager
    }

    /// Advance time and activate/deactivate events.
    pub fn update(&mut self, delta_seconds: f32) -> Vec<WorldEventType> {
        self.time_seconds += delta_seconds;
        let mut triggered: Vec<WorldEventType> = Vec::new();

        for evt in self.events.iter_mut() {
            if !evt.active && self.time_seconds >= evt.trigger_time {
                evt.active = true;
                triggered.push(evt.event_type.clone());
            }
            if evt.active && self.time_seconds >= evt.trigger_time + evt.duration {
                evt.active = false;
            }
        }
        triggered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_activation() {
        let mut mgr = EventManager::with_defaults();
        let fired = mgr.update(301.0);
        assert!(fired.contains(&WorldEventType::VisionJacobLadder));
    }
}
