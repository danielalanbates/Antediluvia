//! World generation and management.
//! 
//! The Pangea Ultima procedural generation system.
//! Seed: "Genesis 6:14"

pub mod terrain;
pub mod corruption;

pub use terrain::*;
pub use corruption::*;

use serde::{Deserialize, Serialize};

/// The global world state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    /// Global Corruption Meter (0.0 to 100.0).
    /// Increases via PVP, Forbidden Tech. Decreases via Ark building, Sacrifices.
    pub corruption_level: f32,
    
    /// The Ark's construction progress (0.0 to 100.0).
    pub ark_progress: f32,
    
    /// Current server time (in-game days elapsed).
    pub server_time_days: u32,
    
    /// Weather state (affects visibility, mob behavior).
    pub weather: WeatherState,
}

/// Weather conditions in the world.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeatherState {
    Clear,
    Mist,
    LightRain,
    HeavyRain,
    Thunderstorm,
    TheDeluge, // The Flood begins
}

impl WorldState {
    /// Create a new world state at the beginning of time.
    pub fn new() -> Self {
        Self {
            corruption_level: 0.0,
            ark_progress: 0.0,
            server_time_days: 0,
            weather: WeatherState::Clear,
        }
    }

    /// Update weather based on corruption level.
    /// As corruption rises, the world darkens.
    pub fn update_weather(&mut self) {
        self.weather = match self.corruption_level {
            c if c < 50.0 => WeatherState::Clear,
            c if c < 80.0 => WeatherState::Mist,
            c if c < 90.0 => WeatherState::HeavyRain,
            c if c < 99.0 => WeatherState::Thunderstorm,
            _ => WeatherState::TheDeluge,
        };
    }

    /// Check if the world has ended (Flood has begun).
    pub fn is_flooded(&self) -> bool {
        self.corruption_level >= 100.0
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}
