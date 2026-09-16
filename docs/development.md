# Development guide

## Prerequisites

- stable Rust through `rustup`;
- the `rustfmt`, Clippy, rust-src, and rust-analyzer components declared in `rust-toolchain.toml`;
- Git;
- native Wayland and Servo prerequisites only when their implementation milestones begin.

Entering this repository lets `rustup` select the declared stable toolchain and components. Install them explicitly when bootstrapping a machine:

```sh
rustup toolchain install stable --profile minimal \
  --component rustfmt,clippy,rust-src,rust-analyzer
```

## Servo dependency model

Servo is an external library dependency and is not copied into this repository. The current embedding baseline declares `servo` 0.5.0 in `Cargo.toml` and commits the exact resolved dependency graph in `Cargo.lock`. The dependency remains optional behind `servo-engine` until the first WebView adapter exists. Cargo stores downloaded dependency source and build caches outside the tracked source tree; `/target/` remains ignored. Servo upgrades are explicit compatibility changes and must update the engine adapter and lockfile together.

An upstream Servo source checkout is needed only for investigation or upstream contribution. Keep that checkout beside this repository, not inside it, and use Servo's own `rust-toolchain.toml` and `./mach bootstrap` workflow. Follow current upstream Servo setup documentation when the embedding spike starts. Validate native package requirements against the supported Ubuntu and Servo versions before recording them here.

## Wayland dependency model

The application-facing Wayland server boundary uses Smithay 0.7.0 through the `wayland-server` feature. Only Smithay's `desktop` and `wayland_frontend` features are enabled. Direct DRM, GBM, libinput, libseat, session, X11 backend, and XWayland features remain disabled. The display host owns hardware facilities; X11 is not supported.

The Sway host boundary uses Smithay Client Toolkit 0.21.1 and swayipc 4.0.0 through the `host-sway` feature. Client Toolkit supplies ordinary Wayland client and layer-shell facilities. swayipc is confined to the Sway adapter and does not enter public Melly contracts.

On Ubuntu, the current `host-sway` dependency slice requires pkg-config and the XKB Common development files:

```sh
sudo apt install pkg-config libxkbcommon-dev
```

## Local checks

Run the same baseline checks before proposing a change:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run -- --help
```

The native-boundary checks are:

```sh
cargo check --locked --features servo-engine
cargo check --locked --features wayland-server,host-sway
cargo clippy --locked --all-targets --all-features -- -D warnings
```

Run these checks in a development environment with the native prerequisites and sufficient build storage.

Use `RUST_BACKTRACE=1` while diagnosing runtime failures. Add structured logging before the first graphical prototype so engine, host-adapter, deployment, and JavaScript failures can be distinguished.

## Runtime client development

The command shell and native application clients use `rusty-melly`. The default endpoint resolves to `$XDG_RUNTIME_DIR/melly/runtime.sock`; do not create sockets in this checkout, `/tmp`, persistent state directories, or shared storage. The current SDK opens a socket only; no command protocol or server exists yet.

Keep protocol work split across these ownership boundaries:

- `melly-protocol` for shared versioned wire primitives;
- `rusty-melly` for client connection, framing, requests, events, and reconnect behavior;
- `melly-runtime` for the server, peer credentials, identity, authorization, policy, routing, and diagnostics;
- `melly-shell` for command-shell behavior through the public SDK, without desktop rendering ownership.

Tests may supply an explicit socket path inside a private temporary directory. Production code uses the standard per-login endpoint. Do not make an explicit override a permission bypass.

## Safe graphics workflow

Develop session-level integration in a KVM/QEMU Ubuntu VM with snapshots. Keep a known working desktop session and TTY access available. Test Sway independently before adding Melly, and do not install display-manager session files on a primary machine until VM login, logout, failure, and uninstall paths are repeatable.

The production desktop host is Rust code using Servo. Do not add Python, WebKitGTK, browser-command, or unrelated renderer implementations as committed examples or fallbacks. A test session may supervise the built Melly executable only after independent terminal, TTY, SSH, and alternate-login recovery are verified.

## Change discipline

- Keep implementation probes within the intended Rust engine, desktop-host, and runtime boundaries so their tests and contracts can migrate into production. Keep disposable local trials outside the Git repository.
- Measure startup time, steady memory, idle CPU wakeups, and input/frame latency during the Servo spike.
- Exercise the Servo adapter with the engine's available GPU path and a software fallback; do not treat performance of a software-only VM as representative of accelerated desktop rendering.
- Add fixtures for host messages rather than requiring a live compositor for every test.
- Update docs when a boundary or safety invariant changes.
