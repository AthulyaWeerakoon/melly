# Rust workspace

The repository is a Cargo workspace organized by runtime responsibility. Directory nesting is for source navigation; each package has an explicit Cargo package name.

```text
melly/
├── apps/
│   └── melly/                    # Rust runtime and desktop-host executable
└── crates/
    ├── core/
    │   ├── adapter-api/          # melly-adapter-api
    │   ├── deployment-control/   # melly-deployment-control
    │   ├── protocol/             # melly-protocol
    │   ├── runtime/              # melly-runtime
    │   └── shell/                # melly-shell command-client scaffold
    ├── sdk/
    │   └── rusty-melly/          # rusty-melly
    └── adapters/
        ├── git/                  # melly-adapter-git
        ├── linux/                # melly-adapter-linux
        ├── servo/                # melly-adapter-servo
        ├── smithay/              # melly-adapter-smithay
        ├── sqlite/               # melly-adapter-sqlite
        └── sway/                 # melly-adapter-sway
```

## Executable

`apps/melly` contains the Rust executable that assembles the runtime, Servo desktop host, application-facing Wayland scene, deployment controller, and configured adapters. The desktop host is not the Melly command shell. `crates/core/shell/` is the Rust Cargo package `melly-shell`, a command-client scaffold.

## Core crates

- `melly-runtime` owns sessions, the IPC server, JavaScript bridge dispatch, authorization, runtime state, storage policy, configuration, and diagnostics.
- `melly-shell` owns command-shell client behavior through `rusty-melly`. It does not own client-surface lifecycle, desktop resources, rendering, input, outputs, or development reload; those belong to the desktop host, scene, and runtime boundaries.
- `melly-deployment-control` owns repository identities, revisions, isolated generations, validation, transactional activation, rollback, and recovery coordination.
- `melly-adapter-api` owns Melly-defined contracts and canonical models for host, rendering, Wayland, revision, persistence, and system-service adapters.
- `melly-protocol` owns versioned transport-neutral wire primitives shared by the runtime and SDKs. It does not own authorization policy or client convenience APIs.

## Client SDK

`rusty-melly` is the supported Rust client library for the Melly runtime. The command shell uses it, and third-party native applications may use the same library without routing requests through the command-shell process. At present it only resolves and opens the Unix socket; it cannot issue commands.

Direct SDK access does not bypass the runtime. The future negotiated protocol subjects clients to runtime identity, capability, permission, and policy checks. These checks are not implemented yet. `rusty-melly` does not expose Sway commands, raw Wayland objects, runtime internals, or privileged command-shell operations.

The SDK is an internal scaffold until the protocol, compatibility guarantees, and publishing metadata are versioned. It can be used as a workspace, path, or Git dependency during development.

## Adapters

Concrete adapter crates translate between Melly-owned contracts and external implementations. External identifiers, messages, errors, and native types remain inside their adapter crate.

Native dependency graphs are feature-gated while their integrations are prototypes:

- `servo-engine` enables the Servo adapter;
- `wayland-server` enables Smithay application-facing server facilities;
- `host-sway` enables the Sway IPC adapter and Smithay Client Toolkit facilities.

## Dependency direction

```text
Native Wayland apps -- Smithay server --+
                                        |
Desktop HTML/CSS/JS ----- Servo --------+-- Melly scene -- display host
                          |             |
                          +-- melly.* / desktop.* / system.* --+-- melly-runtime

Melly command shell -------+
Third-party native apps ---+-- rusty-melly -- Unix socket -- melly-runtime

melly-runtime -- deployment control
              -- adapter API -- concrete adapters
```

The command shell depends on the public client SDK rather than runtime internals. The desktop host presents the web interface through the Servo adapter and supplies the authorized bridge. The runtime and SDK share protocol primitives through `melly-protocol`. Concrete adapters depend inward on Melly-owned contracts; public contracts do not depend on adapter-native types.
