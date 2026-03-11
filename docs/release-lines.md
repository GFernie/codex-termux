# Release Lines

This repository intentionally keeps two public branches only:

- `main`: latest line
- `lts`: long-term support line

## `main`

- package: `@mmmbuto/codex-cli-termux`
- target: latest Termux-focused packaging and maintenance
- role: fastest line for upstream sync and latest-termux packaging

## `lts`

- package: `@mmmbuto/codex-cli-lts`
- upstream base: `0.80.x`
- role: stability-first line for `/chat`, tool compatibility, and conservative backports

## Branch Rules

- GitHub should expose only `main` and `lts`
- experimental work can happen on local branches, but it must land back onto one of the two public lines
- root docs must explain both lines without mixing their changelogs
