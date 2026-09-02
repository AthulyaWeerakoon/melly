# Melly

Melly is the native Rust runtime for a live, source-controlled Linux desktop shell. Its visible interface is supplied by a separate HTML, CSS, and JavaScript repository and will be rendered through Servo on shell surfaces provided by an existing Wayland compositor.

Melly is a shell runtime, **not a compositor**. It will not own DRM/KMS, window composition, XWayland, or general window-management policy.

## Status

This repository is an early Rust scaffold. It currently provides a dependency-free command-line placeholder so the build, formatting, linting, and testing workflow is established before graphics or engine dependencies are introduced.

The first technical milestone is a minimal Wayland layer-shell client. Servo embedding follows as a separate feasibility spike; neither is implemented yet.

## Quick start

Install stable Rust with `rustfmt` and Clippy, then run:

```sh
cargo run -- --help
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Repository responsibilities

This repository will own:

- native shell-surface lifecycle and input routing;
- the narrow Servo embedding layer;
- the permission-checked `melly.*` JavaScript bridge;
- compositor-neutral host capability contracts and adapters;
- live reload, Git generations, validation, health checks, and rollback;
- safe mode and recovery paths outside user-controlled JavaScript.

Desktop source does not belong here. The companion `melly-desktop` repository contains the no-build example desktop and its manifest.

## Architecture rules

- Upper layers depend on semantic capabilities, never Sway or another host's wire types.
- Servo-specific APIs remain inside an engine boundary.
- Repository JavaScript receives only explicitly granted native capabilities.
- A candidate revision cannot replace the known-good generation until validation and health checks pass.
- Normal desktop HTML, CSS, and JavaScript changes never require recompiling this runtime.

See [architecture](docs/architecture.md), [development](docs/development.md), and the [roadmap](docs/roadmap.md) for details.

## Contributing

Read [AGENTS.md](AGENTS.md) before changing the codebase. Keep changes focused on the current milestone and include tests for behavior that can be exercised without a live compositor.
