#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Watch GitHub Actions CI logs for this repository from your terminal.

Usage:
  watch_ci.sh [--branch <branch>] [--run <run-id>] [--workflow <file>]

Options:
  --branch <branch>    Watch the latest run for the given branch (default: current branch)
  --run <run-id>       Watch a specific run id (overrides --branch)
  --workflow <file>    Workflow file name to filter (default: ci.yml)
  -h, --help           Show this help message

Requirements:
  - GitHub CLI installed: https://cli.github.com/
  - Authenticated with: gh auth login
  - This repo's origin is a GitHub remote
USAGE
}

if [[ ${1:-} == "-h" || ${1:-} == "--help" ]]; then
  usage; exit 0
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: GitHub CLI (gh) is not installed. See https://cli.github.com/" >&2
  exit 1
fi

# Default values
BRANCH=""
RUN_ID=""
WORKFLOW="ci.yml"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --branch)
      BRANCH="$2"; shift 2 ;;
    --run)
      RUN_ID="$2"; shift 2 ;;
    --workflow)
      WORKFLOW="$2"; shift 2 ;;
    *)
      echo "Unknown argument: $1" >&2; usage; exit 2 ;;
  esac
done

# Ensure gh is authenticated
if ! gh auth status >/dev/null 2>&1; then
  echo "You are not authenticated to GitHub CLI. Run: gh auth login" >&2
  exit 1
fi

# Determine default branch if none provided
if [[ -z "$BRANCH" && -z "$RUN_ID" ]]; then
  if command -v git >/dev/null 2>&1; then
    BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
  fi
fi

# If RUN_ID provided, watch that run
if [[ -n "$RUN_ID" ]]; then
  echo "Watching run $RUN_ID (workflow: $WORKFLOW)"
  exec gh run watch --exit-status --job "*" --verbose --repo "$(gh repo view --json nameWithOwner -q .nameWithOwner)" "$RUN_ID"
fi

# Otherwise, pick the latest run for branch & workflow
echo "Selecting latest run for workflow '$WORKFLOW'${BRANCH:+ on branch '$BRANCH'}"

QUERY=(gh run list --workflow "$WORKFLOW" --limit 1 --json databaseId)
if [[ -n "$BRANCH" ]]; then
  QUERY+=(--branch "$BRANCH")
fi

RUN_ID=$("${QUERY[@]}" -q '.[0].databaseId')

if [[ -z "$RUN_ID" || "$RUN_ID" == "null" ]]; then
  echo "No runs found for workflow '$WORKFLOW'${BRANCH:+ on branch '$BRANCH'}" >&2
  exit 3
fi

echo "Watching run $RUN_ID ..."
exec gh run watch --exit-status --job "*" --verbose "$RUN_ID"
