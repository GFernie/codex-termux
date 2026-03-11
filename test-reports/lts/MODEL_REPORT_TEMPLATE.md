# LTS Model Test Report

Use this template as-is for worker-produced reports. Keep the wording neutral and avoid environment-specific identity details.

**Version:** 0.80.x-lts
**Date:** YYYY-MM-DD
**Platform:** Linux x86_64
**Binary:** Built from local release source
**Tester:** Local validation
**Provider/Profile:** `<provider>/<profile>`
**Model:** `<model>`
**Resolved Target:** `<provider>/<model>` | `BLOCKED - fallback`

---

## Test Environment

- **OS:** Linux x86_64
- **Build:** Local release build
- **Commit:** `<short-sha>`
- **API key:** Configured via environment

## Worker Notes

- Assigned suite: `<suite-path>`
- Worker outcome type: Release-gate | Secondary validation
- MCP available: Yes | No
- Web search available: Yes | No
- Full sandbox used for mutation checks: Yes | No

---

## Results

### Profile resolution

- Status: PASS | FAIL
- Notes:

### Two-turn chat stability

- Status: PASS | FAIL
- Notes:

### Workspace inspect/search/read

- Status: PASS | FAIL
- Notes:

### Shell execution and output capture

- Status: PASS | FAIL
- Notes:

### Single-file edit

- Status: PASS | FAIL
- Notes:

### Multi-file edit

- Status: PASS | FAIL
- Notes:

### `/plan` and `/code`

- Status: PASS | FAIL
- Notes:

### Failure mode recovery

- Status: PASS | FAIL
- Notes:

---

## Summary

- Overall: PASS | FAIL
- Blocking issues:
- Advisory issues:
- Not verified:
- Actual provider/model used:

## Sanitization Check

- No hostname included
- No tester name included
- No local absolute path included
- No secret or token included
- No env var name included unless strictly necessary
- Commit matches the binary actually tested

## Conclusion

Short neutral conclusion here.
