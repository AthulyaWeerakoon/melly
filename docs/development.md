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

The application-facing server/proxy boundary uses Smithay 0.7.0 through the `wayland-proxy` feature. Only Smithay's `desktop` and `wayland_frontend` features are enabled. Direct DRM, GBM, libinput, libseat, session, X11 backend, and XWayland features remain disabled because the host compositor owns those facilities.

The Sway host boundary uses Smithay Client Toolkit 0.21.1 and swayipc 4.0.0 through the `host-sway` feature. Client Toolkit supplies ordinary Wayland client and layer-shell facilities. swayipc is confined to the Sway adapter and does not enter public Melly contracts.

On Ubuntu, the current `host-sway` dependency slice requires pkg-config and the XKB Common development files:

```sh
sudo apt install pkg-config libxkbcommon-dev
```

## Local checks

Run the same baseline checks before proposing a change:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -- --help
```

The native-boundary checks are:

```sh
cargo check --locked --features servo-engine
cargo check --locked --features wayland-proxy,host-sway
cargo clippy --locked --all-targets --all-features -- -D warnings
```

Run these checks in a development environment with the native prerequisites and sufficient build storage.

Use `RUST_BACKTRACE=1` while diagnosing runtime failures. Add structured logging before the first graphical prototype so engine, host-adapter, deployment, and JavaScript failures can be distinguished.

## Safe graphics workflow

Develop session-level integration in a KVM/QEMU Ubuntu VM with snapshots. Keep a known working desktop session and TTY access available. Test Sway independently before adding Melly, and do not install display-manager session files on a primary machine until VM login, logout, failure, and uninstall paths are repeatable.

Suggested snapshot points are:

1. clean tooling and Sway setup;
2. first layer-shell surface;
3. first interactive Servo surface;
4. first native JavaScript operation;
5. first Melly login session;
6. first transactional Git activation.

## Change discipline

- Keep learning spikes small and write down results, especially go/no-go evidence.
- Measure startup time, steady memory, idle CPU wakeups, and input/frame latency during the Servo spike.
- Add fixtures for host messages rather than requiring a live compositor for every test.
- Update docs when a boundary or safety invariant changes.
