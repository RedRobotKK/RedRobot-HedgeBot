# CI Audit Logs

Each file here is a single CI quality gate run, committed automatically by
GitHub Actions on every push to `master`.

## How logs get here

1. You push a commit to `master`
2. `.github/workflows/ci.yml` fires
3. The workflow SSHes into the VPS and runs `cargo test`, `cargo clippy`, `cargo audit`
4. Results are serialised to JSON by a Python script on the VPS
5. The JSON file is committed back here by `github-actions[bot]`

## Filename format

```
YYYY-MM-DD_HHMMSS_<commit>_<status>.json
```

| Part | Example | Meaning |
|------|---------|---------|
| Date/time | `2026-03-13_080300` | UTC timestamp of the CI run |
| Commit | `3235317` | Short SHA of the commit tested on the VPS |
| Status | `PASSED` / `FAILED` / `ADVISORY` / `ERROR` | Gate outcome |

Example: `2026-03-13_080300_3235317_PASSED.json`

## JSON schema (schema_version: "1.0")

Every log file is valid JSON. The full structure is:

```json
{
  "schema_version": "1.0",

  "meta": {
    "run_at":         "2026-03-13T08:03:00Z",   // ISO-8601 UTC
    "commit":         "3235317",                 // short SHA
    "commit_full":    "3235317abcdef...",         // full SHA
    "commit_message": "fix(dashboard): ...",
    "branch":         "master",
    "overall_status": "PASSED",                  // PASSED | FAILED | ADVISORY | ERROR
    "triggered_by":   "push"
  },

  "environment": {
    "rustc":       "rustc 1.75.0 (82e1608df 2023-12-21)",
    "cargo":       "cargo 1.75.0",
    "os_kernel":   "5.15.0-91-generic",
    "arch":        "x86_64",
    "ram_total":   "1.0G",
    "swap_active": true
  },

  "steps": {

    "tests": {
      "status":           "PASSED",   // PASSED | FAILED | ERROR
      "exit_code":        0,
      "duration_seconds": 42,
      "total_passed":     110,
      "total_failed":     0,
      "total_ignored":    2,
      "failures": []                  // populated on failure (see below)
    },

    "clippy": {
      "status":           "PASSED",
      "exit_code":        0,
      "duration_seconds": 15,
      "error_count":      0,
      "errors": []                    // populated on failure (see below)
    },

    "audit": {
      "status":              "PASSED",  // PASSED | ADVISORY | ERROR
      "exit_code":           0,
      "duration_seconds":    3,
      "vulnerability_count": 0,
      "vulnerabilities": []            // populated if CVEs found (see below)
    }
  },

  "service": {
    "name":         "hedgebot",
    "status":       "active",          // systemctl is-active output
    "active_since": "Fri 2026-03-13 06:00:00 UTC",
    "recent_logs":  [                  // last 5 journal lines
      "cycle 42: scanning candidates...",
      "BTC signal: confidence=0.73"
    ]
  }
}
```

## Failure detail schemas

### Test failures (`steps.tests.failures`)
```json
"failures": [
  "tests::position_test::trade_heat_includes_leverage",
  "web_dashboard::tests::pnl_sign_negative_is_minus_not_empty"
]
```
Test names only — open the raw output in the GHA run for the full panic message.

### Clippy errors (`steps.clippy.errors`)
```json
"errors": [
  {
    "code":    "E0308",
    "message": "mismatched types",
    "file":    "src/main.rs",
    "line":    142,
    "col":     20
  },
  {
    "code":    "clippy::unwrap_used",
    "message": "called `.unwrap()` on a `Result` value",
    "file":    "src/candidates.rs",
    "line":    89,
    "col":     15
  }
]
```

### Security vulnerabilities (`steps.audit.vulnerabilities`)
```json
"vulnerabilities": [
  {
    "id":       "RUSTSEC-2023-0071",
    "crate":    "rustls",
    "version":  "0.21.0",
    "title":    "rustls network-reachable panic in certificate parsing",
    "severity": "high",
    "url":      "https://rustsec.org/advisories/RUSTSEC-2023-0071.html"
  }
]
```

## Reading logs programmatically

```python
import json, pathlib

# Latest run
logs = sorted(pathlib.Path('logs/ci').glob('*.json'))
latest = json.loads(logs[-1].read_text())

print(latest['meta']['overall_status'])          # PASSED
print(latest['steps']['tests']['total_passed'])   # 110
print(latest['steps']['audit']['vulnerabilities']) # []
```

## References

These checks follow authoritative Rust best practices:
- `cargo test`: [Rust Book §11](https://doc.rust-lang.org/book/ch11-00-testing.html)
- `cargo clippy`: [Clippy docs](https://doc.rust-lang.org/clippy/)
- `cargo audit`: [RustSec advisory database](https://rustsec.org)
- Test pyramid (70/20/10): [TestRail](https://www.testrail.com/blog/testing-pyramid/)
- NautilusTrader (production Rust trading engine): [GitHub](https://github.com/nautechsystems/nautilus_trader)
