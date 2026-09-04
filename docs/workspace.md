# Rust workspace

The repository is a Cargo workspace organized by runtime responsibility. Directory nesting is for source navigation; each package has an explicit Cargo package name.

```text
melly/
├── apps/
│   └── melly/                    # executable: melly
└── crates/
    ├── core/
    │   ├── adapter-api/          # melly-adapter-api
    │   ├── deployment-control/   # melly-deployment-control
    │   ├── protocol/             # melly-protocol
    │   ├── runtime/              # melly-runtime
    │   └── shell/                # melly-shell
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

`apps/melly` contains the process entry point. It assembles the runtime, deployment controller, and configured adapters and owns process startup and shutdown. Domain behavior remains in the corresponding crates.

## Core crates

- `melly-runtime` owns sessions, the IPC server, JavaScript bridge dispatch, authorization, runtime state, storage policy, configuration, and diagnostics.
- `melly-shell` owns reference shell behavior, surface lifecycle, desktop manifests, component/resource loading, rendering coordination, input, outputs, and development reload. It communicates with the runtime through `rusty-melly`.
- `melly-deployment-control` owns repository identities, revisions, isolated generations, validation, transactional activation, rollback, and recovery coordination.
- `melly-adapter-api` owns Melly-defined contracts and canonical models for host, rendering, Wayland, revision, persistence, and system-service adapters.
- `melly-protocol` owns versioned transport-neutral wire primitives shared by the runtime and SDKs. It does not own authorization policy or client convenience APIs.

## Client SDK

`rusty-melly` is the supported Rust client library for the Melly runtime. The reference shell uses it, and third-party native applications may use the same library to integrate directly with Melly without routing requests through the shell process.

Direct SDK access does not bypass the runtime. All clients use the same negotiated protocol and remain subject to runtime identity, capability, permission, and policy checks. `rusty-melly` does not expose Sway commands, raw Wayland objects, runtime internals, or privileged shell operations.

The SDK is an internal scaffold until the protocol, compatibility guarantees, and publishing metadata are versioned. It can be used as a workspace, path, or Git dependency during development.

## Adapters

Concrete adapter crates translate between Melly-owned contracts and external implementations. External identifiers, messages, errors, and native types remain inside their adapter crate.

Native dependency graphs are feature-gated while their integrations are prototypes:

- `servo-engine` enables the Servo adapter;
- `wayland-proxy` enables Smithay server/proxy facilities;
- `host-sway` enables the Sway IPC adapter and Smithay Client Toolkit facilities.

## Dependency direction

```text
Melly desktop HTML/CSS/JS
        |
        | melly.* bridge
        v
melly-shell ---- rusty-melly <---- third-party native app
                         |
                         | Unix socket + versioned protocol
                         v
                    melly-runtime
                      /       \
       deployment control     adapter API
                                  |
                            concrete adapters
```

The shell depends on the public client SDK rather than runtime internals. The runtime and SDK share protocol primitives through `melly-protocol`. Concrete adapters depend inward on Melly-owned contracts; public contracts do not depend on adapter-native types.
