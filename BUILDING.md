# Building codex-termux

This fork maintains two separate release lines:

- `main`: latest Termux-focused line
- `lts`: stability line based on upstream `0.80.x`

See [`docs/release-lines.md`](./docs/release-lines.md) before building or packaging.

## Workspace Layout

- Rust workspace: `codex-rs/`
- npm wrappers: `npm-package/`

## Build LTS Locally

From the repo root:

```bash
cd codex-rs
cargo build --release --bin codex --bin codex-exec --bin codex-linux-sandbox
```

Primary local outputs:

- `codex-rs/target/release/codex`
- `codex-rs/target/release/codex-exec`
- `codex-rs/target/release/codex-linux-sandbox`

## LTS Packaging Matrix

- `android-arm64`: built locally from this repo
- `linux-x64`: built locally from this repo
- `darwin-arm64`: built by GitHub Actions for the final npm package

The macOS arm64 artifact is not built on Linux for release packaging. It must come from the dedicated workflow.

More detail:

- [`docs/lts-packaging.md`](./docs/lts-packaging.md)

## Local npm Package Assembly

After building Rust release binaries, populate `npm-package/bin/<target>/` with the correct artifacts for the release line you are packaging.

For LTS, the final npm package is valid only when these targets are present:

- `android-arm64`
- `linux-x64`
- `darwin-arm64`

## Recommended Validation Before Publish

Run the LTS suites:

- Core: [`test-reports/suites/lts/core/linux.md`](./test-reports/suites/lts/core/linux.md)
- Extended: [`test-reports/suites/lts/extended/linux.md`](./test-reports/suites/lts/extended/linux.md)
- Platform guide: [`docs/lts-testing.md`](./docs/lts-testing.md)

## Publish Rules

- do not publish LTS npm until Core validation passes on required platforms/providers
- do not cut GitHub release until required human test reports are stored under `test-reports/lts/`
- do not treat Homebrew as the source of truth for npm packaging
