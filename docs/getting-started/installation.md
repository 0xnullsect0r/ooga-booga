# Installation

This page covers everything you need to build `oogac` — the Ooga Booga compiler — and run `.ooga` programs.

---

## Prerequisites

### Rust toolchain

Ooga Booga's compiler is written in Rust. Install the latest stable Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify:

```bash
rustc --version   # should print 1.70 or later
cargo --version
```

### Node.js

The compiler transpiles `.ooga` files to JavaScript. You need [Node.js](https://nodejs.org/) to *run* the generated output.

```bash
node --version   # should print v18 or later
```

Node.js is **not** required to compile — only to execute the output.

### Optional: readline-sync (for HEAR)

If your program uses `HEAR` (reading from stdin), the generated JavaScript requires the `readline-sync` package:

```bash
npm install readline-sync
```

---

## Building the compiler

Clone the repository and compile with Cargo:

```bash
git clone https://github.com/0xnullsect0r/ooga-booga.git
cd ooga-booga
cargo build --release
```

The compiled binary will be at:

```
target/release/oogac
```

Add it to your `PATH` for convenient use:

```bash
# Linux / macOS
export PATH="$PWD/target/release:$PATH"

# Or copy to a system directory
sudo cp target/release/oogac /usr/local/bin/
```

---

## Verifying the installation

Check that the compiler works:

```bash
oogac --help
```

Expected output:

```
UGH! OOGAC — THE OOGA BOOGA CAVE COMPILER

Usage: oogac <COMMAND>

Commands:
  compile  Transpile a .ooga source file to JavaScript
  run      Transpile and immediately run a .ooga file with node
  check    Check a .ooga file for errors without emitting output
  help     Print this message or the help of the given subcommand(s)
```

Run one of the included examples:

```bash
oogac run examples/hello_world.ooga
```

Expected output:

```
Hello, World! UGH!
Me am:
Thog
Greetings from cave creature: Thog
```

---

## Running tests

```bash
cargo test
```

All 41 unit and integration tests should pass.
