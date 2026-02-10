//! Mob AI system with Pack Tactics.
//! 
//! Mobs hunt in coordinated groups. Solo players cannot defeat them.

use serde::{Deserialize, Serialize};
use glam::Vec3;

/// A mob (hostile creature).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mob {
    pub id: u64,
    pub name: String,
    pub mob_type: MobType,
    pub position: Vec3,
    pub health: f32,
    pub max_health: f32,
    pub level: u32,
    pub pack_id: Option<u32>, // Which pack this mob belongs to
    pub aggro_range: f32,
    pub is_aggressive: bool,
}

/// Types of mobs.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MobType {
    Wolf,
    Lion,
    Nephilim,
    Chimera,
    Corrupted,
}

impl MobType {
    /// Get the base HP for this mob type.
    pub fn base_hp(&self) -> f32 {
        match self {
            MobType::Wolf => 50.0,
            MobType::Lion => 100.0,
            MobType::Nephilim => 500.0,
            MobType::Chimera => 200.0,
            MobType::Corrupted => 150.0,
        }
    }

    /// Get the base damage for this mob type.
    pub fn base_damage(&self) -> f32 {
        match self {
            MobType::Wolf => 10.0,
            MobType::Lion => 20.0,
            MobType::Nephilim => 50.0,
            MobType::Chimera => 30.0,
            MobType::Corrupted => 25.0,
        }
    }

    /// Get the aggro range for this mob type.
    pub fn aggro_range(&self) -> f32 {
        match self {
            MobType::Wolf => 50.0,
            MobType::Lion => 75.0,
            MobType::Nephilim => 200.0,
            MobType::Chimera => 100.0,
            MobType::Corrupted => 80.0,
        }
    }
}

impl Mob {
    /// Create a new mob.
    pub fn new(id: u64, name: String, mob_type: MobType, position: Vec3, level: u32) -> Self {
        let max_health = mob_type.base_hp() * (1.0 + level as f32 * 0.1);
        Self {
            id,
            name,
            mob_type,
            position,
            health: max_health,
            max_health,
            level,
            pack_id: None,
            aggro_range: mob_type.aggro_range(),
            is_aggressive: false,
        }
    }

    /// Check if the mob is alive.
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    /// Apply damage to the mob.
    pub fn take_damage(&mut self, damage: f32) {
        self.health = (self.health - damage).max(0.0);
    }

    /// Get the damage this mob deals.
    pub fn get_damage(&self) -> f32 {
        self.mob_type.base_damage() * (1.0 + self.level as f32 * 0.05)
    }

    /// Check if a target is in aggro range.
    pub fn is_in_range(&self, target_pos: Vec3) -> bool {
        let distance = self.position.distance(target_pos);
        distance <= self.aggro_range
    }
}

/// A pack of mobs that hunt together.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobPack {
    pub id: u32,
    pub mobs: Vec<u64>, // Mob IDs
    pub leader_id: u64,
    pub target: Option<u64>, // Current target (player ID)
    pub coordination_level: f32, // 0.0 (chaotic) to 1.0 (perfect)
}

impl MobPack {
    /// Create a new pack.
    pub fn new(id: u32, leader_id: u64) -> Self {
        Self {
            id,
            mobs: vec![leader_id],
            leader_id,
            target: None,
            coordination_level: 0.8, // Packs are well-coordinated
        }
    }

    /// Add a mob to the pack.
    pub fn add_mob(&mut self, mob_id: u64) {
        if !self.mobs.contains(&mob_id) {
            self.mobs.push(mob_id);
        }
    }

    /// Remove a mob from the pack.
    pub fn remove_mob(&mut self, mob_id: u64) {
        self.mobs.retain(|&id| id != mob_id);
    }

    /// Get the pack size.
    pub fn size(&self) -> usize {
        self.mobs.len()
    }

    /// Check if the pack should attack (requires 2+ mobs or high coordination).
    pub fn should_attack(&self) -> bool {
        self.size() >= 2 || self.coordination_level > 0.9
    }
}

/// Pack Tactics AI behavior.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackTacticsAI {
    pub packs: Vec<MobPack>,
    pub next_pack_id: u32,
}

impl PackTacticsAI {
    /// Create a new Pack Tactics AI.
    pub fn new() -> Self {
        Self {
            packs: Vec::new(),
            next_pack_id: 1,
        }
    }

    /// Create a new pack and return its ID.
    pub fn create_pack(&mut self, leader_id: u64) -> u32 {
        let pack_id = self.next_pack_id;
        self.packs.push(MobPack::new(pack_id, leader_id));
        self.next_pack_id += 1;
        pack_id
    }

    /// Find nearby mobs to recruit into a pack.
    pub fn recruit_nearby(&mut self, pack_id: u32, mobs: &[Mob], recruit_range: f32) {
        if let Some(pack) = self.packs.iter_mut().find(|p| p.id == pack_id) {
            let leader = mobs.iter().find(|m| m.id == pack.leader_id);
            if let Some(leader) = leader {
                for mob in mobs {
                    if !pack.mobs.contains(&mob.id) && mob.mob_type == leader.mob_type {
                        let distance = leader.position.distance(mob.position);
                        if distance <= recruit_range {
                            pack.add_mob(mob.id);
                        }
                    }
                }
            }
        }
    }

    /// Get the total damage output of a pack.
    pub fn get_pack_damage(&self, pack_id: u32, mobs: &[Mob]) -> f32 {
        if let Some(pack) = self.packs.iter().find(|p| p.id == pack_id) {
            pack.mobs
                .iter()
                .filter_map(|&mob_id| mobs.iter().find(|m| m.id == mob_id))
                .map(|m| m.get_damage())
                .sum()
        } else {
            0.0
        }
    }
}

impl Default for PackTacticsAI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mob_creation() {
        let mob = Mob::new(1, "Wolf".to_string(), MobType::Wolf, Vec3::ZERO, 1);
        assert_eq!(mob.id, 1);
        assert!(mob.is_alive());
    }

    #[test]
    fn test_pack_tactics() {
        let mut ai = PackTacticsAI::new();
        let pack_id = ai.create_pack(1);
        assert_eq!(pack_id, 1);
        assert_eq!(ai.packs[0].size(), 1);
    }

    #[test]
    fn test_pack_recruitment() {
        let mut ai = PackTacticsAI::new();
        let pack_id = ai.create_pack(1);
        
        let mobs = vec![
            Mob::new(1, "Wolf".to_string(), MobType::Wolf, Vec3::ZERO, 1),
            Mob::new(2, "Wolf".to_string(), MobType::Wolf, Vec3::new(30.0, 0.0, 0.0), 1),
        ];
        
        ai.recruit_nearby(pack_id, &mobs, 50.0);
        assert_eq!(ai.packs[0].size(), 2);
    }
}
