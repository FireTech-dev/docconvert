# Release Audit Report — docconvert

> Generated 2026-09-25 by the Release Auditor, against `project/architecture.md`, `project/blueprint.md`, `project/agent.md`, and the project itself as it actually stands.

## Verdict: NOT APPROVED

Three Blocking failures stand in the way, one of them found by this audit's own hands-on verification (a core CLI flow that breaks on the most natural invocation). The codebase itself is in strong shape — 139/0/1 green, zero-vulnerability audit, honest errors everywhere — but "ready to ship" additionally means finished (P6-S05/tag), verified shippable (a release actually cut), and working when invoked the way the README shows. None of those three hold yet.

## Summary

| | Count |
| --- | --- |
| Requirements checked | 28 |
| Passed | 20 |
| Failed — Blocking | 3 |
| Failed — Advisory | 1 |
| Not Applicable | 4 |

## Results by Category

### Correctness & Functionality

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Every blueprint step Complete, acceptance verified | Blocking | Fail | P6-S05 In Progress in agent.md; acceptance NOT MET; no v1.0 tag; §14 evidence gaps (24h fuzz, push, tag) |
| No open Known Issue above Low | Blocking | Pass | Both audit Highs resolved same-session; ttf-parser is warning-severity; remaining bullets untagged prose |
| Core flows work end-to-end | Blocking | Fail | Bare-filename md/html conversion fails (`docconvert README.md` → "I/O error: entity not found"); proven Rust `parent()`→`Some("")` + `canonicalize("")` mechanism; `./`-prefixed and subdir paths work |
| Non-happy-path states handled | Advisory | Pass | Corrupt/empty/unknown/encrypted/timeout all yield typed errors with exit codes (corpus + negative-suite evidence) |

### Security

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Security Checklist complete or N/A | Blocking | Pass | Every line [x] or explicit N/A; no bare unchecked boxes |
| No secrets committed | Blocking | Pass | Grep clean (keys/tokens/private blocks); no surprising untracked files |
| No unresolved Critical/High in External Capabilities | Blocking | Pass | cargo-audit 0.22.2: 0 vulnerabilities post quick-xml/calamine remediation |
| Input validation per threat model | Blocking | Pass | Security audit 2026-09-25 verified caps, loaders, staging, argv spawning |
| Dependencies checked for known vulns | Advisory | Pass | cargo audit run same-session, result in agent.md |

### Testing

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| Primary path covered, all tests pass, skips documented | Blocking | Pass | 139/0/1, zero warnings; single ignore (samples write test) documented as user-generated |
| Coverage proportionate to risk | Advisory | Pass | Parsers fuzz-smoked, adversarial + corpus evidence; gap that hid the bare-name bug (tests always use absolute Temp paths) is fixed by B2's recommended regression test |

### Reliability & Error Handling

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| External calls have real error handling | Blocking | Pass | Typed errors throughout; panics converted via catch_unwind; batch isolation; staged-write rollback |
| Logging/diagnostics sufficient | Advisory | Pass | JSON/text reports, warnings-as-data, progress lines, preview mode |

### Performance & Code Quality

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No red flags at expected scale | Advisory | Pass | 100-file gate met (18.3s); documented outliers only (10MB font-dense PDFs slow; debug builds ~15x slower — measure release) |
| Consistent capability level | Advisory | Pass | House (minified) style consistent incl. recent fixes (third audit) |

### Documentation

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| README exists, install/usage verified | Blocking | Pass | Verified end-to-end (`./README.md` → converted, exit 0); stale facts noted below, not the criterion |
| API ref / CHANGELOG / CONTRIBUTING | Advisory | Pass | Arch §14.6 doc set complete (cli-ref, quirks, adding-a-format, pdfium, ci-setup); no CHANGELOG goal exists |

### Configuration & Deployment

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No hardcoded env-specific config | Blocking | Pass | Placeholders only; the one real PDFium URL lives in docs as evidence, not code |
| Release process documented + exercised | Blocking | Fail | Packaging exercised (local artifacts + green CI matrix) but no tag or published artifact exists — the release itself is the unshipped item |
| Rollback path | Advisory | Pass | Git history with scoped commits |

### UI/UX & Accessibility

Not Applicable — headless CLI, no visual interface (architecture.md §1).

### Legal & Compliance

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| License file exists | Advisory | Fail | No LICENSE/COPYING file in tree |
| User-data consideration | Advisory | Pass | Local-only tool; explicitly considered in architecture §9 |

### Project Hygiene

| Requirement | Type | Result | Notes |
| --- | --- | --- | --- |
| No dead code / debug paths | Advisory | Pass | Three audits found none; temporary test/probe files removed |
| Dependencies Architect-approved | Advisory | Pass | Bumps stay inside approved crates (quick-xml, calamine/AD-3); versions recorded in agent.md per standing practice |

## Issues to Address

Ordered by severity — every Blocking failure first.

### Blocking

1. **[Correctness]** — P6-S05 acceptance not met — no v1.0 tag; §14 evidence open (24h fuzz, push, reviews) — close per `docs/ci-setup.md` sequence, then tag.
2. **[Correctness]** — bare-filename md/html conversion fails (`path.parent()` yields `Some("")`, `canonicalize("")` → NotFound) — normalize empty parents to `"."` at the Markdown/HTML loader call sites (and audit the default-output-dir path), plus a regression test converting by bare CWD-relative name. Absolute paths and other formats are unaffected.
3. **[Configuration & Deployment]** — no shipped release to exercise the process against — cut the v1.0 tag + artifacts once B1/B2 close, publish per platform plan.

### Advisory

1. **[Legal]** — no LICENSE file — add the decided license text (never invented here; needs the person's decision).
2. **[Documentation]** — README carries stale facts (95 vs 139 tests, "no lockfile supplied") — refresh the dated claims.
3. **[Project Hygiene]** — stale Known-Issues bullets (e.g. "No Cargo.lock", "uncompiled") — prune to current reality.

## Re-Audit

This report reflects the project as of 2026-09-25 only. Once every Blocking issue is addressed, re-run this audit rather than assuming the fix was sufficient — a new dated report, not an edit to this one.
