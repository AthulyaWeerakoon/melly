# Repository guidance for contributors and coding agents

These instructions apply to the entire `melly` repository.

## Purpose and scope

This repository implements the native Rust runtime described in `README.md` and `docs/architecture.md`. It does not contain the user's desktop HTML/CSS/JavaScript, implement a compositor, or replace Git.

## Required invariants

1. Keep compositor-specific details behind host adapter/provider boundaries. Do not expose Sway, Labwc, or protocol-specific identifiers through the public JavaScript API.
2. Keep Servo embedding details behind a narrow engine module. Desktop manifests and host adapters must not depend on Servo types.
3. Authorize every native JavaScript request before it reaches a host adapter. New capabilities must default to denied.
4. Preserve recovery outside customizable web content. Never make safe mode or rollback depend exclusively on repository JavaScript.
5. Treat Git commits as candidate generations, not as automatically trusted active state.
6. Keep normal desktop-source changes runtime-loaded; do not introduce a Node.js or compilation requirement for desktop repositories.

## Rust conventions

- Use stable Rust and keep `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` passing.
- Prefer small modules with explicit responsibilities and typed errors at architectural boundaries.
- Avoid `unsafe` unless a platform boundary genuinely requires it; document the invariant beside each unsafe block.
- Do not add large native dependencies before the milestone that needs them. Record the reason and boundary when adding Servo, Wayland, async runtime, or IPC dependencies.
- Unit-test pure policy and routing logic. Isolate compositor-dependent integration so it can be covered by fixtures or integration tests.

## Planned module boundaries

When implementation begins, preserve these conceptual areas even if exact crate names change:

- `core`: runtime coordination and lifecycle;
- `engine`: the Melly-owned interface over Servo;
- `host`: compositor-neutral capability/provider traits;
- `host/*`: concrete adapters such as Sway or generic Wayland;
- `bridge`: permission-checked `melly.*` JavaScript API;
- `deployment`: repository watching, generations, validation, activation, and rollback;
- `recovery`: safe mode and failure containment.

Update the corresponding documentation whenever a public contract, manifest assumption, security rule, or milestone changes.
