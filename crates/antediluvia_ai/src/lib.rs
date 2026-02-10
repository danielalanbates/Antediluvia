//! Antediluvia AI Module
//! 
//! LLM integration for NPC dialogue and world interactions.
//! Supports local inference (Llama-3-8B) and cloud APIs (Groq, OpenAI).

pub mod dialogue;
pub mod npc_brain;
pub mod knowledge_base;
pub mod error;

pub use dialogue::*;
pub use npc_brain::*;
pub use knowledge_base::*;
pub use error::*;
