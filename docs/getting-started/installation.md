# Installation

This page covers everything you need to install the Ooga Booga toolchain and run `.ooga` programs.

---

## Prerequisites

### Rust toolchain

Ooga Booga compiles `.ooga` programs to Rust, then uses `cargo` / `rustc` to produce a native binary. You need the Rust toolchain installed.

Install via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify:

```bash
rustc --version   # 1.70 or later
cargo --version
```

That is the only prerequisite. No Node.js, no npm, no extra runtimes.

---

## Option 1 — Install script (recommended)

Clone the repository and run the install script:

```bash
git clone https://github.com/0xnullsect0r/ooga-booga.git
cd ooga-booga
./install-ooga.sh
```

This builds `ooga` and `oogac` in release mode and installs them to `/usr/local/bin/`. If you don't have write access there, use the `--user` flag:

```bash
./install-ooga.sh --user      # installs to ~/.local/bin/
```

Or specify a custom prefix:

```bash
./install-ooga.sh --prefix /opt/cave-tools
```

After a `--user` install, add `~/.local/bin` to your PATH if it isn't already:

```bash
export PATH="$HOME/.local/bin:$PATH"   # add to ~/.bashrc or ~/.zshrc
```

---

## Option 2 — Manual build

```bash
git clone https://github.com/0xnullsect0r/ooga-booga.git
cd ooga-booga
cargo build --release
```

The binaries are at:

```
target/release/ooga     # build tool (like cargo)
target/release/oogac    # low-level transpiler
```

Add them to your PATH:

```bash
export PATH="$PWD/target/release:$PATH"
```

---

## Verifying the installation

```bash
ooga --help
```

Expected output:

```
UGH! OOGA — CAVE BUILD TOOL FOR OOGA BOOGA PROGRAMS

Usage: ooga <COMMAND>

Commands:
  new    Create a new Ooga Booga project
  build  Build the project (transpile + compile)
  run    Build and run the project
  check  Check the project for errors without building
  clean  Remove build artifacts
  test   Run tests
  help   Print this message or the help of the given subcommand(s)
```

---

## Quick smoke test

```bash
ooga new hello-cave
cd hello-cave
ooga run
```

Expected output:

```
Ooga Booga! Cave greet world!
```

---

## Running tests

From the repository root:

```bash
cargo test
```

All unit and integration tests should pass. Integration tests compile and run example programs with `rustc`.
