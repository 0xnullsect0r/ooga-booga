#!/usr/bin/env bash
# install-ooga.sh — Install the Ooga Booga compiler and build tool.
#
# Usage:
#   ./install-ooga.sh              # installs to /usr/local/bin (may need sudo)
#   ./install-ooga.sh --user       # installs to ~/.local/bin (no sudo)
#   ./install-ooga.sh --prefix /some/dir
#
# Requirements: Rust toolchain (https://rustup.rs)

set -euo pipefail

# ── Defaults ────────────────────────────────────────────────────────────────────
PREFIX="/usr/local"
USER_INSTALL=false

# ── Argument parsing ────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --user)
            USER_INSTALL=true
            PREFIX="$HOME/.local"
            shift
            ;;
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [--user] [--prefix DIR]"
            echo ""
            echo "  --user          Install to ~/.local/bin (no sudo required)"
            echo "  --prefix DIR    Install to DIR/bin"
            echo ""
            echo "Default: install to /usr/local/bin (may require sudo)"
            exit 0
            ;;
        *)
            echo "BONK! Unknown argument: $1"
            echo "Run '$0 --help' for usage."
            exit 1
            ;;
    esac
done

BIN_DIR="$PREFIX/bin"

# ── Banner ───────────────────────────────────────────────────────────────────────
echo ""
echo "  ╔═══════════════════════════════════════╗"
echo "  ║   OOGA BOOGA INSTALLER  v0.2.0        ║"
echo "  ║   Cave creature install big compiler  ║"
echo "  ╚═══════════════════════════════════════╝"
echo ""

# ── Check prerequisites ──────────────────────────────────────────────────────────
if ! command -v cargo &>/dev/null; then
    echo "BONK! CAVE NEED RUST TOOLCHAIN!"
    echo ""
    echo "Install Rust by running:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "Then run this installer again."
    exit 1
fi

CARGO_VERSION=$(cargo --version)
echo "✓ Found: $CARGO_VERSION"

if ! command -v rustc &>/dev/null; then
    echo "BONK! rustc NOT FOUND. Cave need full Rust toolchain."
    exit 1
fi

RUSTC_VERSION=$(rustc --version)
echo "✓ Found: $RUSTC_VERSION"

# ── Build ────────────────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo ""
echo "UGH! CAVE BUILD COMPILER (this take few moments)..."
echo ""

cd "$SCRIPT_DIR"
cargo build --release --quiet

echo ""
echo "UGGA! BUILD DONE!"

# ── Install ──────────────────────────────────────────────────────────────────────
mkdir -p "$BIN_DIR"

for BINARY in ooga oogac; do
    SRC="$SCRIPT_DIR/target/release/$BINARY"
    DST="$BIN_DIR/$BINARY"
    if [[ ! -f "$SRC" ]]; then
        echo "BONK! Binary not found: $SRC"
        exit 1
    fi
    if [[ "$USER_INSTALL" == "false" && ! -w "$BIN_DIR" ]]; then
        echo "Need write access to $BIN_DIR — trying with sudo..."
        sudo install -m 755 "$SRC" "$DST"
    else
        install -m 755 "$SRC" "$DST"
    fi
    echo "✓ Installed: $DST"
done

# ── PATH reminder ────────────────────────────────────────────────────────────────
echo ""
if [[ "$USER_INSTALL" == "true" ]]; then
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        echo "⚠  $BIN_DIR is not in your PATH."
        echo "   Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "     export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
    fi
fi

# ── Done ─────────────────────────────────────────────────────────────────────────
echo "╔══════════════════════════════════════════════════════╗"
echo "║  OOGA BOOGA INSTALLED! CAVE VERY HAPPY.              ║"
echo "║                                                      ║"
echo "║  Quick start:                                        ║"
echo "║    ooga new my-cave-app                              ║"
echo "║    cd my-cave-app                                    ║"
echo "║    ooga run                                          ║"
echo "║                                                      ║"
echo "║  Transpile only:                                     ║"
echo "║    oogac compile program.ooga -o program.rs          ║"
echo "╚══════════════════════════════════════════════════════╝"
