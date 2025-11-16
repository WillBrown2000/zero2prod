Zero2Prod – Project Overview, Linting and CI Integration

This repository is configured with a GitHub Actions workflow (.github/workflows/ci.yml) that builds, lints, runs migrations, and tests your project on every push and pull request.

See online CI builds and failures inside your IDE (RustRover)

JetBrains IDEs (including RustRover) have built-in integration for GitHub and GitHub Actions. Once connected, you can view all workflow runs, inspect logs, and drill into failures directly in the IDE without leaving your local environment.

Quick setup

1) Connect your GitHub account
   - RustRover: Settings/Preferences → Version Control → GitHub → Log In.
   - Choose either your web browser or a token to authenticate.

2) Open the GitHub Actions tool window
   - View → Tool Windows → GitHub → Actions.
   - You should see workflow runs for this repository (name: CI).

3) Inspect failures
   - Select a run → select a job (build-test-lint) → open the failing step to see full logs and error messages.
   - You can re-run jobs from the IDE if your permissions allow it.

Alternative: use GitHub CLI from your terminal

If you prefer the terminal or don’t want to connect the IDE, you can stream CI logs locally using the GitHub CLI (gh). A helper script is included:

scripts/watch_ci.sh

Usage:
  - ./scripts/watch_ci.sh                          # watch the latest run for the default branch
  - ./scripts/watch_ci.sh --run <run-id>           # watch a specific run
  - ./scripts/watch_ci.sh --branch <branch-name>   # watch latest run for a branch

The script requires:
  - gh installed: https://cli.github.com/
  - You are authenticated: gh auth login
  - Your git remote “origin” points to the correct GitHub repo

Optional: add a CI status badge to GitHub README

If this repository is public (or you’re fine with badges), add the following snippet to your GitHub README to show the latest workflow status. Edit <OWNER> and <REPO> to match your GitHub namespace:

![CI](https://github.com/<OWNER>/<REPO>/actions/workflows/ci.yml/badge.svg)

Troubleshooting

- The Actions tool window doesn’t show runs
  - Ensure you opened the same repository that is hosted on GitHub (check VCS → Git Remotes for “origin”).
  - Ensure you are logged into GitHub in the IDE.
  - Ensure the workflow file is on the current branch and has been pushed.

- Permissions errors when re-running jobs from IDE
  - You might need write permissions on the repository or specific workflow permissions.

- Database failures in CI
  - This workflow starts a Postgres service in CI and runs migrations with sqlx.
  - Locally, ensure your .env/configuration.yaml settings match what tests expect.


Local linting and auto-fix (Rustfmt + Clippy)

You can run lint checks and automatically fix most issues found by Clippy and rustfmt.

- Check only (no changes):
  - cargo fmt --all -- --check
  - cargo clippy --all-targets --all-features -D warnings

- Auto-fix (apply safe fixes):
  - cargo fix --allow-dirty --allow-staged
  - cargo fmt --all
  - Or use aliases: cargo fmt-all or cargo fmt-fix

Convenient script

Use the helper script to run both steps together:

  - ./scripts/lint.sh check   # verify formatting and clippy lints (fails on warnings)
  - ./scripts/lint.sh fix     # apply clippy fixes and format the code

FAQ: “How do I make cargo fmt --all -- --check auto-fix?”

- The -- --check flag tells rustfmt to only verify formatting and exit with a non-zero status if changes are needed. To auto-fix formatting, drop the check flag:
  - cargo fmt --all
- Equivalent shortcuts:
  - cargo fmt-all
  - cargo fmt-fix
- Running ./scripts/lint.sh fix will also format the whole workspace after applying lint fixes.

RustRover IDE

- Rustfmt on save: Preferences/Settings → Languages & Frameworks → Rust → Rustfmt → Format on Save.
- Run Clippy: Tools → Rust → Run Clippy (or use the Clippy inspection in the Problems tool window). Many quick-fixes are available via Alt+Enter.

Notes about cargo aliases and a common error

- This repo defines Cargo command aliases in .cargo/config.toml (not in Cargo.toml). If you previously saw a warning like:
  - "warning: unused manifest key: alias" — that is fixed now by moving aliases to the right file.

- To run lints, use the aliases directly, e.g.:
  - cargo lint
  - cargo fix-lint && cargo fmt-all
  Do NOT run "cargo run lint" — that would execute the application binary and pass the word "lint" as an argument to the server.

- If you see: Error: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
  - It means the configured server port is already taken (or you accidentally started the server instead of running lints).
  - Close the running server or choose a free port in configuration.yaml, or simply run the correct lint commands above.

Clippy auto-fix notes

- On stable toolchain, `cargo fix-lint` applies compiler suggestion fixes only. To also apply Clippy fixes automatically, use one of the following:
  - Recommended: `./scripts/lint.sh fix` — detects support and applies Clippy fixes when available, then formats.
  - If you have nightly installed: `cargo clippy-fix-nightly && cargo fmt-all`.

Troubleshooting: "unexpected argument '--clippy'"

- If you previously saw `error: unexpected argument '--clippy' found` when running `cargo fix-lint`, that was because the alias used `cargo fix --clippy`, which is not supported on your Cargo version. The alias has been updated to a stable-safe form; re-run:
  - `cargo fix-lint && cargo fmt-all` (stable)
  - Or use the script/nightly options above for Clippy auto-fixes.
