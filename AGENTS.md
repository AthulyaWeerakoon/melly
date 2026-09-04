# Repository guidance for contributors and coding agents

These instructions apply to the entire `melly` repository.

## Purpose and scope

This repository implements the native Rust runtime described in `README.md` and `docs/architecture.md`. It does not contain the user's desktop HTML/CSS/JavaScript, replace the host compositor's hardware/display responsibilities, or replace Git. An application-facing Wayland proxy/compositor role is an implementation direction to validate, not a public authoring contract.

## Required invariants

1. Preserve HTML, CSS, and JavaScript as the complete developer-facing interface for customizing the surface Melly defines. Platform complexity belongs below that contract.
2. Promise only semantics Melly can honor reliably. Do not publish a misleading API for behavior that depends on unavailable or inconsistent host support; use a documented optional capability or leave it unexposed.
3. Keep compositor-specific details behind host adapter/provider boundaries. Do not expose Sway, Labwc, or protocol-specific identifiers through the public JavaScript API.
4. Keep Servo embedding details behind a narrow engine module. Desktop manifests and host adapters must not depend on Servo types.
5. Do not vendor Servo source or build output into this repository. Use a released `servo` crate selected for the embedding milestone and commit the resolved version in `Cargo.lock`. Keep any source checkout used for upstream investigation outside this repository.
6. Authorize every native JavaScript request before it reaches a host adapter. New capabilities must default to denied.
7. Preserve recovery outside customizable web content. Never make safe mode or rollback depend exclusively on repository JavaScript.
8. Treat Git commits as candidate generations, not as automatically trusted active state.
9. Keep normal desktop-source changes runtime-loaded; do not introduce a Node.js or compilation requirement for desktop repositories.
10. After normal authorization, route applications that Melly cannot manage safely directly to the host when possible. Mark them host-managed and emit a structured diagnostic. Do not claim Melly support for the bypassed behavior.
11. Write repository documentation declaratively. State the design, contract, status, constraints, and validation criteria without persuasive comparisons, editorial opinions, or explanations of why a decision is superior.

## Rust conventions

- Use stable Rust and keep `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` passing.
- Use small modules with explicit responsibilities and typed errors at architectural boundaries.
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
