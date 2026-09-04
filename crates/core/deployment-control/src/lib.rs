//! Repository revisions, candidate validation, activation, rollback, and recovery.

pub mod activation;
pub mod generations;
pub mod recovery;
pub mod repository;
pub mod revisions;
pub mod rollback;
pub mod validation;

/// Identifies the current implementation stage without defining deployment policy.
pub const STATUS: &str = "scaffold";
