//! The 3D Inventory System (Satchel)
//!
//! No 2D grid. Items are physical objects in a 3D bag.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// An item in the inventory.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub name: String,
    pub quantity: u32,
    pub weight: f32, // In arbitrary units
}

/// The player's satchel.
#[derive(Component, Clone, Debug)]
pub struct Satchel {
    pub items: Vec<InventoryItem>,
    pub max_weight: f32,
    pub is_open: bool,
}

impl Satchel {
    /// Create a new satchel.
    pub fn new(max_weight: f32) -> Self {
        Self {
            items: Vec::new(),
            max_weight,
            is_open: false,
        }
    }

    /// Get the current weight of the satchel.
    pub fn current_weight(&self) -> f32 {
        self.items.iter().map(|item| item.weight * item.quantity as f32).sum()
    }

    /// Check if an item can be added.
    pub fn can_add(&self, item: &InventoryItem) -> bool {
        self.current_weight() + (item.weight * item.quantity as f32) <= self.max_weight
    }

    /// Add an item to the satchel.
    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        if self.can_add(&item) {
            // Check if item already exists
            if let Some(existing) = self.items.iter_mut().find(|i| i.name == item.name) {
                existing.quantity += item.quantity;
            } else {
                self.items.push(item);
            }
            true
        } else {
            false
        }
    }

    /// Remove an item from the satchel.
    #[allow(dead_code)]
    pub fn remove_item(&mut self, name: &str, quantity: u32) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.name == name && i.quantity >= quantity) {
            self.items[pos].quantity -= quantity;
            if self.items[pos].quantity == 0 {
                self.items.remove(pos);
            }
            return true;
        }
        false
    }
}

/// Component for inventory UI display
#[derive(Component)]
pub struct InventoryUI;

/// Component for inventory item display
#[derive(Component)]
pub struct InventoryItemDisplay {
    pub item_name: String,
}

/// System to handle inventory input (Pressing 'I') and spawn/despawn UI
pub fn inventory_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Satchel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_query: Query<Entity, With<InventoryUI>>,
) {
    if query.is_empty() {
        return;
    }

    let Ok(mut satchel) = query.single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::KeyI) {
        satchel.is_open = !satchel.is_open;

        if satchel.is_open {
            spawn_inventory_ui(&mut commands, &satchel, &asset_server);
            println!("Satchel opened. Weight: {:.1}/{:.1}", satchel.current_weight(), satchel.max_weight);
            for item in &satchel.items {
                println!("  - {} x{} ({:.1} weight)", item.name, item.quantity, item.weight * item.quantity as f32);
            }
        } else {
            // Despawn inventory UI
            for entity in ui_query.iter() {
                commands.entity(entity).despawn();
            }
            println!("Satchel closed.");
        }
    }
}

/// Spawn the inventory UI panel
fn spawn_inventory_ui(
    commands: &mut Commands,
    satchel: &Satchel,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load("FiraSans-Bold.ttf");

    // Main panel
    commands
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                bottom: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            InventoryUI,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("SATCHEL"),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Weight bar
            let weight_percent = (satchel.current_weight() / satchel.max_weight).min(1.0);
            let weight_color = if weight_percent > 0.9 {
                Color::srgb(1.0, 0.0, 0.0) // Red when full
            } else if weight_percent > 0.7 {
                Color::srgb(1.0, 0.8, 0.0) // Yellow when nearly full
            } else {
                Color::srgb(0.0, 0.8, 0.0) // Green when OK
            };

            parent.spawn((
                Text::new(format!(
                    "Weight: {:.1}/{:.1} ({:.0}%)",
                    satchel.current_weight(),
                    satchel.max_weight,
                    weight_percent * 100.0
                )),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(weight_color),
            ));

            // Separator
            parent.spawn((
                Text::new("―――――――――――――――"),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    margin: UiRect::axes(Val::ZERO, Val::Px(10.0)),
                    ..default()
                },
            ));

            // Items list
            for item in &satchel.items {
                parent.spawn((
                    Text::new(format!(
                        "  {} x{}",
                        item.name,
                        item.quantity
                    )),
                    TextFont {
                        font: font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    InventoryItemDisplay {
                        item_name: item.name.clone(),
                    },
                ));
            }

            // Instructions
            parent.spawn((
                Text::new("\nPress I to close"),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.3, 0.3)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satchel_weight() {
        let mut satchel = Satchel::new(100.0);
        let item = InventoryItem {
            name: "Gopher Wood".to_string(),
            quantity: 5,
            weight: 10.0,
        };
        assert!(satchel.add_item(item));
        assert_eq!(satchel.current_weight(), 50.0);
    }

    #[test]
    fn test_satchel_overflow() {
        let mut satchel = Satchel::new(100.0);
        let item = InventoryItem {
            name: "Stone".to_string(),
            quantity: 20,
            weight: 10.0,
        };
        assert!(!satchel.add_item(item));
    }
}
