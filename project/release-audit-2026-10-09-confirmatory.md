# Release Audit Report — docconvert (confirmatory)

> Generated 2026-10-09 by the Release Auditor, against `project/architecture.md`, `project/blueprint.md`, `project/agent.md`, and the project itself as it actually stands. Confirmatory re-audit of `release-audit-2026-10-09.md` (verdict: NOT APPROVED, 2 Blocking failures — unshipped release). Both Blocking items have since been actioned: tag `v1.0` cut + pushed, tag-CI 10/10 green, Release `v1.0` published with 21 assets. All three prior Advisory items were fixed in `bb50bf7` (dual license, README refresh, state prune).

## Verdict: APPROVED

Zero Blocking requirements fail. The release is real and verified live: tag `v1.0` on origin at `bb50bf7`, tag-CI run 37913020914 green across all 10 jobs (API-verified, not user-reported), 21 published assets hash-verified with the shipped linux-x86_64 binary smoke-tested end-to-end. The one residual gap — §14 item-4 corpus depth for epub/html/odt/xml — is explicitly owner-waived, publicly disclosed in the release notes, and carried below as an Advisory, not a silent loss. Nothing else stands in the way of use.

## Summary

| | Count |
| --- | --- |
| Requirements checked | 28 |
| Passed | 23 |
| Failed — Blocking | 0 |
| Failed — Advisory | 1 |
| Not Applicable | 4 |

## Results by Category

### Correctness & Functionality

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Every blueprint step Complete, acceptance verified | Blocking | Pass | Phases 0–6 archived Closed; P6-S05 acceptance now evidenced: tag + green tag-CI + published artifacts (all API-verified this session). §14-item-4 corpus partial (see Advisory) is owner-waived per project documented-decision precedent |
| No open Known Issue above Low | Blocking | Pass | Both audit Highs resolved (quick-xml→0.41, calamine→0.36.1); ttf-parser warning-severity residual only; fuzz blocker resolved + gate met |
| Core flows work end-to-end | Blocking | Pass | Shipped release binary verified live: `--help`, md→txt (correct heading/link rendering), `--preview` counts; prior audit covered csv/html+assets/bare-name on byte-identical code |
| Non-happy-path states handled | Advisory | Pass | Typed errors + exit codes; preview/unknown/overwrite/encrypted paths covered by suite and smoke tests |

### Security

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Security Checklist complete or N/A | Blocking | Pass | Every line [x] or explicit N/A, as previously verified; nothing added since |
| No secrets committed | Blocking | Pass | Tracked-tree grep clean (only benign glyph/test/doc matches). Token lives in untracked `.git/config` + shell history only — rotate it; never commit it |
| No unresolved Critical/High in External Capabilities | Blocking | Pass | `cargo audit` 0 vulnerabilities (today); dependency tree unchanged since |
| Input validation per threat model | Blocking | Pass | Security re-audit 2026-10-09: 0 blocking; code tree unchanged since (only `agent.md` + reports) |
| Dependencies checked for known vulns | Advisory | Pass | `cargo audit` executed 2026-10-09, result recorded |

### Testing

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Primary path covered, all tests pass, skips documented | Blocking | Pass | 142 passed, 0 failed, 1 ignored (documented `write_samples`), zero warnings; `git diff` confirms `src/`+`tests/` identical to the verified tree — only `agent.md` changed |
| Coverage proportionate to risk | Advisory | Pass | 24h×3 fuzz clean (12M/34M/793k runs), 27 adversarial + 27 regression tests, corpus evidence recorded |

### Reliability & Error Handling

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| External calls have real error handling | Blocking | Pass | Typed errors; per-file `catch_unwind`; batch isolation + `--strict`; staged-write rollback; symlink refusals — unchanged, suite-pinned |
| Logging/diagnostics sufficient | Advisory | Pass | JSON/text reports, warnings-as-data, preview, progress lines (all observed live) |

### Performance & Code Quality

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No red flags at expected scale | Advisory | Pass | 100-file batch 18.3s (≪5min gate); documented PDF outliers only |
| Consistent capability level | Advisory | Pass | House style consistent across all five audit passes incl. vendoring + warning-silence work |

### Documentation

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| README exists, install/usage verified | Blocking | Pass | Usage pattern proven on the shipped binary; `cargo run --locked` path verified in prior audit on identical code; `bb50bf7` README edits are prose facts only (dates, lockfile claim, license section, trailer removal) |
| API ref / CHANGELOG / CONTRIBUTING | Advisory | Pass | §14.6 doc set complete (cli-ref, quirks, adding-a-format, pdfium, ci-setup); no CHANGELOG goal exists |

### Configuration & Deployment

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No hardcoded env-specific config | Blocking | Pass | `src/` clean; PDFium URL lives in docs as evidence, not code |
| Release process documented + exercised | Blocking | Pass | `docs/ci-setup.md` §6; exercised for real: tag pushed → CI built → Release `v1.0` (id 407834593) with 21 assets; tarball hash matches published SHA256SUMS; shipped binary smoke-tested |
| Rollback path | Advisory | Pass | Tag + published release + full pushed history; prior release is one click away |

### UI/UX & Accessibility

Not Applicable — headless CLI, no visual interface (architecture.md §1).

### Legal & Compliance

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| License file exists | Advisory | Pass | `LICENSE-MIT` + `LICENSE-APACHE` committed in `bb50bf7`; `Cargo.toml` declares `MIT OR Apache-2.0` |
| User-data consideration | Advisory | Pass | Local-only tool; explicitly considered in architecture §9 |

### Project Hygiene

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No dead code / debug paths | Advisory | Pass | Five audits found none; all temp probes removed; worktree diff vs tag is `agent.md` logs only |
| Dependencies Architect-approved | Advisory | Pass | Vendoring user-authorized (AQ-013); bumps inside approved crates; lockfiles committed |

## Issues to Address

Ordered by severity — every Blocking failure first.

### Blocking

None.

### Advisory

1. **[Correctness]** — §14 item-4 corpus depth partial for epub/html/odt/xml (representative-only samples, no ≥20-doc-per-format evidence) — owner-waived with documented caveats and publicly disclosed in the v1.0 release notes; strictly, full evidence would require a broader corpus run. Transparent as stated; close by running the corpus if the bar is ever required literally.
2. **[Security]** — rotate the GitHub token supplied for publication (lives in local `.git/config` + shell history, both untracked) and strip it from the config afterwards.

## Re-Audit

This report reflects the project as of 2026-10-09 only. It closes the release gate: no further audit is required for v1.0. Future versions get their own dated reports, not edits to this one.
