//! Shared primitives for the versioned Melly IPC contract.
//!
//! This crate contains transport-neutral values shared by the runtime server and
//! client SDKs. It does not grant capabilities or expose compositor-native data.

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

/// Name of the private directory created beneath `XDG_RUNTIME_DIR`.
pub const SOCKET_DIRECTORY: &str = "melly";

/// Name of the runtime's Unix-domain socket.
pub const SOCKET_FILE: &str = "runtime.sock";

/// Returns the default per-login Melly socket path.
pub fn default_socket_path() -> Result<PathBuf, SocketPathError> {
    let runtime_directory = env::var_os("XDG_RUNTIME_DIR")
        .filter(|value| !value.is_empty())
        .ok_or(SocketPathError::MissingRuntimeDirectory)?;

    socket_path_in(runtime_directory)
}

/// Resolves a Melly socket path beneath an explicit runtime directory.
pub fn socket_path_in(runtime_directory: impl AsRef<Path>) -> Result<PathBuf, SocketPathError> {
    let runtime_directory = runtime_directory.as_ref();
    if !runtime_directory.is_absolute() {
        return Err(SocketPathError::RuntimeDirectoryNotAbsolute(
            runtime_directory.as_os_str().to_owned(),
        ));
    }

    Ok(runtime_directory.join(SOCKET_DIRECTORY).join(SOCKET_FILE))
}

/// Failure to resolve the per-user runtime socket location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketPathError {
    /// `XDG_RUNTIME_DIR` was missing or empty.
    MissingRuntimeDirectory,
    /// The supplied runtime directory was not absolute.
    RuntimeDirectoryNotAbsolute(OsString),
}

impl fmt::Display for SocketPathError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRuntimeDirectory => {
                formatter.write_str("XDG_RUNTIME_DIR is missing or empty")
            }
            Self::RuntimeDirectoryNotAbsolute(path) => write!(
                formatter,
                "XDG_RUNTIME_DIR must be absolute, got {:?}",
                Path::new(path)
            ),
        }
    }
}

impl std::error::Error for SocketPathError {}

#[cfg(test)]
mod tests {
    use super::{SOCKET_DIRECTORY, SOCKET_FILE, SocketPathError, socket_path_in};
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn socket_lives_under_the_runtime_directory() {
        assert_eq!(
            socket_path_in("/run/user/1000").unwrap(),
            PathBuf::from("/run/user/1000")
                .join(SOCKET_DIRECTORY)
                .join(SOCKET_FILE)
        );
    }

    #[test]
    fn relative_runtime_directories_are_rejected() {
        assert_eq!(
            socket_path_in("tmp/runtime").unwrap_err(),
            SocketPathError::RuntimeDirectoryNotAbsolute(OsString::from("tmp/runtime"))
        );
    }
}
