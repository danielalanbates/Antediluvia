//! Error types for AI module.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("LLM inference failed: {0}")]
    InferenceFailed(String),

    #[error("Knowledge base error: {0}")]
    KnowledgeBaseError(String),

    #[error("Dialogue generation failed: {0}")]
    DialogueError(String),

    #[error("API error: {0}")]
    APIError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AIError>;
