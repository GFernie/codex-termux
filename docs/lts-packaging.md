# LTS Packaging

The LTS npm package is multi-platform, but not every artifact is produced on the same machine.

## Final npm target set

- `android-arm64`
- `linux-x64`
- `linux-arm64`
- `darwin-x64`
- `darwin-arm64`

## Build source by platform

- `android-arm64`: built locally from this repo
- `linux-x64`: built locally from this repo
- `linux-arm64`: built locally from this repo
- `darwin-x64`: built by GitHub Actions and then folded into the final npm package
- `darwin-arm64`: built by GitHub Actions and then folded into the final npm package

## Packaging rules

- do not publish final LTS npm without both macOS artifacts
- do not substitute a locally cross-named artifact for the GitHub Actions macOS builds
- keep LTS package contents aligned with the actual `npm-package/bin/<target>/` layout

## Homebrew

Homebrew can exist as an additional distribution channel for LTS, but it is not the canonical source of truth for the npm package contents.

## Validation before publish

- run LTS Core suite on the required provider matrix
- collect human reports under `test-reports/lts/`
- verify updater behavior stays on LTS and does not point users at latest/non-LTS versions
