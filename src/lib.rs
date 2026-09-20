//! # avdoc
//!
//! `avdoc` is a command-line interface and automation core designed to orchestrate
//! AI provider configurations, interactive agent setups, and filesystem helpers.
//!
//! ## Module Architecture
//!
//! - [`commands`]: Entry points and handlers for top-level CLI interactive flows.
//! - [`config`]: Global and provider-specific persistent configuration storage models.
//! - [`helpers`]: Utilities for filesystem inspection and provider validation heuristics.
//! - [`models`]: Data transfer objects and catalog structures for models and pricing.
//! - [`providers`]: Interactive authentication and verification handlers for supported LLM backends.

pub mod helpers;
pub mod commands;
pub mod providers;
pub mod config;
pub mod models;