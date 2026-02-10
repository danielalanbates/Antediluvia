//! Error types for Antediluvia Core.

use thiserror::Error;

/// Core error type for Antediluvia.
#[derive(Error, Debug)]
pub enum AntediluviaError {
    #[error("World generation failed: {0}")]
    WorldGenError(String),

    #[error("Entity spawn failed: {0}")]
    EntitySpawnError(String),

    #[error("Invalid coordinates: {0}")]
    InvalidCoordinates(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AntediluviaError>;
