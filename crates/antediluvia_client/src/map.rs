//! The Diegetic Map System
//!
//! No minimap. No GPS. Players hold physical paper maps.
//! Pressing 'M' raises the map to eye level.

use bevy::prelude::*;

/// A map item that the player can hold.
#[derive(Component, Clone, Debug)]
pub struct MapItem {
    pub region: MapRegion,
    pub is_held: bool,
}

/// The regions of the world.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MapRegion {
    Havilah,      // Starting zone
    Nod,          // East (City of Enoch)
    GopherWood,   // North (Noah's Ark)
    Bethel,       // Hidden (Jacob's Ladder)
}

impl MapRegion {
    pub fn name(&self) -> &'static str {
        match self {
            MapRegion::Havilah => "Map of Havilah",
            MapRegion::Nod => "Map of Nod",
            MapRegion::GopherWood => "Map of the Gopher Wood",
            MapRegion::Bethel => "Map of Bethel",
        }
    }
}

/// The map entity that is held in the player's hand.
#[derive(Component)]
pub struct MapEntity;

/// System to handle map input (Pressing 'M').
pub fn map_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut map_query: Query<&mut Transform, With<MapEntity>>,
) {
    if map_query.is_empty() {
        return;
    }

    let Ok(mut map_transform) = map_query.single_mut() else {
        return;
    };

    // When 'M' is pressed, raise the map to eye level
    if keyboard.pressed(KeyCode::KeyM) {
        // Lerp to "Eye Level" position
        let target = Vec3::new(0.0, -0.2, 0.5); // In front of camera
        map_transform.translation = map_transform.translation.lerp(target, 0.1);
        map_transform.rotation = Quat::IDENTITY; // Face camera
    } else {
        // Lower map to "Walking Position" (at hip)
        let target = Vec3::new(0.3, -0.5, 0.3);
        map_transform.translation = map_transform.translation.lerp(target, 0.1);
        map_transform.rotation = Quat::from_rotation_x(-1.5); // Flat in hand
    }
}

/// System to render the map region as a 2D overlay when held.
pub fn map_render_system(
    map_query: Query<&MapItem, Changed<MapItem>>,
) {
    for map in map_query.iter() {
        if map.is_held {
            println!("Viewing: {}", map.region.name());
            // In a full implementation, this would render a texture overlay
            // For now, we just log the region.
        }
    }
}
