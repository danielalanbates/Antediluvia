//! Combat system with party mechanics and skill chains.
//! 
//! Inspired by FFXI. Mobs require parties. Skill chains provide massive damage bonuses.

use serde::{Deserialize, Serialize};

/// A combat action (ability/spell).
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CombatAction {
    // Shepherd (Tank)
    ShepherdRebuke,
    ShepherdBlock,
    
    // Levite (Healer)
    LevitePrayer,
    LeviteHeal,
    
    // Hunter (Melee DPS)
    HunterThrust,
    HunterSlash,
    
    // Forge (Burst DPS)
    ForgeSmash,
    ForgeFire,
    
    // Psalmist (Support)
    PsalmistSong,
    PsalmistBuff,
}

impl CombatAction {
    /// Get the damage dealt by this action.
    pub fn damage(&self) -> f32 {
        match self {
            CombatAction::ShepherdRebuke => 20.0,
            CombatAction::ShepherdBlock => 0.0,
            CombatAction::LevitePrayer => 0.0,
            CombatAction::LeviteHeal => 0.0,
            CombatAction::HunterThrust => 50.0,
            CombatAction::HunterSlash => 40.0,
            CombatAction::ForgeSmash => 60.0,
            CombatAction::ForgeFire => 70.0,
            CombatAction::PsalmistSong => 0.0,
            CombatAction::PsalmistBuff => 0.0,
        }
    }

    /// Get the cooldown in seconds.
    pub fn cooldown(&self) -> f32 {
        match self {
            CombatAction::ShepherdRebuke => 3.0,
            CombatAction::ShepherdBlock => 2.0,
            CombatAction::LevitePrayer => 5.0,
            CombatAction::LeviteHeal => 4.0,
            CombatAction::HunterThrust => 2.5,
            CombatAction::HunterSlash => 2.0,
            CombatAction::ForgeSmash => 3.5,
            CombatAction::ForgeFire => 4.0,
            CombatAction::PsalmistSong => 6.0,
            CombatAction::PsalmistBuff => 5.0,
        }
    }

    /// Get the action type for skill chain purposes.
    pub fn action_type(&self) -> ActionType {
        match self {
            CombatAction::ShepherdRebuke | CombatAction::ShepherdBlock => ActionType::Physical,
            CombatAction::LevitePrayer | CombatAction::LeviteHeal => ActionType::Magic,
            CombatAction::HunterThrust | CombatAction::HunterSlash => ActionType::Physical,
            CombatAction::ForgeSmash | CombatAction::ForgeFire => ActionType::Physical,
            CombatAction::PsalmistSong | CombatAction::PsalmistBuff => ActionType::Magic,
        }
    }
}

/// The type of action for skill chain matching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionType {
    Physical,
    Magic,
}

/// A skill chain (combo).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillChain {
    pub first_action: CombatAction,
    pub second_action: CombatAction,
    pub damage_multiplier: f32,
    pub name: String,
}

impl SkillChain {
    /// Create a skill chain.
    pub fn new(first: CombatAction, second: CombatAction, multiplier: f32, name: String) -> Self {
        Self {
            first_action: first,
            second_action: second,
            damage_multiplier: multiplier,
            name,
        }
    }

    /// Check if a sequence of actions matches this chain.
    pub fn matches(&self, first: CombatAction, second: CombatAction) -> bool {
        self.first_action == first && self.second_action == second
    }
}

/// The skill chain registry.
pub fn get_skill_chains() -> Vec<SkillChain> {
    vec![
        SkillChain::new(
            CombatAction::HunterThrust,
            CombatAction::ForgeSmash,
            2.5,
            "Shatter".to_string(),
        ),
        SkillChain::new(
            CombatAction::HunterSlash,
            CombatAction::ForgeFire,
            2.0,
            "Inferno".to_string(),
        ),
        SkillChain::new(
            CombatAction::ShepherdRebuke,
            CombatAction::HunterThrust,
            1.8,
            "Righteous Fury".to_string(),
        ),
    ]
}

/// Combat state for a single entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatState {
    pub in_combat: bool,
    pub current_target: Option<u64>, // Entity ID
    pub last_action: Option<CombatAction>,
    pub action_cooldown: f32,
    pub combo_window: f32, // Time remaining to complete a skill chain
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            in_combat: false,
            current_target: None,
            last_action: None,
            action_cooldown: 0.0,
            combo_window: 0.0,
        }
    }

    /// Attempt to perform an action.
    pub fn perform_action(&mut self, action: CombatAction) -> bool {
        if self.action_cooldown > 0.0 {
            return false; // Still on cooldown
        }

        self.last_action = Some(action);
        self.action_cooldown = action.cooldown();
        self.combo_window = 3.0; // 3 seconds to chain

        true
    }

    /// Check if a skill chain can be triggered.
    pub fn check_skill_chain(&self, second_action: CombatAction) -> Option<SkillChain> {
        if let Some(first_action) = self.last_action {
            if self.combo_window > 0.0 {
                for chain in get_skill_chains() {
                    if chain.matches(first_action, second_action) {
                        return Some(chain);
                    }
                }
            }
        }
        None
    }

    /// Update timers.
    pub fn update(&mut self, delta: f32) {
        self.action_cooldown = (self.action_cooldown - delta).max(0.0);
        self.combo_window = (self.combo_window - delta).max(0.0);
    }
}

impl Default for CombatState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_chain_matching() {
        let chain = SkillChain::new(
            CombatAction::HunterThrust,
            CombatAction::ForgeSmash,
            2.5,
            "Shatter".to_string(),
        );

        assert!(chain.matches(CombatAction::HunterThrust, CombatAction::ForgeSmash));
        assert!(!chain.matches(CombatAction::HunterSlash, CombatAction::ForgeSmash));
    }

    #[test]
    fn test_combat_state() {
        let mut state = CombatState::new();
        assert!(state.perform_action(CombatAction::HunterThrust));
        assert!(!state.perform_action(CombatAction::ForgeSmash)); // Still on cooldown

        state.update(3.0); // Wait for cooldown
        assert!(state.perform_action(CombatAction::ForgeSmash));
    }
}
