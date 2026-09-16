//! Public Rust client SDK for communicating with the Melly runtime.
//!
//! The shell and third-party native applications use this same
//! boundary. Connecting directly bypasses the shell process, not runtime
//! authentication, permission checks, capability checks, or policy.

use std::io;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

pub use melly_protocol::{SocketPathError, default_socket_path};

/// Builder for a local connection to the Melly runtime.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    socket_path: PathBuf,
}

impl ClientBuilder {
    /// Uses the standard socket beneath `XDG_RUNTIME_DIR`.
    pub fn from_environment() -> Result<Self, SocketPathError> {
        Ok(Self {
            socket_path: default_socket_path()?,
        })
    }

    /// Uses an explicit socket path, primarily for tests and supervised setups.
    pub fn with_socket_path(socket_path: impl Into<PathBuf>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    /// Opens the Unix-domain socket.
    ///
    /// A protocol handshake and capability negotiation will be added before
    /// this API is declared stable.
    pub fn connect(self) -> io::Result<Client> {
        let stream = UnixStream::connect(&self.socket_path)?;
        Ok(Client {
            stream,
            socket_path: self.socket_path,
        })
    }
}

/// An open connection to the local Melly runtime.
#[derive(Debug)]
pub struct Client {
    stream: UnixStream,
    socket_path: PathBuf,
}

impl Client {
    /// Returns the socket used by this connection.
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Returns the address reported for the connected runtime endpoint.
    pub fn peer_addr(&self) -> io::Result<std::os::unix::net::SocketAddr> {
        self.stream.peer_addr()
    }
}

#[cfg(test)]
mod tests {
    use super::ClientBuilder;
    use std::io::ErrorKind;
    use std::os::unix::net::UnixListener;
    use std::path::Path;

    #[test]
    fn explicit_socket_path_is_preserved() {
        let builder = ClientBuilder::with_socket_path("/run/user/1000/melly/test.sock");
        assert_eq!(
            builder.socket_path,
            Path::new("/run/user/1000/melly/test.sock")
        );
    }

    #[test]
    fn connects_to_an_explicit_socket_in_a_private_test_directory() {
        let directory = tempfile::tempdir().unwrap();
        let socket_path = directory.path().join("runtime.sock");
        let _listener = UnixListener::bind(&socket_path).unwrap();

        let client = ClientBuilder::with_socket_path(&socket_path)
            .connect()
            .unwrap();
        assert_eq!(client.socket_path(), socket_path.as_path());
        assert!(client.peer_addr().is_ok());
    }

    #[test]
    fn missing_socket_reports_a_connection_error() {
        let directory = tempfile::tempdir().unwrap();
        let error = ClientBuilder::with_socket_path(directory.path().join("missing.sock"))
            .connect()
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NotFound);
    }
}
