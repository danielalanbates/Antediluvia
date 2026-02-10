//! Crafting and Economy System
//! 
//! Players gather resources and craft items. No P2W cosmetics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A craftable item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CraftableItem {
    pub name: String,
    pub recipe: Recipe,
    pub quality: ItemQuality,
    pub corruption_cost: f32, // Crafting this increases corruption
}

/// A recipe for crafting.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub ingredients: HashMap<String, u32>,
    pub time_seconds: f32,
    pub skill_required: String,
    pub skill_level_required: u32,
}

/// Item quality tiers.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemQuality {
    Slag,      // Failed craft
    Standard,  // Normal
    Fine,      // Good
    Masterwork, // Excellent
    Legendary, // Rare
}

impl ItemQuality {
    /// Get the damage multiplier for this quality.
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            ItemQuality::Slag => 0.5,
            ItemQuality::Standard => 1.0,
            ItemQuality::Fine => 1.25,
            ItemQuality::Masterwork => 1.5,
            ItemQuality::Legendary => 2.0,
        }
    }

    /// Get the durability multiplier for this quality.
    pub fn durability_multiplier(&self) -> f32 {
        match self {
            ItemQuality::Slag => 0.5,
            ItemQuality::Standard => 1.0,
            ItemQuality::Fine => 1.5,
            ItemQuality::Masterwork => 2.0,
            ItemQuality::Legendary => 3.0,
        }
    }
}

/// A crafted item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CraftedItem {
    pub name: String,
    pub quality: ItemQuality,
    pub durability: f32,
    pub max_durability: f32,
}

impl CraftedItem {
    /// Create a new crafted item.
    pub fn new(name: String, quality: ItemQuality) -> Self {
        let max_durability = 100.0 * quality.durability_multiplier();
        Self {
            name,
            quality,
            durability: max_durability,
            max_durability,
        }
    }

    /// Degrade the item's durability.
    pub fn degrade(&mut self, amount: f32) {
        self.durability = (self.durability - amount).max(0.0);
    }

    /// Repair the item.
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }

    /// Check if the item is broken.
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }
}

/// The crafting system.
pub struct CraftingSystem {
    recipes: HashMap<String, CraftableItem>,
}

impl CraftingSystem {
    /// Create a new crafting system.
    pub fn new() -> Self {
        let mut recipes = HashMap::new();

        // Bronze Sword
        recipes.insert(
            "Bronze Sword".to_string(),
            CraftableItem {
                name: "Bronze Sword".to_string(),
                recipe: Recipe {
                    ingredients: {
                        let mut m = HashMap::new();
                        m.insert("Bronze Ingot".to_string(), 3);
                        m.insert("Leather Grip".to_string(), 1);
                        m
                    },
                    time_seconds: 30.0,
                    skill_required: "Metalworking".to_string(),
                    skill_level_required: 10,
                },
                quality: ItemQuality::Standard,
                corruption_cost: 0.0,
            },
        );

        // Iron Sword (Forbidden Tech)
        recipes.insert(
            "Iron Sword".to_string(),
            CraftableItem {
                name: "Iron Sword".to_string(),
                recipe: Recipe {
                    ingredients: {
                        let mut m = HashMap::new();
                        m.insert("Iron Ingot".to_string(), 3);
                        m.insert("Leather Grip".to_string(), 1);
                        m
                    },
                    time_seconds: 40.0,
                    skill_required: "Metalworking".to_string(),
                    skill_level_required: 25,
                },
                quality: ItemQuality::Fine,
                corruption_cost: 5.0, // Crafting iron increases corruption
            },
        );

        // Linen Tunic
        recipes.insert(
            "Linen Tunic".to_string(),
            CraftableItem {
                name: "Linen Tunic".to_string(),
                recipe: Recipe {
                    ingredients: {
                        let mut m = HashMap::new();
                        m.insert("Linen Cloth".to_string(), 5);
                        m.insert("Thread".to_string(), 2);
                        m
                    },
                    time_seconds: 20.0,
                    skill_required: "Weaving".to_string(),
                    skill_level_required: 5,
                },
                quality: ItemQuality::Standard,
                corruption_cost: 0.0,
            },
        );

        Self { recipes }
    }

    /// Get a recipe.
    pub fn get_recipe(&self, item_name: &str) -> Option<&CraftableItem> {
        self.recipes.get(item_name)
    }

    /// Attempt to craft an item.
    pub fn craft(
        &self,
        item_name: &str,
        skill_level: u32,
        ingredients: &HashMap<String, u32>,
    ) -> Option<CraftedItem> {
        let recipe_item = self.get_recipe(item_name)?;

        // Check skill requirement
        if skill_level < recipe_item.recipe.skill_level_required {
            return None;
        }

        // Check ingredients
        for (ingredient, required_qty) in &recipe_item.recipe.ingredients {
            let available = ingredients.get(ingredient).copied().unwrap_or(0);
            if available < *required_qty {
                return None;
            }
        }

        // Determine quality based on skill
        let quality = if skill_level >= recipe_item.recipe.skill_level_required + 20 {
            ItemQuality::Masterwork
        } else if skill_level >= recipe_item.recipe.skill_level_required + 10 {
            ItemQuality::Fine
        } else {
            ItemQuality::Standard
        };

        Some(CraftedItem::new(item_name.to_string(), quality))
    }

    /// Get all recipes.
    pub fn get_all_recipes(&self) -> Vec<&CraftableItem> {
        self.recipes.values().collect()
    }
}

impl Default for CraftingSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crafting() {
        let system = CraftingSystem::new();
        let mut ingredients = HashMap::new();
        ingredients.insert("Bronze Ingot".to_string(), 3);
        ingredients.insert("Leather Grip".to_string(), 1);

        let item = system.craft("Bronze Sword", 15, &ingredients);
        assert!(item.is_some());
        assert_eq!(item.unwrap().quality, ItemQuality::Fine);
    }

    #[test]
    fn test_item_durability() {
        let mut item = CraftedItem::new("Bronze Sword".to_string(), ItemQuality::Masterwork);
        assert_eq!(item.max_durability, 200.0);
        item.degrade(50.0);
        assert_eq!(item.durability, 150.0);
    }
}
