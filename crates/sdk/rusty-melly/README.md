# rusty-melly

`rusty-melly` is the Rust client SDK for the local Melly runtime. The reference shell and third-party native applications use the same SDK and Unix-socket protocol.

The crate currently resolves the standard per-user socket path and can establish a Unix-stream connection. Protocol framing, negotiation, requests, events, identity, and stable compatibility guarantees are not implemented yet.

During workspace development:

```toml
[dependencies]
rusty-melly = { path = "../melly/crates/sdk/rusty-melly" }
```

A Git dependency can be used from a separate checkout. Crates.io publication begins only after the protocol and package metadata are versioned.

Using this SDK bypasses the visible shell process only. It does not bypass Melly runtime authentication, permissions, capabilities, or policy.
