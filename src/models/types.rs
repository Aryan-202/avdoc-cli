//! Domain models for AI providers, available models, and token cost pricing structures.
//!
//! Provides serializable schemas used to structure model listings and execution cost metrics.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Catalog containing model collections grouped by provider name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCatalog {
    /// Map of provider names to their list of available [`ModelEntry`] specifications.
    pub providers: HashMap<String, Vec<ModelEntry>>,
}

/// Metadata description of an individual model offering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    /// Unique identifier or API slug of the model.
    pub id: String,
    /// Human-readable display name of the model.
    pub name: String,
    /// Maximum context window size in tokens, if specified.
    pub context_length: Option<usize>,
    /// Token cost pricing tier information, if available.
    pub pricing: Option<Pricing>,
}

/// Pricing metrics per token unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    /// Cost per input token (or per million tokens depending on schema usage).
    pub input: f64,
    /// Cost per output/generated token.
    pub output: f64,
}