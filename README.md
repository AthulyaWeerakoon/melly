# Melly

**A desktop you can shape with web technologies.** Melly is an early Rust project exploring a programmable Linux desktop whose interface is written in HTML, CSS, and JavaScript.

## Where things stand

Melly is currently a CLI/workspace scaffold. The help screen runs, but there is no usable desktop runtime or Servo host yet. You can build and inspect the scaffold; you cannot install or apply it as a desktop. The companion [Melly Desktop](https://github.com/AthulyaWeerakoon/melly-desktop) repository has a browser-previewable interface, but it is not connected to this runtime. Rust license selection is still pending.

## Build and try the CLI

Install stable Rust with `rustup`, then from this repository run:

```sh
cargo run -- --help
```

That is currently the practical way to run Melly: it displays the scaffold's CLI help. There is not yet a desktop to launch or apply. For the Rust workspace checks, see the [development guide](docs/development.md).

## Contribute

Start with the [development guide](docs/development.md) for setup and checks, and the [workspace map](docs/workspace.md) for project orientation. There is not yet a separate contribution workflow; please keep changes focused and include relevant tests. No license has been selected, so contribution and distribution terms are not yet established.
