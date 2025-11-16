#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
HOOKS_DIR="$ROOT_DIR/.githooks"

if [ ! -d "$HOOKS_DIR" ]; then
  echo "Error: hooks directory not found: $HOOKS_DIR" >&2
  exit 1
fi

# Ensure hooks are executable
chmod +x "$HOOKS_DIR"/* || true

# Point this repo to use the versioned hooks directory
git config core.hooksPath .githooks

echo "Git hooks installed. This repository will now run pre-commit formatting."
echo "- Hook path: .githooks"
echo "- Hook(s): pre-commit (runs rustfmt on staged .rs files)"
echo
echo "Tips:"
echo "- Bypass temporarily: git commit --no-verify"
echo "- Remove later: git config --unset core.hooksPath"
