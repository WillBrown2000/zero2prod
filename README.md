Zero2Prod – Project Overview and CI Integration

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
