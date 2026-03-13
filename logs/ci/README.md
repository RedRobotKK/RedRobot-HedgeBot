# CI Audit Logs

Each file here is a single CI quality gate run, pushed automatically by `deploy.sh`.

## Filename format

```
YYYY-MM-DD_HHMMSS_<commit>_<result>.log
```

| Part | Example | Meaning |
|------|---------|---------|
| Date/time | `2026-03-13_080300` | When the CI run started (local time) |
| Commit | `3235317` | Git commit hash that was tested |
| Result | `PASSED` / `FAILED` / `SKIPPED` | Gate outcome |

Example: `2026-03-13_080300_3235317_PASSED.log`

## What each log contains

```
════════════════════════════════════════════════════════════════
CI RUN  2026-03-13 08:03:00 JST  commit=3235317
════════════════════════════════════════════════════════════════

── Step 1/3: cargo test --all ──────────────────────────────────
   (full test output — every pass/fail with test names)
✓ PASS  110 passed, 0 failed

── Step 2/3: cargo clippy --all-targets -D warnings ────────────
   (full clippy output — zero means clean)
✓ PASS

── Step 3/3: cargo audit (RustSec CVE scan) ────────────────────
   (dependency vulnerability scan against rustsec.org)
✓ PASS  0 vulnerabilities

════════════════════════════════════════════════════════════════
CI SUMMARY  commit=3235317
────────────────────────────────────────────────────────────────
  cargo test   │ ✅ PASS (110 tests)
  cargo clippy │ ✅ PASS
  cargo audit  │ ✅ PASS
────────────────────────────────────────────────────────────────
  Passed: 3/3   Failed: 0/3
════════════════════════════════════════════════════════════════
```

On failure, the log includes full diagnostics:
- **Test failures**: test name, panic message, file/line
- **Clippy failures**: error code, file path, line number, fix suggestion
- **Audit failures**: CVE ID, affected crate, severity, advisory URL

## References

These checks follow authoritative Rust best practices:
- `cargo test`: [Rust Book §11](https://doc.rust-lang.org/book/ch11-00-testing.html)
- `cargo clippy`: [Clippy docs](https://doc.rust-lang.org/clippy/)
- `cargo audit`: [RustSec advisory database](https://rustsec.org)
