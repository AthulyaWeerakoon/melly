//! Command-shell client for controlling the Melly runtime.
//!
//! This crate is for the future interactive or command-line shell, not the
//! desktop renderer. It connects through `rusty-melly` and has no private IPC
//! path. The SDK currently only establishes a socket connection; command
//! framing and runtime dispatch are not implemented.

use std::io;

use rusty_melly::{Client, ClientBuilder, SocketPathError};

/// A command-shell connection to the Melly runtime.
#[derive(Debug)]
pub struct CommandShell {
    client: Client,
}

impl CommandShell {
    /// Connects through the standard per-user runtime socket.
    pub fn connect() -> Result<Self, CommandShellConnectError> {
        let client = ClientBuilder::from_environment()?.connect()?;
        Ok(Self { client })
    }

    /// Returns the SDK client used by the command shell.
    pub fn client(&self) -> &Client {
        &self.client
    }
}

/// Failure to resolve or connect to the runtime socket.
#[derive(Debug)]
pub enum CommandShellConnectError {
    SocketPath(SocketPathError),
    Connect(io::Error),
}

impl From<SocketPathError> for CommandShellConnectError {
    fn from(error: SocketPathError) -> Self {
        Self::SocketPath(error)
    }
}

impl From<io::Error> for CommandShellConnectError {
    fn from(error: io::Error) -> Self {
        Self::Connect(error)
    }
}
