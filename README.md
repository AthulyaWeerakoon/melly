# Melly

Melly is the native Rust runtime for a live, source-controlled Linux desktop whose developer-facing customization surface is HTML, CSS, and JavaScript. Its visible interface comes from a separate desktop-source repository and is intended to use normal web-platform behavior plus a small set of host-neutral Melly extensions.

Melly guarantees complete programmability of the interface surface it defines through HTML, CSS, and JavaScript. It does not guarantee control of all Linux or compositor features. The runtime places Servo, Wayland, permissions, deployment, and host integration behind semantic `melly.*` APIs. Physical display ownership, DRM/KMS, global composition, and other host responsibilities remain outside the Melly contract.

The native topology is not finalized. The current direction includes a rootless Wayland proxy/compositor layer for Melly-managed applications. This layer remains below the HTML/CSS/JavaScript contract and does not expose host-specific concepts to desktop source.

Legacy X11 applications are host-managed through the host compositor's XWayland path from the first version and do not pass through Melly's proxy. Other application or protocol cases that Melly cannot mediate safely use the same host-managed path when authorized and supported by the host. Melly logs each declined-management decision. Host-managed applications have no guarantee of Melly chrome, events, or control.

## Status

This repository is an early Cargo workspace scaffold. Its runtime, reference shell, deployment controller, shared IPC protocol, public Rust client SDK, adapter contracts, and concrete adapter packages now have explicit compilation boundaries. The selected native libraries are locked through Cargo behind implementation-boundary features, but no socket server, protocol handshake, WebView, Wayland proxy, or functional Sway adapter is implemented yet:

- `servo-engine`: Servo 0.5.0;
- `wayland-proxy`: Smithay 0.7.0 with only its desktop and Wayland frontend facilities;
- `host-sway`: Smithay Client Toolkit 0.21.1 and swayipc 4.0.0.

The command-line placeholder and dependency-light crate scaffolds establish the workspace's formatting, linting, and testing workflow.

The runtime and reference shell will communicate through `$XDG_RUNTIME_DIR/melly/runtime.sock`. The shell consumes the `rusty-melly` client SDK. Third-party native applications will be able to use the same SDK to integrate with the runtime without depending on the shell. Direct clients remain subject to Melly runtime identity, capability, permission, and policy checks.

The first architectural proof will combine Wayland and Servo narrowly enough to validate one HTML-controlled native surface/window path, input, one semantic action, and live source reload. The rootless proxy direction and Servo embedding are feasibility work; neither is implemented or guaranteed yet.

## Quick start

Install stable Rust through `rustup`. The checked-in toolchain file selects rustfmt, Clippy, rust-src, and rust-analyzer. Then run:

```sh
cargo run -- --help
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

The default build keeps the early command-line scaffold lightweight. Feature checks enable the downloaded native graphs and require the build prerequisites described in the development guide.

## Repository responsibilities

This repository will own:

- the per-user runtime socket server, shared protocol, and `rusty-melly` Rust client SDK;
- native Melly surface/window lifecycle and input routing for the paths the prototypes validate;
- the narrow Servo embedding layer;
- the permission-checked `melly.*` JavaScript bridge;
- compositor-neutral host capability contracts and adapters;
- live reload, Git generations, validation, health checks, and rollback;
- safe mode and recovery paths outside user-controlled JavaScript.

Desktop source does not belong here. The companion `melly-desktop` repository contains the no-build example desktop and its manifest.

Third-party source and build output do not belong here either. The runtime declares released Servo, Smithay, Smithay Client Toolkit, and swayipc libraries through Cargo, and `Cargo.lock` records the exact resolved dependency graph.

## Architecture rules

- Every exposed Melly interface behavior must be controllable from HTML, CSS, and JavaScript.
- Do not expose a semantic operation until the runtime can state and test what it guarantees. Optional host features remain capability-gated.
- Upper layers depend on semantic capabilities, never Sway or another host's wire types.
- Servo-specific APIs remain inside an engine boundary.
- Repository JavaScript receives only explicitly granted native capabilities.
- The reference shell and native application clients use `rusty-melly`; direct socket access never grants authority or bypasses runtime policy.
- A candidate revision cannot replace the known-good generation until validation and health checks pass.
- Normal desktop HTML, CSS, and JavaScript changes never require recompiling this runtime.

See the [workspace map](docs/workspace.md), [local IPC contract](docs/ipc.md), [architecture](docs/architecture.md), [development guide](docs/development.md), and [roadmap](docs/roadmap.md) for details.

## Contributing

Read [AGENTS.md](AGENTS.md) before changing the codebase. Keep changes focused on the current milestone and include tests for behavior that can be exercised without a live compositor.
