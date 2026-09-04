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
12. Keep the runtime-to-client boundary on a per-user Unix-domain socket beneath `XDG_RUNTIME_DIR`. Never place the live socket in a repository, installation tree, persistent data directory, or shared `/tmp` path.
13. Use `rusty-melly` as the supported Rust client boundary for both the reference shell and third-party native applications. The shell must not receive a private protocol, implicit authority, or direct dependency on runtime internals.
14. Treat bypassing the shell as bypassing only the visible shell process. Every client still performs protocol negotiation and passes runtime identity, capability, permission, and policy checks.
15. Keep shared wire models in `melly-protocol`; keep transport and client ergonomics in `rusty-melly`; keep socket ownership, peer validation, authorization, dispatch, and policy in `melly-runtime`.
16. Run desktop JavaScript without ambient filesystem, process, service, compositor, socket, or machine-configuration authority. Native effects outside the renderer sandbox must pass through a declared, permission-checked `melly.*` operation.
17. Define the desktop resource root as the directory containing its manifest entry document. Canonically resolve every loaded document, module, worker, stylesheet, template, font, image, and media path beneath that root, and reject traversal, absolute paths, symlink escapes, and resolution races.
18. Keep network data authority separate from resource and filesystem authority. Permission-gated AJAX may reach approved localhost or outbound endpoints, but responses must not supply executable source or interface assets. A localhost helper acts only with its own operating-system identity and must never inherit Melly runtime privileges.

## Rust conventions

- Use stable Rust and keep `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace` passing in an environment with the required native libraries.
- Use small modules with explicit responsibilities and typed errors at architectural boundaries.
- Avoid `unsafe` unless a platform boundary genuinely requires it; document the invariant beside each unsafe block.
- Do not add large native dependencies before the milestone that needs them. Record the reason and boundary when adding Servo, Wayland, async runtime, or IPC dependencies.
- Unit-test pure policy and routing logic. Isolate compositor-dependent integration so it can be covered by fixtures or integration tests.

## Workspace boundaries

Preserve these conceptual areas even if internal modules change:

- `apps/melly`: runtime executable and application wiring;
- `crates/core/runtime`: runtime coordination, authorization, IPC server, and lifecycle;
- `crates/core/shell`: reference shell-side behavior implemented through `rusty-melly`;
- `crates/core/deployment-control`: repository generations, validation, activation, rollback, and recovery;
- `crates/core/adapter-api`: compositor- and service-neutral adapter contracts;
- `crates/core/protocol`: shared versioned IPC wire primitives;
- `crates/sdk/rusty-melly`: supported Rust client SDK;
- `crates/adapters/*`: concrete external integrations such as Sway, Servo, Smithay, Git, SQLite, and Linux services.

Update the corresponding documentation whenever a public contract, manifest assumption, security rule, or milestone changes.
