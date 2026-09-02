# Implementation roadmap

The proposal estimates a 16–22 month MVP at roughly 8–12 focused hours per week. Project tracking uses milestone exit criteria.

## Foundation

- **Tooling and test lab:** reproducible Rust workflow, Ubuntu VM, Sway test session, logs, snapshots, and recovery access.
- **Contract slice:** specify the smallest versioned HTML/DOM/`melly.*` environment that every supported backend must honor, including explicit preview and unsupported-operation behavior.
- **Wayland foundations:** learn client/server/session concepts and build narrow probes for both host-facing surfaces and the application-facing proxy direction.
- **Servo feasibility:** render and interact with one Servo WebView on a shell surface, measure resource use and latency, and decide whether the integration works without a large permanent fork.
- **Compatibility routing:** classify applications as Melly-managed or host-managed, send X11 applications through Sway/XWayland from the start, and log every unsupported-case bypass without weakening authorization.

## Architectural proof

- **One native application path:** receive one native Wayland application's toplevel through the candidate Melly boundary and represent it as a host window without exposing host-specific identifiers publicly.
- **HTML-controlled frame:** render a small Servo HTML/CSS frame around or alongside the real application surface and reflect title/focus state into the DOM.
- **Input and intent:** route pointer/keyboard input correctly, make one HTML close action work, and prove an HTML drag region or equivalent semantic action where the host supports it.
- **Live source update:** reload the frame's local HTML/CSS/JavaScript without restarting the client application, host compositor, or Melly session.
- **Host-managed smoke test:** launch an X11 application outside the proxy, confirm Sway/XWayland keeps it usable, and confirm diagnostics identify that Melly features are not promised for it.
- **Architecture decision:** retain, revise, or reject the rootless proxy topology from measured correctness, compatibility, resource, and latency evidence. Dependency and topology changes below the contract must not force desktop authors into native or host-specific code.

## Interactive shell

- **Live prototype:** load HTML/CSS/JavaScript, watch source files, reload predictably, and expose one Rust-backed application operation.
- **Host abstraction:** define composable capability providers and implement the first Sway adapter without leaking Sway types upward.
- **Core JavaScript API:** establish host-neutral apps, windows, workspaces, outputs, capability, and error contracts.
- **Multiple surfaces:** support background, panel, launcher/overlay, multiple outputs, and keyboard/focus behavior.

## Safe deployment

- **Git generations:** isolate candidate checkouts, validate, health-check, atomically activate, and retain known-good generations.
- **Security and recovery:** enforce explicit permissions, add safe mode, emergency rollback, and crash-loop protection.
- **Remote workflow:** clone/install desktop repositories and validate updates and merged revisions as normal Git history.

## Release hardening

- **Alternative login session:** make clean VM installation, login/logout, supervision, recovery, and uninstall repeatable.
- **Second backend:** prove portability with Labwc or a materially different generic Wayland path, then profile and document the result.

## Go/no-go gates

1. Servo must render interactively on the required shell surface without a large permanent fork.
2. The prototype must prove a functional managed native application/surface path with HTML-controlled interface behavior before broad managed-window APIs or full-session work expands.
3. Public Melly contracts must contain no Sway-specific, raw Wayland, or backend wire identifiers after the first adapter/proxy implementation.
4. Every exposed operation must have a testable guarantee, capability/permission story, and explicit unavailable behavior.
5. A bad committed revision must not replace the known-good desktop.
6. Clean VM session installation and external recovery must be repeatable.
7. A second backend must work without restructuring the public contract, even if internal adapters differ.
8. X11 and every bypassed case must remain host-usable where the reference host supports it, with an observable reason that Melly management was declined.
