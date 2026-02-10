//! Job/Class system. Classless progression with hidden unlocks.

use serde::{Deserialize, Serialize};

/// A job mastery tracker.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JobMastery {
    pub shepherd: f32,      // Tank
    pub levite: f32,        // Healer
    pub hunter: f32,        // Melee DPS
    pub forge: f32,         // Burst DPS
    pub psalmist: f32,      // Support
}

/// The five archetypes.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Job {
    /// Shepherd of Abel. Tank. Defends the flock.
    Shepherd,
    
    /// Levitical Guardian. Healer. Tends the wounded.
    Levite,
    
    /// Nephilim Hunter. Melee DPS. Slays giants.
    Hunter,
    
    /// Disciple of the Forge. Burst DPS. Crafts weapons.
    Forge,
    
    /// Psalmist. Support. Sings buffs.
    Psalmist,
}

impl Job {
    /// Get the unlock condition description for this job.
    pub fn unlock_condition(&self) -> &'static str {
        match self {
            Job::Shepherd => "Defend a flock or NPC from predators for 5 minutes",
            Job::Levite => "Heal a wounded stranger with herbs and bandages",
            Job::Hunter => "Deal >50% damage to a Giant-class enemy",
            Job::Forge => "Craft a Masterwork quality weapon",
            Job::Psalmist => "Play a perfect rhythm on the Lyre at a campfire",
        }
    }

    /// Get the primary weapon type for this job.
    pub fn primary_weapon(&self) -> &'static str {
        match self {
            Job::Shepherd => "Staff / Sling",
            Job::Levite => "Censer",
            Job::Hunter => "2H Spear",
            Job::Forge => "Hammer",
            Job::Psalmist => "Lyre",
        }
    }

    /// Get the core mechanic for this job.
    pub fn core_mechanic(&self) -> &'static str {
        match self {
            Job::Shepherd => "Rebuke (Aggro Shout)",
            Job::Levite => "Incense Burning (Resource-based Healing)",
            Job::Hunter => "Momentum (Stacking Attack Speed)",
            Job::Forge => "Heat Management (Durability Trade-off)",
            Job::Psalmist => "Songs of Ascent (Aural Buffs)",
        }
    }
}

impl JobMastery {
    /// Get the mastery level for a specific job.
    pub fn get_level(&self, job: Job) -> f32 {
        match job {
            Job::Shepherd => self.shepherd,
            Job::Levite => self.levite,
            Job::Hunter => self.hunter,
            Job::Forge => self.forge,
            Job::Psalmist => self.psalmist,
        }
    }

    /// Increase mastery for a job.
    pub fn increase(&mut self, job: Job, amount: f32) {
        match job {
            Job::Shepherd => self.shepherd = (self.shepherd + amount).min(100.0),
            Job::Levite => self.levite = (self.levite + amount).min(100.0),
            Job::Hunter => self.hunter = (self.hunter + amount).min(100.0),
            Job::Forge => self.forge = (self.forge + amount).min(100.0),
            Job::Psalmist => self.psalmist = (self.psalmist + amount).min(100.0),
        }
    }

    /// Check if a job is unlocked (mastery > 0).
    pub fn is_unlocked(&self, job: Job) -> bool {
        self.get_level(job) > 0.0
    }

    /// Get the primary job (highest mastery).
    pub fn primary_job(&self) -> Option<Job> {
        let jobs = [
            (Job::Shepherd, self.shepherd),
            (Job::Levite, self.levite),
            (Job::Hunter, self.hunter),
            (Job::Forge, self.forge),
            (Job::Psalmist, self.psalmist),
        ];

        jobs.iter()
            .filter(|(_, level)| *level > 0.0)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(job, _)| *job)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_unlock_conditions() {
        assert!(!Job::Shepherd.unlock_condition().is_empty());
        assert!(!Job::Levite.unlock_condition().is_empty());
    }

    #[test]
    fn test_mastery_tracking() {
        let mut mastery = JobMastery::default();
        mastery.increase(Job::Shepherd, 25.0);
        assert_eq!(mastery.shepherd, 25.0);
        assert!(mastery.is_unlocked(Job::Shepherd));
        assert!(!mastery.is_unlocked(Job::Levite));
    }
}
