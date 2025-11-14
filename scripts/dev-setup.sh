#!/usr/bin/env bash
set -euo pipefail

echo "==> Checking for rustup..."
if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup not found. Installing via official script..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
else
  echo "rustup is already installed."
fi

echo "==> Ensuring toolchain per rust-toolchain.toml is installed (stable)"
rustup show >/dev/null

echo "==> Adding essential components (rust-src, rustfmt, clippy)"
rustup component add rust-src rustfmt clippy --toolchain stable || true

echo "==> rustc version: $(rustc --version)"
echo "==> cargo version: $(cargo --version)"
echo "Setup complete. If your IDE was showing 'stdlib and rustup not found', restart the IDE and reopen the project."
