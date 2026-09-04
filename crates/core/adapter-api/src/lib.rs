//! Host-neutral contracts and canonical boundary models for external adapters.
//!
//! Concrete compositor, engine, revision, persistence, and Linux-service types
//! must not cross this boundary.

pub mod engine;
pub mod error;
pub mod host;
pub mod model;
pub mod persistence;
pub mod revision;
pub mod system;
pub mod wayland;

pub use model::{CapabilityId, InvalidCapabilityId};
