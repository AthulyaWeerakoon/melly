# Repository guidance for contributors and coding agents

These instructions apply to the entire `melly` repository.

## Purpose and scope

This repository implements the native Rust runtime described in `README.md` and `docs/architecture.md`. It does not contain the user's desktop HTML/CSS/JavaScript, replace the display host's hardware responsibilities, or replace Git. Melly is the application-facing Wayland compositor and owns the programmable desktop scene presented through a separate Wayland display host.

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
10. Support native Wayland applications through Melly's application-facing server. Do not route unsupported clients directly to the display host as a compatibility mode. X11 and XWayland are unsupported until a separately designed Melly-owned integration exists.
11. Write repository documentation declaratively. State the design, contract, status, constraints, and validation criteria without persuasive comparisons, editorial opinions, or explanations of why a decision is superior.
12. Keep the runtime-to-client boundary on a per-user Unix-domain socket beneath `XDG_RUNTIME_DIR`. Never place the live socket in a repository, installation tree, persistent data directory, or shared `/tmp` path.
13. Use `rusty-melly` as the supported Rust client boundary for the Melly command shell, desktop host where needed, and third-party native applications. None receives a private protocol, implicit authority, or direct dependency on runtime internals.
14. Treat bypassing the command shell as direct use of the runtime SDK, not as bypassing runtime policy. Every client still performs protocol negotiation and passes runtime identity, capability, permission, and policy checks when those mechanisms are implemented.
15. Keep shared wire models in `melly-protocol`; keep transport and client ergonomics in `rusty-melly`; keep socket ownership, peer validation, authorization, dispatch, and policy in `melly-runtime`.
16. Run desktop JavaScript without ambient filesystem, process, service, compositor, socket, or machine-configuration authority. Use standard web APIs for existing web concepts; reserve `melly.*` for Melly environment, granted permissions, configuration, and runtime state; use `desktop.*` for scene and client-surface interactions; and use permission-checked `system.*` for operating-system services. Native effects outside the renderer sandbox require a declared fine-grained manifest permission.
17. Define the desktop resource root as the directory containing its manifest entry document. Canonically resolve every loaded document, module, worker, stylesheet, template, font, image, and media path beneath that root, and reject traversal, absolute paths, symlink escapes, and resolution races.
18. Keep network data authority separate from resource and filesystem authority. Permission-gated AJAX may reach approved localhost or outbound endpoints, but responses must not supply executable source or interface assets. A localhost helper acts only with its own operating-system identity and must never inherit Melly runtime privileges.
19. Keep credentials, per-developer environment configuration, machine-specific paths, local-only overrides, and scratch trials outside this Git repository. Committed tests, runnable code, and contributor documentation must each serve a maintained repository purpose.
20. Reserve **Melly shell** for the command shell that controls Melly through `rusty-melly`. The desktop host presents the HTML interface through the Servo engine and is not the command shell. Do not place rendering, desktop resources, surface lifecycle, or input-compositing code in `crates/core/shell`.
21. Keep client surfaces as the fundamental foreign-UI primitive. Conventional windows, workspaces, decorations, launchers, docks, and tiling are desktop presentation policy unless an unavoidable protocol or hardware semantic requires a Core representation.
22. Implement the production desktop host in Rust with Servo. Python/WebKit probes, cycle plans, machine paths, credentials, and disposable trials belong outside this repository and must not appear in committed source, tests, examples, or CI.
23. Keep portable contributor setup and prototyping automation under tracked `dev/` only when it is machine-neutral, credential-free, maintained, and directly supports the repository.

## Rust conventions

- Use stable Rust and keep `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace --locked` passing in an environment with the required native libraries.
- Use small modules with explicit responsibilities and typed errors at architectural boundaries.
- Avoid `unsafe` unless a platform boundary genuinely requires it; document the invariant beside each unsafe block.
- Do not add large native dependencies before the milestone that needs them. Record the reason and boundary when adding Servo, Wayland, async runtime, or IPC dependencies.
- Unit-test pure policy and routing logic. Isolate compositor-dependent integration so it can be covered by fixtures or integration tests.

## Workspace boundaries

Preserve these conceptual areas even if internal modules change:

- `apps/melly`: Rust executable wiring the runtime, Servo desktop host, and application-facing Wayland scene;
- `crates/core/runtime`: runtime coordination, authorization, IPC server, and lifecycle;
- `crates/core/shell`: command-shell client behavior implemented through `rusty-melly`, without rendering or desktop-surface ownership;
- `crates/core/deployment-control`: repository generations, validation, activation, rollback, and recovery;
- `crates/core/adapter-api`: compositor- and service-neutral adapter contracts;
- `crates/core/protocol`: shared versioned IPC wire primitives;
- `crates/sdk/rusty-melly`: supported Rust client SDK;
- `crates/adapters/*`: concrete external integrations such as Sway, Servo, Smithay, Git, SQLite, and Linux services.

Update the corresponding documentation whenever a public contract, manifest assumption, security rule, or milestone changes.

## Requested open-source quality review

When asked to assess open-source quality or independent-contribution readiness, use the repository skill at `.agents/skills/melly-open-source-review/SKILL.md`. It is a report-only review of names, applicable standards, test evidence, README accuracy, commit hygiene, and contributor/release quality; ordinary implementation work does not automatically trigger a full audit.
