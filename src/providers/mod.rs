//! AI backend provider integrations and credential verification handlers.
//!
//! Exposes submodules providing interactive CLI authentication, API key validation,
//! model enumeration, and persistent configuration updates for supported LLM services.

pub mod anthropic;
pub mod deepseek;
pub mod gemini;
pub mod groq;
pub mod openai;
pub mod openrouter;
