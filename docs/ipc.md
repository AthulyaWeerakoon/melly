# Local IPC contract

Melly uses a Unix-domain socket for local communication between the runtime and approved clients. The reference shell is one such client. Native applications may connect through `rusty-melly` without using the shell process.

## Socket location and lifecycle

The default socket is:

```text
$XDG_RUNTIME_DIR/melly/runtime.sock
```

`XDG_RUNTIME_DIR` must be present and absolute. The runtime creates the `melly` directory for the active user with mode `0700` and exposes the socket with mode `0600`. The socket is per-user, per-login ephemeral state. It is never created in a source checkout, package installation directory, persistent state directory, shared storage mount, or a shared `/tmp` location.

The runtime owns socket creation, stale-socket handling, permissions, accepting connections, peer validation, and shutdown cleanup. A client never deletes or replaces the socket. Stale-socket removal must verify that the path is a socket owned by the active user and that no live runtime accepts connections before unlinking it.

An explicit socket-path override is limited to tests and supervised development setups. Production discovery uses the standard path.

## Protocol and authority

The protocol will begin with a versioned handshake before commands or events are accepted. The runtime determines peer credentials from the local socket and associates each connection with an application or installed-package identity. Path access and successful connection are not authorization grants.

Every operation is checked against:

- the negotiated protocol version;
- authenticated local peer and application identity;
- runtime-supported capabilities;
- permissions granted to that identity;
- current session and policy state.

The socket protocol carries Melly semantic operations and canonical models. It does not carry arbitrary shell commands, raw Sway IPC commands, backend-native identifiers, unrestricted Wayland objects, or implicit filesystem access.

## Crate ownership

- `melly-protocol` defines shared versioned wire primitives and compatibility rules.
- `rusty-melly` owns connection setup, protocol framing, request/response handling, subscriptions, and Rust client ergonomics.
- `melly-runtime` owns the server, peer validation, authorization, routing, policy, and diagnostics.
- `melly-shell` uses `rusty-melly` and receives no privileged private transport.

The HTML/CSS/JavaScript desktop does not open this socket. Its native requests use the permission-checked `melly.*` bridge provided by the shell environment.

## Current status

The standard path resolver and initial `rusty-melly` connection builder are scaffolded. Message framing, the handshake, peer-credential validation, identity binding, authorization, request routing, event subscriptions, reconnect behavior, and socket-server lifecycle are not implemented and are not yet a stable external contract.
