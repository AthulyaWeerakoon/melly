# Implementation roadmap

The proposal estimates a 16–22 month MVP at roughly 8–12 focused hours per week. This repository uses milestone exit criteria rather than calendar promises.

## Foundation

- **Tooling and test lab:** reproducible Rust workflow, Ubuntu VM, Sway test session, logs, snapshots, and recovery access.
- **Wayland foundations:** learn client/session concepts and build a minimal non-Servo layer-shell client.
- **Servo feasibility:** render and interact with one Servo WebView on a shell surface, measure resource use and latency, and decide whether the integration works without a large permanent fork.

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
2. Public Melly contracts must contain no Sway-specific identifiers after the first host adapter.
3. A bad committed revision must not replace the known-good desktop.
4. Clean VM session installation and external recovery must be repeatable.
5. A second backend must work without restructuring the core runtime.
