//! Antediluvia Core Library
//! 
//! The foundational logic layer for the Antediluvian MMORPG.
//! Contains world generation, entity definitions, and shared game logic.

pub mod world;
pub mod entity;
pub mod error;
pub mod combat;
pub mod mob;
pub mod crafting;
pub mod endgame;
pub mod network;
pub mod events;

pub use world::*;
pub use entity::*;
pub use error::*;
pub use combat::*;
pub use mob::*;
pub use crafting::*;
pub use endgame::*;
pub use network::*;
pub use events::*;
