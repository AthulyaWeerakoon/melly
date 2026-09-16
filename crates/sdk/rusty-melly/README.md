# rusty-melly

`rusty-melly` is the Rust client SDK for the local Melly runtime. The Melly command shell and third-party native applications use the same SDK and Unix-socket protocol. The SDK currently resolves the socket path and opens a connection; protocol commands and a server are not implemented. The desktop host is a separate presentation component, not the command shell.

The crate currently resolves the standard per-user socket path and can establish a Unix-stream connection. Protocol framing, negotiation, requests, events, identity, and stable compatibility guarantees are not implemented yet.

During development, use a Cargo path dependency pointing to your checkout or a Git dependency pointing to the repository. The SDK is not published to crates.io; publication begins only after the protocol and package metadata are versioned.

Using this SDK bypasses the visible shell process only. It does not bypass Melly runtime authentication, permissions, capabilities, or policy.
