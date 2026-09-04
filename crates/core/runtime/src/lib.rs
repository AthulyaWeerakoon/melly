//! Melly session coordination, policy, messaging, storage, and diagnostics.
//!
//! The runtime owns the server side of Melly IPC. Socket clients never bypass
//! runtime authorization or capability checks.

pub mod api;
pub mod configuration;
pub mod diagnostics;
pub mod messaging;
pub mod security;
pub mod session;
pub mod storage;

use std::path::PathBuf;

use melly_protocol::{SocketPathError, default_socket_path};

/// Runtime configuration that is independent of a compositor implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// Per-user Unix-domain socket exposed to approved local clients.
    pub socket_path: PathBuf,
}

impl RuntimeConfig {
    /// Resolves the standard per-login runtime configuration.
    pub fn from_environment() -> Result<Self, SocketPathError> {
        Ok(Self {
            socket_path: default_socket_path()?,
        })
    }
}
