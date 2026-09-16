# Melly

Melly is the native Rust runtime for a live, source-controlled Linux desktop whose developer-facing customization surface is HTML, CSS, and JavaScript. Its visible interface comes from a separate desktop-source repository and is intended to use normal web-platform behavior plus a small set of host-neutral Melly extensions.

Melly guarantees complete programmability of the desktop scene it defines through HTML, CSS, and JavaScript. Applications normally connect to Melly's Wayland server, and their client surfaces are composed into that scene at geometry produced by Servo layout. The application-owned pixels remain foreign content; the surrounding desktop UI and organization policy remain programmable web content. Physical display ownership, DRM/KMS, and hardware presentation remain host-compositor responsibilities.

Melly is a rootless/nested compositor from an application's perspective and a Wayland client from the display host's perspective. Sway is the initial development host, not part of the public desktop model. Smithay supplies application-facing Wayland machinery; Servo supplies the HTML/CSS/JavaScript runtime.

Melly supports native Wayland applications. X11 and XWayland are outside the supported architecture and are not routed around Melly through the display host. A future Melly-owned XWayland integration may be designed separately, but it is not part of the current contract.

## Status

This repository is an early Cargo workspace scaffold. Its runtime, command shell, deployment controller, shared IPC protocol, Rust client SDK, adapter contracts, and concrete adapter packages have explicit compilation boundaries. The SDK currently opens a Unix socket but implements no commands. The selected native libraries are locked through Cargo behind implementation-boundary features, but no socket server, protocol handshake, Rust/Servo WebView adapter, application-facing Wayland server, or functional display-host adapter is implemented yet:

- `servo-engine`: Servo 0.5.0;
- `wayland-server`: Smithay 0.7.0 with only its desktop and Wayland frontend facilities;
- `host-sway`: Smithay Client Toolkit 0.21.1 and swayipc 4.0.0.

The command-line placeholder and dependency-light crate scaffolds establish the workspace's formatting, linting, and testing workflow.

The runtime and its native clients will communicate through `$XDG_RUNTIME_DIR/melly/runtime.sock`. The Melly command shell will consume the `rusty-melly` client SDK to control the runtime; it does not own desktop rendering. Third-party native applications will be able to use the same SDK without depending on the command shell. Direct clients remain subject to Melly runtime identity, capability, permission, and policy checks.

## Quick start

Install stable Rust through `rustup`. The checked-in toolchain file selects rustfmt, Clippy, rust-src, and rust-analyzer. Then run:

```sh
cargo run -- --help
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The default build keeps the early command-line scaffold lightweight. Feature checks enable the downloaded native graphs and require the build prerequisites described in the development guide.

## Repository responsibilities

This repository will own:

- the per-user runtime socket server, shared protocol, and `rusty-melly` Rust client SDK;
- native Wayland client-surface lifecycle, scene composition, and input routing;
- the narrow Servo embedding layer;
- the distinct `melly.*` runtime, `desktop.*` scene, and permission-checked `system.*` service APIs;
- compositor-neutral host capability contracts and adapters;
- live reload, Git generations, validation, health checks, and rollback;
- safe mode and recovery paths outside user-controlled JavaScript.

Desktop source does not belong here. The companion `melly-desktop` repository contains the no-build Melly desktop and its manifest.

Third-party source and build output do not belong here either. The runtime declares released Servo, Smithay, Smithay Client Toolkit, and swayipc libraries through Cargo, and `Cargo.lock` records the exact resolved dependency graph.

## Architecture rules

- Every exposed Melly interface behavior must be controllable from HTML, CSS, and JavaScript.
- Do not expose a semantic operation until the runtime can state and test what it guarantees. Optional host features remain capability-gated.
- Upper layers depend on semantic capabilities, never Sway or another host's wire types.
- Servo-specific APIs remain inside an engine boundary.
- Repository JavaScript receives only explicitly granted native capabilities.
- Client surfaces are the foreign-UI primitive; conventional windows, workspaces, title bars, docks, and tiling are HTML presentation policy rather than mandatory Core concepts.
- The command shell and native application clients use `rusty-melly`; direct socket access never grants authority or bypasses runtime policy.
- A candidate revision cannot replace the known-good generation until validation and health checks pass.
- Normal desktop HTML, CSS, and JavaScript changes never require recompiling this runtime.

See the [workspace map](docs/workspace.md), [local IPC contract](docs/ipc.md), [architecture](docs/architecture.md), and [development guide](docs/development.md) for details.

## Contributing

Read [AGENTS.md](AGENTS.md) before changing the codebase. Keep changes focused on the current milestone and include tests for behavior that can be exercised without a live compositor.

License selection is pending: this repository has no license file or Cargo license metadata yet. Public contribution and distribution terms are not established.

For an evidence-based open-source quality audit, invoke the repo-local `melly-open-source-review` skill from this repository (for example, ask an agent to “use melly-open-source-review on melly-runtime”). It reports findings and contribution readiness without changing source.

The [contributor checks](.github/workflows/ci.yml) run Rust formatting, linting, and tests on pushes and pull requests. Optional native-feature builds remain manual until their platform prerequisites and integration tests are established. Keep credentials, machine-specific overrides, cycle plans, and scratch trials outside the Git repository. Portable contributor tooling may live under tracked `dev/` when it contains no machine-specific state.
