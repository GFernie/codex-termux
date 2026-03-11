# LTS Model Testing Worker Guide

This page is the worker-facing runbook for model-focused Linux validation on the LTS line.

Use it as the single entry point when a worker is asked to validate one or more provider/model profiles.

The worker should not improvise the test structure. The worker should:

- pick the assigned model suite
- execute it in order
- record only neutral technical results
- produce a sanitized report suitable for commit history or repository storage
- reject fallback-to-another-model runs as blocked, not validated

## Target Profiles

Required release-gate profiles:

- `glm5-coding`
- `qwen35-coding`

Secondary validation profiles:

- `qwen3-coder-plus`
- `qwen3-coder-next`
- `qwen3max-plan`
- `glm47-coding`
- `kimi-k25-coding`
- `minimax-m25-coding`

## Suite Locations

- GLM-5 Linux model suite: [`../test-reports/suites/lts/models/linux-glm5.md`](../test-reports/suites/lts/models/linux-glm5.md)
- Qwen 3.5 Linux model suite: [`../test-reports/suites/lts/models/linux-qwen35.md`](../test-reports/suites/lts/models/linux-qwen35.md)
- Qwen 3 Coder Plus Linux model suite: [`../test-reports/suites/lts/models/linux-qwen3-coder-plus.md`](../test-reports/suites/lts/models/linux-qwen3-coder-plus.md)
- Qwen 3 Coder Next Linux model suite: [`../test-reports/suites/lts/models/linux-qwen3-coder-next.md`](../test-reports/suites/lts/models/linux-qwen3-coder-next.md)

Profiles without a dedicated suite yet should be recorded as availability-only until they receive a model-specific suite.

## Worker Flow

Follow this order exactly unless the task explicitly says otherwise:

1. confirm the installed binary is the intended LTS binary
2. confirm the assigned profile resolves
3. run the generic Linux Core suite first if it has not already been signed off
4. run the model-specific suite for the assigned profile
5. write the report using the neutral template
6. sanitize the report before storing or committing it

If the worker is assigned more than one model:

1. `glm5-coding`
2. `qwen35-coding`
3. `qwen3-coder-plus`
4. `qwen3-coder-next`

This order puts release-gate profiles first.

## Worker Inputs

Before starting, the worker should know:

- LTS version under test
- current commit or package candidate
- target profile
- whether MCP/web search is expected to be available
- whether the session is allowed to use full sandbox for mutation tests

## Execution Rules

The worker should keep the execution disciplined:

- use disposable workspaces for edit tests
- keep prompts short and operational
- record actual behavior, not interpretations
- if a test fails, capture the smallest useful error snippet
- if a failure is environmental, label it clearly as environmental
- do not paste full logs or session transcripts unless specifically requested

## How To Use The Suites

Run model suites after the generic Linux Core suite is already green.

Each model suite is meant to prove three things:

- the profile resolves correctly
- the model handles real Codex work without derailing
- the failure mode stays recoverable when a task is intentionally awkward

The model suites are intentionally narrower than the full Extended suite:

- they emphasize realistic coding and tool use
- they avoid redundant coverage already handled in Core
- they make it easier to compare one model against another

If the requested profile silently resolves to a different provider or model, the suite is
not validated. Record the run as `BLOCKED` and state the actual provider/model used.

## What The Worker Must Record

For each model report, record:

- version under test
- commit under test
- profile and model slug
- pass/fail for each suite section
- one short note per failure
- whether the failure is blocking or advisory
- the actual resolved provider/model when it differs from the target

Do not record:

- personal names
- hostnames
- local machine nicknames
- absolute local paths
- API key names when a neutral phrase is enough
- raw credentials, cookies, headers, or tokens
- large pasted conversation dumps

## Neutral Report Rules

When you compile a report for commit history or release notes, keep it neutral:

- do not include real hostnames
- do not include real tester names
- do not include local absolute paths
- do not include API key names if avoidable
- do not include raw tokens, secrets, headers, or session dumps
- prefer `Configured via environment` over a real env var name
- prefer `Linux x86_64` over a machine name
- prefer `Local release build` over a local filesystem path

Good examples:

- `Platform: Linux x86_64`
- `Tester: Local validation`
- `API key: Configured via environment`
- `Binary source: Local release build`

Bad examples:

- `Platform: Linux x86_64 (host.example.com)`
- `Tester: personal identity`
- `Binary path: /home/user/...`
- `API Key: REAL_PROVIDER_KEY_NAME`

## Worker Sanitization Checklist

Before saving the report, the worker should verify:

- platform text is generic
- tester text is generic
- binary source text is generic
- no env var names are exposed unless needed for a debugging-only local note
- no host or path data appears in command examples or conclusions
- command output is trimmed to the minimal useful lines
- commit reference matches the build actually tested

## Report Template

Use the neutral template:

- [`../test-reports/lts/MODEL_REPORT_TEMPLATE.md`](../test-reports/lts/MODEL_REPORT_TEMPLATE.md)

## Release Interpretation

Interpretation for Linux model signoff:

- `glm5-coding`: must pass
- `qwen35-coding`: must pass
- `qwen3-coder-plus`: should pass before broader provider promotion
- `qwen3-coder-next`: should pass before broader provider promotion

If a secondary profile fails but the required release-gate profiles are green, LTS release may still proceed, but the failure should be recorded clearly.

Fallback to a different provider/model does not count as a pass for that profile, even if
the generic CLI behavior is otherwise healthy.

## Expected Output From A Worker

The final worker artifact should be:

- one sanitized report file per tested model
- short enough to review quickly
- explicit about what passed, what failed, and what remains unverified

Good worker output is concise, neutral, and reproducible.
