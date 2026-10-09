# Release Audit Report — docconvert

> Generated 2026-10-09 by the Release Auditor, against `project/architecture.md`, `project/blueprint.md`, `project/agent.md`, and the project itself as it actually stands. Re-audit of the 2026-09-25 report (verdict: NOT APPROVED, 3 Blocking failures).

## Verdict: NOT APPROVED

Two Blocking failures remain, both the same underlying cause: the release itself is unshipped — P6-S05 still In Progress with no v1.0 tag and no published artifacts. Everything that *can* be verified without shipping now passes: the prior audit's bare-filename failure is fixed and proven live, the suite is 142/0/1 with zero warnings, `cargo audit` is clean, the security re-audit found nothing blocking, and the README's install/usage path was exercised verbatim. This project is one tag plus published artifacts away from approval, not one unknown away.

## Summary

| | Count |
| --- | --- |
| Requirements checked | 28 |
| Passed | 21 |
| Failed — Blocking | 2 |
| Failed — Advisory | 1 |
| Not Applicable | 4 |

## Results by Category

### Correctness & Functionality

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Every blueprint step Complete, acceptance verified | Blocking | Fail | P6-S05 In Progress, acceptance NOT MET (no v1.0 tag; §14 corpus/CI/publish gaps persist). Fuzz/push/security-review sub-items closed since 2026-09-25 |
| No open Known Issue above Low | Blocking | Pass | Both audit Highs resolved; ttf-parser is warning-severity; fuzz blocker resolved; remainder untagged prose |
| Core flows work end-to-end | Blocking | Pass | Verified live this session: md→txt, txt, csv→md, html→md+assets, bare-CWD-name conversion (the old failure — fixed), preview counts; all exit 0 with correct outputs |
| Non-happy-path states handled | Advisory | Pass | Typed errors + exit codes; preview/unknown/overwrite paths covered by suite and spot checks |

### Security

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Security Checklist complete or N/A | Blocking | Pass | Every line [x] or explicit N/A, verified against the tree |
| No secrets committed | Blocking | Pass | Tracked-tree grep clean. Note (not committed, not a fail): the GitHub PAT used for push lives in local `.git/config` — rotate it; it must never enter a tracked file |
| No unresolved Critical/High in External Capabilities | Blocking | Pass | `cargo audit` run 2026-10-09: 0 vulnerabilities; quick-xml unified 0.41.0 |
| Input validation per threat model | Blocking | Pass | Security re-audit 2026-10-09 (0 blocking) + 3 fresh live probes held |
| Dependencies checked for known vulns | Advisory | Pass | `cargo audit` executed today, result recorded |

### Testing

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Primary path covered, all tests pass, skips documented | Blocking | Pass | Full suite re-run this session: 142 passed, 0 failed, 1 ignored (documented `write_samples`), zero build warnings, 16 suites |
| Coverage proportionate to risk | Advisory | Pass | 24h×3 fuzz clean, 27 adversarial + 27 regression tests, corpus evidence; bare-name gap closed with a regression test |

### Reliability & Error Handling

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| External calls have real error handling | Blocking | Pass | Typed errors throughout; per-file `catch_unwind`; batch isolation + `--strict`; staged-write rollback; symlink refusals |
| Logging/diagnostics sufficient | Advisory | Pass | JSON/text reports (default JSON observed live), warnings-as-data, preview, progress lines |

### Performance & Code Quality

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No red flags at expected scale | Advisory | Pass | 100-file batch gate (18.3s), fuzz throughput (12M/34M/793k runs), documented PDF outliers only |
| Consistent capability level | Advisory | Pass | House style consistent across all five audit passes incl. vendored-hardening and warning-silence work |

### Documentation

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| README exists, install/usage verified | Blocking | Pass | `cargo run --locked -- <file> -o <dir>` exercised verbatim (exit 0, output written); `--preview` verified; doc-set links resolve |
| API ref / CHANGELOG / CONTRIBUTING | Advisory | Pass | §14.6 doc set complete (cli-ref, quirks, adding-a-format, pdfium, ci-setup); no CHANGELOG goal exists |

### Configuration & Deployment

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No hardcoded env-specific config | Blocking | Pass | `src/` grep clean (no URLs/keys); PDFium URL lives in docs as evidence, not code |
| Release process documented + exercised | Blocking | Fail | Documented (`docs/ci-setup.md` §6, `scripts/package_binary.py` present) and locally exercised once (default archive, Sep 25), but no tag and no published artifact exist — the release is the unshipped item |
| Rollback path | Advisory | Pass | Scoped git history, pushed to origin (9cc9be1..ee8981d verified in sync) |

### UI/UX & Accessibility

Not Applicable — headless CLI, no visual interface (architecture.md §1).

### Legal & Compliance

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| License file exists | Advisory | Fail | Still no LICENSE/COPYING file in tree (unchanged since 2026-09-25; needs the person's license decision, never invented here) |
| User-data consideration | Advisory | Pass | Local-only tool; explicitly considered in architecture §9 |

### Project Hygiene

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No dead code / debug paths | Advisory | Pass | Five audits found none in shipped code; all temp probe files removed (`tests/zz*` absent); warning-silence work is allows/renames, not dead code |
| Dependencies Architect-approved | Advisory | Pass | Vendoring user-authorized (AQ-013); bumps inside approved crates; patch pins recorded; lockfiles committed |

## Issues to Address

Ordered by severity — every Blocking failure first.

### Blocking

1. **[Correctness]** — P6-S05 acceptance not met — no v1.0 tag; §14 items 1–2, 4–5, 7 without full evidence (corpus short for epub/html/odt/xml, remote CI matrix never executed, no published artifacts) — close per `docs/ci-setup.md` §6, then tag. (Fuzz, push, and security-review sub-items are done since the last audit.)
2. **[Configuration & Deployment]** — no shipped release to exercise the process against — cut the v1.0 tag + per-platform artifacts for both build profiles once item 1 closes, publish per the platform plan.

### Advisory

1. **[Legal]** — no LICENSE file — add the decided license text (needs the person's decision).
2. **[Documentation]** — README carries stale facts (2026-09-23/95-test claims, "no lockfile supplied" though Cargo.lock is committed) plus four stray `# docconvert` trailer headings — refresh dated claims and remove the trailer.
3. **[Project Hygiene]** — stale `agent.md` prose (Known-Issues "No Cargo.lock"/"uncompiled" bullets, Overall Progress "unpushed" note) — prune to current reality.

## Re-Audit

This report reflects the project as of 2026-10-09 only. Once every Blocking issue is addressed, re-run this audit rather than assuming the fix was sufficient — a new dated report, not an edit to this one.
