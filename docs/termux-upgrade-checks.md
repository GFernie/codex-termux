# Termux Upgrade Checks

This fork reuses the desktop Codex update infrastructure, but a few Termux
quirks can prevent the update banner from appearing. Use this page to debug and
fix upgrade detection issues without reading through the TUI source every time.

## Symptoms

- No `Update available` cell appears in history even though a newer release
  exists on GitHub.
- The file `~/.config/codex/version.json` is never created or updated after
  starting the CLI in `--release` mode.

## Root cause

`codex-rs/tui/src/updates.rs` calls the GitHub API and then extracts the
version number with `extract_version_from_latest_tag`. In older builds, this
helper only accepted tags starting with `rust-v`, while this fork publishes
tags like `v0.58.4-termux`. If the tag format is not recognized, the parser
fails and `version.json` is not written, so no upgrade banner is shown.

Also, the update module is compiled only in `--release` builds, so `cargo run`
without `--release` will never show the update banner, even if the parser is
correct.

## Fix strategy

Current releases (>= 0.58.4-termux) already use a more permissive parser that:

- accepts both upstream tags like `rust-v0.58.0` and fork tags like
  `v0.58.4-termux`, and
- strips the `-termux` suffix before comparing versions.

If you are debugging an older build, you have two options:

1. **Align GitHub tags** – publish (or rename) releases with a `rust-v`
   prefix (for example `rust-v0.58.4-termux`), so they match the original
   upstream parser.
2. **Support multiple tag formats** – update `extract_version_from_latest_tag`
   to accept both `rust-v*` and `v*-termux`, then rebuild the CLI in
   `--release` and verify that `version.json` contains the new tag.

## Verification steps

1. Run the CLI in release mode (`cargo run --release --bin codex-tui` or the
   installed Termux binary).
2. Check `~/.config/codex/version.json` and confirm that it contains an up to
   date `latest_version` field.
3. On the next launch, confirm that the `Update available` cell appears with
   the detected version.

If the file is still not updated, run:

```bash
curl -s -H 'User-Agent: codex-termux-debug' \
  https://api.github.com/repos/DioNanos/codex-termux/releases/latest
```

and verify that the `tag_name` field matches one of the formats supported by
the parser.

## LTS Channel Note

If you are testing an LTS build (version contains `-lts`), the update checker must only consider
`*-lts` releases and must not jump to the latest `*-termux` tag. On 2026-02-06 we updated the logic
to:

- For LTS builds: list releases and filter tags ending with `-lts` within the same major.minor line.
- For Termux builds: keep using `releases/latest`.

If you are on macOS with a Homebrew-managed Node.js, note that the global npm shim can live under
`/opt/homebrew/bin`. Upstream update-action detection can treat that as a Homebrew install unless the
launcher sets `CODEX_MANAGED_BY_NPM=1`. This repo's npm wrappers set that env var to ensure the
updater uses the npm update path consistently.
