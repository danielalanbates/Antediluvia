//! The Flood End Game Event
//! 
//! When corruption reaches 100%, the world floods. Players must board the Ark or perish.

use serde::{Deserialize, Serialize};

/// The flood event state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FloodPhase {
    /// 0-99% corruption. Normal gameplay.
    PreFlood,
    
    /// 100% corruption. The fountains of the deep break forth.
    FloodBegins,
    
    /// Water level rising. Players have 7 real-time days to board.
    Rising,
    
    /// Water covers all peaks. The Ark is the only refuge.
    Deluge,
    
    /// The door closes. Game over for those outside.
    DoorClosed,
}

/// The Flood event manager.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FloodEvent {
    pub phase: FloodPhase,
    pub water_level: f32, // 0.0 (ground) to 1.0 (peak)
    pub time_remaining_seconds: f32,
    pub players_on_ark: Vec<u64>, // Player IDs
    pub players_drowned: Vec<u64>,
}

impl FloodEvent {
    /// Create a new flood event.
    pub fn new() -> Self {
        Self {
            phase: FloodPhase::PreFlood,
            water_level: 0.0,
            time_remaining_seconds: 604800.0, // 7 days in seconds
            players_on_ark: Vec::new(),
            players_drowned: Vec::new(),
        }
    }

    /// Trigger the flood.
    pub fn begin_flood(&mut self) {
        self.phase = FloodPhase::FloodBegins;
        self.water_level = 0.1;
    }

    /// Update the flood state.
    pub fn update(&mut self, delta_seconds: f32) {
        match self.phase {
            FloodPhase::PreFlood => {
                // Do nothing
            }
            FloodPhase::FloodBegins => {
                self.phase = FloodPhase::Rising;
                self.water_level = 0.1;
            }
            FloodPhase::Rising => {
                // Water rises slowly
                self.water_level += delta_seconds / 604800.0; // Rise over 7 days
                self.time_remaining_seconds -= delta_seconds;

                if self.water_level >= 1.0 {
                    self.phase = FloodPhase::Deluge;
                    self.water_level = 1.0;
                }

                if self.time_remaining_seconds <= 0.0 {
                    self.phase = FloodPhase::DoorClosed;
                }
            }
            FloodPhase::Deluge => {
                self.water_level = 1.0;
                self.time_remaining_seconds -= delta_seconds;

                if self.time_remaining_seconds <= 0.0 {
                    self.phase = FloodPhase::DoorClosed;
                }
            }
            FloodPhase::DoorClosed => {
                // Game over
                self.water_level = 1.0;
            }
        }
    }

    /// Check if a player at a given height is safe.
    pub fn is_safe(&self, player_height: f32, max_height: f32) -> bool {
        let water_height = self.water_level * max_height;
        player_height > water_height
    }

    /// Board a player on the Ark.
    pub fn board_ark(&mut self, player_id: u64) {
        if !self.players_on_ark.contains(&player_id) {
            self.players_on_ark.push(player_id);
        }
    }

    /// Drown a player.
    pub fn drown_player(&mut self, player_id: u64) {
        if !self.players_drowned.contains(&player_id) {
            self.players_drowned.push(player_id);
        }
    }

    /// Get the percentage of players saved.
    pub fn survival_rate(&self, total_players: usize) -> f32 {
        if total_players == 0 {
            0.0
        } else {
            (self.players_on_ark.len() as f32 / total_players as f32) * 100.0
        }
    }
}

impl Default for FloodEvent {
    fn default() -> Self {
        Self::new()
    }
}

/// The Ark entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ark {
    pub position: glam::Vec3,
    pub capacity: usize,
    pub animals_loaded: usize,
    pub max_animals: usize,
    pub is_sealed: bool,
}

impl Ark {
    /// Create a new Ark.
    pub fn new(position: glam::Vec3) -> Self {
        Self {
            position,
            capacity: 100, // Max 100 players
            animals_loaded: 0,
            max_animals: 1000,
            is_sealed: false,
        }
    }

    /// Check if the Ark is full.
    pub fn is_full(&self) -> bool {
        self.animals_loaded >= self.max_animals
    }

    /// Load an animal.
    pub fn load_animal(&mut self) -> bool {
        if !self.is_full() {
            self.animals_loaded += 1;
            true
        } else {
            false
        }
    }

    /// Seal the Ark (close the door).
    pub fn seal(&mut self) {
        self.is_sealed = true;
    }

    /// Get the Ark's readiness (0.0 to 1.0).
    pub fn readiness(&self) -> f32 {
        (self.animals_loaded as f32 / self.max_animals as f32).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_event() {
        let mut flood = FloodEvent::new();
        assert_eq!(flood.phase, FloodPhase::PreFlood);
        
        flood.begin_flood();
        assert_eq!(flood.phase, FloodPhase::FloodBegins);
    }

    #[test]
    fn test_ark() {
        let mut ark = Ark::new(glam::Vec3::new(5000.0, 50.0, 5000.0));
        assert!(!ark.is_full());
        
        for _ in 0..1000 {
            ark.load_animal();
        }
        
        assert!(ark.is_full());
        assert_eq!(ark.readiness(), 1.0);
    }

    #[test]
    fn test_survival_rate() {
        let mut flood = FloodEvent::new();
        flood.board_ark(1);
        flood.board_ark(2);
        flood.drown_player(3);
        
        assert_eq!(flood.survival_rate(3), 66.66666);
    }
}
