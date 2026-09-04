//! Reference shell-side coordination for Melly surfaces and desktop source.
//!
//! The shell is an ordinary client of the runtime through `rusty-melly`. It has
//! no private IPC path and does not expose the Unix socket to desktop JavaScript.

pub mod components;
pub mod input;
pub mod manifest;
pub mod outputs;
pub mod reload;
pub mod rendering;
pub mod resources;
pub mod surfaces;

use std::io;

use rusty_melly::{Client, ClientBuilder, SocketPathError};

/// A reference-shell connection to the Melly runtime.
#[derive(Debug)]
pub struct ShellRuntime {
    client: Client,
}

impl ShellRuntime {
    /// Connects through the standard per-user runtime socket.
    pub fn connect() -> Result<Self, ShellConnectError> {
        let client = ClientBuilder::from_environment()?.connect()?;
        Ok(Self { client })
    }

    /// Returns the SDK client used by the shell implementation.
    pub fn client(&self) -> &Client {
        &self.client
    }
}

/// Failure to resolve or connect to the runtime socket.
#[derive(Debug)]
pub enum ShellConnectError {
    SocketPath(SocketPathError),
    Connect(io::Error),
}

impl From<SocketPathError> for ShellConnectError {
    fn from(error: SocketPathError) -> Self {
        Self::SocketPath(error)
    }
}

impl From<io::Error> for ShellConnectError {
    fn from(error: io::Error) -> Self {
        Self::Connect(error)
    }
}
