#!/usr/bin/env bash
set -euo pipefail

cmd=${1:-check}

run_check() {
  echo "Running rustfmt check..."
  cargo fmt --all -- --check
  echo "Running clippy with -D warnings..."
  cargo clippy --all-targets --all-features -D warnings
}

run_fix() {
  echo "Applying clippy fixes (stable when possible)..."
  if cargo fix --help | grep -q -- "--clippy"; then
    cargo fix --allow-dirty --allow-staged --clippy --all-targets --all-features || true
  else
    echo "cargo fix --clippy not supported by this cargo; trying cargo clippy --fix (nightly)"
    if rustup run nightly cargo -V >/dev/null 2>&1; then
      RUSTC_WRAPPER= rustup run nightly cargo clippy --fix -Z unstable-options --allow-dirty --allow-staged --all-targets --all-features || true
    else
      echo "Nightly toolchain not available; skipping clippy auto-fix."
    fi
  fi
  echo "Running rustfmt..."
  cargo fmt --all
}

case "$cmd" in
  check)
    run_check
    ;;
  fix)
    run_fix
    ;;
  *)
    echo "Usage: $0 [check|fix]"
    exit 2
    ;;
esac
