# Development guide

## Prerequisites

- stable Rust through `rustup`;
- the `rustfmt` and Clippy components;
- Git;
- native Wayland and Servo prerequisites only when their implementation milestones begin.

Follow current upstream Servo setup documentation when the embedding spike starts. Do not copy an old system-package list into this repository without validating it against the supported Ubuntu and Servo versions.

## Local checks

Run the same baseline checks before proposing a change:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- --help
```

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
