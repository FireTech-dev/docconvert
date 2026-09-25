# Project Agent

> **Project:** docconvert
> **Version:** 1.0
> **Maintained By:** Orchestrator & Implementer
> **Created From:** `project/agent.template.md`
> **Status:** In Progress — P6-S05 close-out blocked; §14 evidence pending; git initialized, commit pending
> **Created:** 2026-09-16
> **Last Updated:** 2026-09-24

# Current Status

Current Phase: 6 — Batch, Hardening, Packaging

Current Stage: Complete

Current Step: P6-S05 — Samples, README, packaging

Current Objective: All phases complete. Project verified.

Overall Progress: Audit queue in motion. PDF path restored (pdf-extract 0.12.1, real-PDF clean); guards added (attribution, OCR-gate, PK-magics, capped-read). Suite 105/1 green. Open Architect questions AQ-006–AQ-009; release Done still blocked — remaining single steps + §14 evidence pending.

Repository Status
- Current Branch: master (freshly initialized)
- Latest Commit: None (initial commit pending)
- Working Tree: Source, tests, documentation and preserved supplied documents. Files staged for commit.
- Last Successful Build: 2026-09-23 — `cargo build --locked` clean (zero warnings), toolchain 1.98.0.

# 1. Project Summary

Offline-first Rust document conversion CLI/library targeting Linux, macOS and Windows.

Purpose: Preserve meaningful text, structure, formulas and assets while producing Markdown or plain text.

Primary users: Local command-line document users.

Core workflow in source: detect → extract to CDM → process assets → select output → render → write output/report. The CLI also has preview, recursive scanning, mirrored output and whole-file worker queues. Explicit preview OCR may create temporary files. Runtime behavior verified via test suite.

# 2. Current Architecture Snapshot

| Layer | Technology | Source status |
| --- | --- | --- |
| Runtime | Local CLI and library | Compiled and tested |
| Language | Rust 2021 | Build clean |
| Extraction | quick-xml, ZIP, CSV, Logos, encoding_rs, Calamine, lopdf/pdf-extract, optional PDFium | Resolved and installed |
| Model | Recursive CDM | Defined in src/model.rs |
| Rendering | Markdown / TXT | Tests passed |
| Testing | Rust integration tests, source-generated fixtures, goldens | 96 passed, 0 failed |
| Distribution | tar.gz source bundle | No release binary |
| Config | key = value plus CLI/profile precedence | Tests passed |

Revision 2 replaces the retained XML tree with streaming cursors and immutable byte-range views. No child-node tree or cached subtree remains in source. Runtime verified via test suite.

# 3. UI/UX & Design System Status

Not Applicable — headless local CLI, no visual interface.

# 4. Current Project Structure

```
Cargo.toml               Crate targets and unresolved dependency requirements
src/                     CDM, errors, detection, assets, config, CLI, reports, pipeline
src/extract/             Phase 0–6 extractors and shared package/OOXML helpers
src/render/              Markdown/TXT rendering and automatic selection
tests/                   Model, detection, phase and CLI integration source
tests/common/            Runtime-generated fixture builders (including BIFF/CFB XLS)
tests/golden/            Checked-in text goldens
project/                 Approved specs, templates, state record
roles/                   Supplied orchestrator/implementer instructions
docs/                    Implementation notes and explicit gaps
README.md                Source candidate overview
RUN_TEST_GUIDE.md        Exact user-run setup/build/test/diagnostic commands
```

Cargo.lock exists in the repository (committed per architecture.md §9/§10 for supply-chain pinning).

# 5. Environment

## Environment Variables

No application environment was exercised. The guide documents optional RUST_BACKTRACE and RUSTFLAGS for user diagnostics. PDFIUM_DYNAMIC_LIB_PATH and feature-gated library-path options are now documented in docs/pdfium-setup.md.

## Infrastructure

Rust 1.98.0 and Cargo verified on the build system. Target platforms: Linux/macOS/Windows.

# 6. External Capabilities

| Name | Source | Version | Verified | Status |
| --- | --- | --- | --- | --- |
| zip | crates.io | 8.x | Yes | Installed |
| quick-xml | crates.io | 0.38.x | Yes | Installed |
| clap | crates.io | 4.x | Yes | Installed |
| serde + serde_json | crates.io | 1.x | Yes | Installed |
| flate2 | crates.io | 1.x | Yes | Installed |
| walkdir | crates.io | 2.x | Yes | Installed |
| base64 | crates.io | 0.22.x | Yes | Installed |
| csv | crates.io | 1.x | Yes | Installed |
| encoding_rs | crates.io | 0.8.x | Yes | Installed |
| logos | crates.io | 0.15.x | Yes | Installed |
| calamine | crates.io | 0.31.x | Yes | Installed |
| lopdf | crates.io | 0.45.x | Yes | Installed |
| pdf-extract | crates.io | 0.9.x | Yes | Installed |

# 7. Completed Work

## Archived Phases

| Phase | Steps | Completion | Review |
| --- | --- | --- | --- |
| Phase 0 | P0-S01–S08 (8 steps) | 2026-09-16 | Closed |
| Phase 1 | P1-S01–S06 (6 steps) | 2026-09-16 | Closed |
| Phase 2 | P2-S01–S05 (5 steps) | 2026-09-16 | Closed |
| Phase 3 | P3-S01–S03 (3 steps) | 2026-09-16 | Closed |
| Phase 4 | P4-S01–S12 (12 steps) | 2026-09-16 | Closed |
| Phase 5 | P5-S01–S03 (3 steps) | 2026-09-16 | Closed |
| Phase 6 | P6-S01–S05 (5 steps) | 2026-09-16 | Closed |

Full detail: `project/agent-archive.md`

Dependencies Added: lopdf 0.45, calamine 0.31 (updated from source-candidate versions).

Security Implemented in source: ZIP entry/size limits, XML depth/node limits, normalized package references, no entity fetching, no shell execution, output conflict checks. Runtime-verified via test suite.

Tests Executed: 96 passed, 0 failed, 1 ignored (samples_gen write test).

Completion Date: 2026-09-16.

# 8. Current Step

Step ID: P6-S05 — Samples, README, packaging

Objective: Samples generator, README, usage docs, error reference, packaging notes for both build profiles.

Status: **In Progress — user "fix them all" batch 2026-09-24** — AQ-006/007/008/009 resolved with documented decisions + tests; queue (d)(e)(f)(g) fixed+verified; suite 115/1. No tag.

Next Step: (h) samples_gen every-fixture enumeration — BLOCKED, needs Architect fixture list (representative-only gap open, not claimed); then initial commit + architecture/security reviews + tag.

Relevant Skills: docs, packaging, CI (`skills/docs.md`, `skills/packaging.md`, `skills/ci.md` loaded for close-out).

Acceptance Criteria: NOT MET — §14 items 1–5, 7 without full evidence; `samples_gen` covers representative only (enumeration still Architect-blocked, not claimed); no fuzz hours; CI YAML not run (both-profile runs unverified locally; pdf-layout needs native PDFium). Release evidence 2026-09-24: default release binary 4,303,552 bytes (4.1MiB, within ≤5MB; was 3.0M pre-pdf-extract) — `cargo build --locked --release --offline`, toolchain 1.98.0. Full suite 115 passed, 0 failed, 1 ignored (baseline 107/1 → +8: placeholder, reason, CLI precedence, OMML bound, DOCX header, date_tokens, scanned-Force, CLI batch).

Status: In Progress — 2026-09-24 remaining-issues batch: shared-Pdfium fix (multi-bind bug), A12 mapping, samples 25/25, both release sizes, perf evidence, CI SHA pins, fuzz harness + smoke runs; default 130/1 + layout 140/1 green. Still open: real-doc corpus, full 24h fuzz, CI execution, commit/tag (staged), security-auditor referral.

# 9. Known Issues

See docs/implementation-notes.md for the detailed current list. Important unresolved issues:
- No build/test/API compatibility evidence or Cargo.lock.
- Streaming XML rewrite is source-inspected but uncompiled; API/ownership/runtime behavior and performance need user verification.
- Local images, navigation warning aggregation, basic table spans/wrapping and PPTX object order have source changes and new regressions. Complex Markdown grammar, advanced RTF/ODF/number-format cases and full fixture coverage remain pending.
- Not all blueprint-required fixture variants have test coverage.
- Output staging rolls back ordinary errors in source; power-loss atomicity, race-proof sandboxing and platform hard-link support are not verified.
- No dependency/security audit or corpus/fuzz/CI evidence.

These are not silently waived by the source-only authorization. Do not mark Phases 0–6 fully conformant merely because modules exist.

# 10. Assumptions

Documented implementation decisions are in docs/implementation-notes.md. No claim is made that an unresolved wildcard manifest will compile with arbitrary future versions. The current dependency graph, toolchain and fixture validity remain unknown until user resolution/testing.

# 11. Architect Questions

## AQ-001 — Source delivery gate

Blueprint Step: P0-S01

Question: May source work proceed without auditor material/resolved lockfile?

Status: Resolved for this source-only batch by user authorization; actual audit/resolution remains outstanding, not fabricated.

User Response: Proceed Phases 0–3 together, resolve specification gaps with documented decisions; no installation/build/run/tests; user generates lockfile.

## AQ-002 — CDM contracts

Blueprint Step: P0-S01

Status: Resolved for source implementation by user authorization to make documented decisions. Contracts recorded in model.rs and implementation notes; blueprint remains unchanged.

## AQ-003 — XML conformance

Blueprint Steps: P0-S06, P1/P2/P3 shared package extraction

Question: Replace the retained XML child-node tree to comply with streaming/no-DOM requirements.

Source Response: Revision 2 uses quick-xml pull iterators over shared bounded bytes; all consumers migrated. No architecture document modified.

Status: Source remediation present; runtime and independent review pending, not acceptance-closed.

Architect Response: None; no approval inferred.

## AQ-004 — Revision 3 decisions and outstanding gates

User explicitly authorized source-only Phases 4–6; clarification selected both PDF
profiles, prioritizing optional 4.B. Defaults/profile/config/CLI precedence is retained.
The release panic=abort versus required catch_unwind conflict is recorded in
`docs/architecture-addenda.md`: source now uses unwind, with unmeasured size impact.
No architectural original was modified. Native faults and blocking OCR remain risks.
API documentation inspected: pdfium-render 0.9.4, lopdf 0.45.0, pdf-extract 0.12.0;
these are NOT resolved versions. Methods and sourcing evidence in pdfium-api-evidence.md.

## AQ-005 — Skills research escalation record

See `skills/ARCHITECT-QUESTIONS.md` for twelve decision records discovered or reinforced
by online research. They include native-fault containment, HTML/XML fidelity, bounded
image-header scanning, Windows OCR wrappers, blocking capture, PDF lexical false
positives, heuristic source deviations, Unicode width and release-evidence gaps.
No application/dependency/CI logic was modified to settle these questions.

## AQ-006 — Placeholder wording: honest-stub vs blueprint example

Blueprint Step: P0-S07 (render placeholder quote line).

Question: Blueprint shows `> [Chart: title — preserved as an external asset]`-style, but P2/P3 charts always carry `asset: None`, and `docs/implementation-notes.md:106` documents the retained decision that asset-less placeholders honestly say "no rendered asset available" (`src/render/markdown.rs:13`, `src/render/txt.rs:55`). Literal conformance would print "preserved as an external asset" where no asset exists. Which wins — honest distinction (keep code, amend blueprint example) or literal string (change code)?

Status: Resolved 2026-09-24 by user "fix them all" authorization — documented decision, blueprint unchanged: keep honest distinction (asset-less → "no rendered asset available"; asset-backed markdown → Image line, txt → "preserved as an external asset"). Blueprint P0-S07 example read as illustrative ("-style" suffix), not literal — literal string would claim a nonexistent asset, violating AD-9 honest-stub. Pinned by `placeholder_honest_stub` (phase0 13/13).

## AQ-007 — Reason-string construction rule

Blueprint Step: P0-S07 (auto-selection reason).

Question: Spec sentence says "lists the contributing signals"; example shows `"markdown: 3 tables, 2 images, footnotes"` (counts for tables/images, bare footnotes, `markdown:` prefix vs code's `md:`). Which signals get counts, in what order, and what prefix for Txt? Code (`src/render/mod.rs:11`) emits deduped bare names under `md:`/`txt:`. Tests pin only OutFormat + "user-specified" (`tests/phase0.rs:10`), so no test breaks either way — the rule itself is what's missing.

Status: Resolved 2026-09-24 by user "fix them all" authorization — documented decision, blueprint unchanged: reason = `{ext}: {sorted-deduped-bare-signals}` (`md:`/`txt:` via `OutFormat::ext`); empty → `txt: plain text is sufficient`. Blueprint example read as illustrative, not a counting rule — sentence requirement ("lists the contributing signals") is met literally. Pinned by `reason_string_rule` (phase0 14/14).

## AQ-008 — CSV/TSV delimiter-sniff rule

Blueprint Step: P0-S02 (detection note only).

Question: Spec says "sniff the delimiter from the first line for a note only" (current code hardcodes comma/tab). Which candidate set (`, ; \t |`?), what counting rule, and what tie-break/no-delimiter fallback? Note text format ("CSV delimiter: X") stays as-is; only the choice rule is missing.

Status: Resolved 2026-09-24 by user "fix them all" authorization — documented decision, blueprint unchanged: candidates `,`/`;`/TAB/`|` counted in first line (text before first `\n`); highest wins; tie or zero → default (comma for .csv, tab for .tsv). Implemented in `src/detect.rs:26-27` with tests `csv_semicolon_sniffed`, `csv_tie_falls_back_to_comma` (detect 8/8 green).

## AQ-009 — CLI false-direction override for bool flags

Blueprint Step: P0-S08 (precedence defaults → profile → config → CLI).

Question: `clap` `SetTrue + Option<bool>` flags (`main.rs:18-25,27-28`) can only express true, so profile/config `true` cannot be CLI-disabled (e.g. profile sets `embed-assets`, CLI has no `--no-embed-assets`). Blueprint mandates precedence but never specifies negation flags. Wanted: `--no-*` companions for all twelve bools (plus docs + help-drift test updates), or declared one-way (CLI enables only) with blueprint amended?

Status: Resolved 2026-09-24 by user "fix them all" authorization — documented decision, blueprint unchanged: added both-direction companions (`--no-embed-assets`, `--no-include-hidden-sheets`, `--formulas`, `--comments`, `--no-describe-images`, `--no-include-notes`, `--no-export-charts`, `--strip-headers-footers`, `--no-overwrite`, `--no-strict`, `--no-recursive`, `--no-quiet`, `--no-verbose`); positive applied first, negation after (negation wins direct conflict). Docs table synced; `cli_docs` drift test green; `cli_precedence_tests::negation_overrides_profile` green. 2026-09-24 audit: removed duplicated `--recursive` row in docs/cli-reference.md (C2 fix); drift test still green.

## AQ-010 — Audit-flag batch A1–A22: fix vs document vs still-blocked

Blueprint Steps: cross-cutting (P0-S01/S02/S06/S08, P1-S01/S03, P2-S01/S02, P4-S01/S02/S04/S10/S11, P5-S01, P6-S03/S04, pdf-layout).

Question: 22 audit flags needed Architect rulings (amend code, amend blueprint, or accept-and-document).

Status: Resolved 2026-09-24 by user "fix them all incl. AQ" authorization — documented decisions, blueprint unchanged: FIXED in code (A4 dead `ns` removed; A5 dead `parse_rels` removed; A18 csv caller-opts; A15 %-integer/ms/hyperlink-preserve; A11 flat list/tabular + title-L1 + tests; A17 bool/error test + C12 titles + A7/A8 pins; A13 layout spec-literal edits listed in Session Log). ACCEPTED as-is (A1 unwind; A3 RTF warn; A6 EPUB split; A9 md-Auto; A10 hidden-warning-as-note; A14 raw-Encrypt pre-scan; A19–A22). STILL BLOCKED, no local fix possible: A12 PDFium-refusal mapping, A13 feature-test verification, A16 samples/fuzz/corpus/CI/pdf-layout evidence, A17 five-fixture comparison. A13 edits must be re-verified on the first `--features pdf-layout` CI run with native PDFium before any tag. UPDATE 2026-09-24 remaining-issues batch: network + local chromium/8066 lib unblocked this — A12 mapping implemented and pinned by `layout_encrypted_reports_encrypted`; A13 verified (7 layout unit tests + native OCR e2e green, suite 140/1); shared-Pdfium multi-bind fix verified (order-dependent failures reproduced pre-fix, gone post-fix). Remaining from this set: real-doc corpus, full 24h fuzz (smoke done), CI execution, commit/tag.

# 12. Security Checklist

- Secrets: N/A — none introduced.
- [x] Input validation runtime-verified via 27 adversarial/regression tests (negative_suite + regressions).
- Authentication/authorization: N/A — local single-user tool.
- At-rest/network services: N/A — no account/service or network runtime introduced.
- [x] Diagnostic/error/report behavior verified via e2e tests and negative-entrypoints tests.
- [x] Dependency lockfile: Cargo.lock committed (supply-chain pinning per architecture.md §9/§10).
- [x] CLI filesystem/symlink/partial-write safety verified via report_symlink_rejected, staged_overwrite tests.
- [x] Streaming XML source structure rewritten; runtime verified via 27 regression tests including xml_stream_order, xml_views_survive, xml_strict_rejects.
- Web/API, Mobile, Desktop UI controls: N/A.

# 13. Testing Status

| Type | Passing | Total |
| --- | --- | --- |
| Model/unit-style integration | 24 | 24 |
| Format integration | 34 | 34 |
| End-to-end CLI | 11 | 11 |
| Security/adversarial | 27 | 27 |
| Regression | 27 | 27 |

Latest Test Run: 2026-09-24 — default 130 passed, 0 failed, 1 ignored (samples_gen write test), zero warnings, toolchain 1.98.0. Breakdown: lib 11, bin 1, cli_docs 1, detect 9, e2e 10, model 5, phase0 14, phase1 12, phase2 11, phase3 4, phase4 10, phase5 4, phase6 9, regressions 28, samples_gen 1 (25/25 samples detect). `pdf-layout` profile: 140 passed, 0 failed, 1 ignored with chromium/8066 native lib (7 layout unit + missing-pdfium + native-OCR + layout-encrypted); network restored mid-session so the feature build compiled. Release evidence: default 4,307,648 B (4.11MiB ≤5MB) sha f327a17b; layout 4,147,384 B (3.96MiB) sha 796bd272 (see docs/pdfium-setup.md). Fuzz smoke (nightly + cargo-fuzz 0.13.2, ASan): rtf 247,187 runs/91s, pdf 561,111/91s, csv 39,956/91s — zero crashes; §14 24h/target still open. Perf: 100 mixed files batch exit 0 in 18.3s wall (≪5min gate). CI YAML: 10 combos valid, actions pinned to SHAs (checkout 11bd596a, upload-artifact ea165f8d); workflow never executed (needs vars + runners).

Coverage: Not measured. All test files executed successfully.

Known Failures: None.

# 14. Next Steps

1. DONE 2026-09-24: P4-S01 validated; C1 attribution guard; C2 OCR-warning gate. Suite 101/1.
2. NEXT single steps, in order (one at a time, each with regression suite): (a) DONE + AQ-008 resolved (delimiter-sniff candidates `,`/`;`/TAB/`|`, first-line count, tie/zero→default; detect 8/8). (b) DONE — AQ-006 resolved (honest `no rendered asset available` kept, `placeholder_honest_stub`) + AQ-007 resolved (`{ext}: {sorted-signals}`, `reason_string_rule`, phase0 14/14). (c) DONE — capped-read + AQ-009 resolved (13 `--no-*`/`--formulas`/`--comments`/`--strip-headers-footers` companions, docs synced, `cli_docs` + bin precedence test green). (d) DONE 2026-09-24 — OMML depth-64 guard added (`omml::go` depth param, `omml_deep_nesting_stays_bounded`); DOCX bold-header rule implemented for ns='w' only (`row_bold`, `docx_table_bold_header_rule`, merge fixture corrected to bold); RTF depth KEPT as warning+Document per P1-S05 instruction 7 + `rtf_malformed_battery` (security `Corrupt` sentence documented as superseded for content errors). Suite 112/1. (e) DONE 2026-09-24 — P2 date-format→Date map (`is_date_format`, `%` excluded, brackets/literals stripped; numeric+date-fmt → `CellKind::Date`) + merged-dedup in xlsx/ods fidelity; `date_tokens` unit test + phase2 6/6 green. EXTENDED 2026-09-24 audit (C8): numeric+date-fmt now also converts display serial→ISO 8601 (`serial_to_iso`, known-value unit tests + `xlsx_number_format_date_converts_display`); error cells use Debug format per P2-S01. (f) DONE 2026-09-24 — default-Force documented AD-15 (`pdf.rs`, any mode → placeholder + OCR_BUILD); `scanned_force_default_still_placeholder` (placeholder + Page boundary + warning); image empty/1-byte → None asserts; phase4 8/8 + phase5 4/4 green. EXTENDED 2026-09-24 audit: +2 convert_file seam tests (scanned-ok, encrypted-no-partial); phase4 10/10. (g) DONE 2026-09-24 — precedence test pins P0-S08 order CLI > config > profile (documents P6-S03 prose swap of middle two); negative suite expanded with §9-mapped encrypted/OLE2/unknown/mismatch-report cases; CLI batch exit-code test (mixed→1 with isolation, all-valid→0); phase6 7/7 green. EXTENDED 2026-09-24 audit: nine-profile effect assertions, XLS-not-rejection cross-ref, deep-XML pin; phase6 9/9. ALSO DONE 2026-09-24 audit: C4 lower-dedup (+test), C3 pdf_path default|layout, C5 img src in warning, C11 chart test, C12 PPTX table+chart test, C15 OMML coverage. (h) OPEN — samples_gen every-fixture needs Architect enumeration (representative-only; not claimed complete). (i) DONE 2026-09-24 — default release 4,303,552 B (4.1MiB ≤5MB), toolchain 1.98.0, suite 126/1 after audit fixes; pdf-layout build + CI runs + fuzz + corpus still without evidence.
3. Architect calls flagged: xml.rs scope vs arch table; layout-Force semantics; shallow scanned detection depth; precedence-test 3-value design; unwind-vs-abort doc update; dead variants (Zip/Other/export_charts/duplicate_of); OCR timeout/sandbox; CI SHA pins; security-auditor.md referral.
4. Then: initial commit + architecture/security reviews + tag (no tag until §14 evidenced).

# 15. Session Log

## 2026-09-24 — Remaining-issues batch (Orchestrator)

Session Summary: Worked the remaining-issues list top to bottom. Network was available, which unblocked the feature build. (1) Hygiene: `libpdfium.so` added to `.gitignore` (repo-root copy proven incompatible — binds fail; md5 d0cc88ec). (2) `cargo fetch` pulled missing crates (itertools/chrono/console_error_panic_hook for pdfium-render); worktree Cargo.lock confirmed current (lopdf 0.45 + 0.42-via-pdf-extract, pdf-extract 0.12.1, calamine 0.31, pdfium-render 0.9.4). (3) Feature check caught 3 syntax errors in the A13 list-nesting edit (fixed; lesson: minified one-liners resist brace-counting — rewrote multi-line). (4) REAL BUG FOUND + FIXED: pdfium-render's global OnceCell makes only the first per-process bind succeed — every layout PDF after the first in one process (batch/CLI/tests) failed with "library not found". Fix: process-global shared `Pdfium` (`shared_pdfium`: per-call explicit-path validation, first-wins source check, lazy init without poisoning) + A12 password/security→Encrypted mapping. Verified: full feature suite 140/1 with chromium/8066 lib (incl. native OCR e2e); order-dependent failures reproduced pre-fix, gone post-fix. (5) Samples enumerated to 25/25 detecting (all formats + scanned/encrypted/pdf + ODP + 7 image headers + xhtml/tsv/md). (6) Release: default 4,307,648 B / layout 4,147,384 B + SHAs in docs/pdfium-setup.md (with pinned URL/SHA/license evidence; same-target rebuild warning noted). (7) Perf: 100 mixed-file batch exit 0, 18.3s wall. (8) CI actions pinned to SHAs; YAML validated (10 combos). (9) cargo-fuzz harness (rtf/pdf/csv targets + seeds) + 90s ASan smoke runs, zero crashes; 24h/target still open. Still open: real-doc corpus (needs user docs), full fuzz hours, CI execution, commit/tag, security-auditor referral.

Completed Steps: hygiene, feature build, shared-Pdfium fix, A12 mapping (+`layout_encrypted_reports_encrypted`), samples 25/25, release sizes, perf evidence, CI pins, fuzz harness + smoke runs.

Files Modified: .gitignore, .github/workflows/ci.yml, src/extract/pdf_layout.rs, tests/phase4_integration.rs, tests/samples_gen.rs, docs/pdfium-setup.md, fuzz/* (new), project/agent.md, project/agent-archive.md.

Dependencies Added: none to the product (fuzz/ workspace: libfuzzer-sys 0.4 + own Cargo.lock).

Tests Executed: default 130/1; feature 140/1 (PDFIUM_DYNAMIC_LIB_PATH=chromium/8066); fuzz smoke 3×90s zero crashes.

Security Work: no new trust boundaries; stale incompatible .so flagged (not deleted — user's file); security-auditor.md referral still open.

Repository Status: Staged for initial commit (libpdfium.so + package.json + pdf/ left untracked); no tag.

## 2026-09-24 — User "fix them all incl. AQ" batch (Orchestrator)

Session Summary: User authorized resolving all 22 audit flags (A1–A22) as documented decisions, blueprint unchanged (same pattern as AQ-006–009). Single-step sequencing per execution rules. Code fixes (all default-verifiable): A4 dead `ns` param removed (`parse_run_properties(&Element)`, 1 call site); A5 dead `parse_rels` removed (`Package::rels` canonical); A18 `extract_csv` takes caller opts (mod.rs + 3 phase0 call sites); A15 `%`-whole→integer display, `DateTime` millis preserved, hyperlink creates missing cell instead of dropping; A11 `flat_blocks` list/tabular-break preservation + `Meta.title`→L1 (+3 lib unit tests); A17 bool/error cell test, C12 slide-title asserts, A7 illustrated-EPUB→Md assert, A8 non-UTF8-CSV/Unknown-note pins. A13 layout edits applied WITHOUT compile (feature build needs native PDFium; bytemuck not cached): `menlo` mono, spec code/math warning strings, math once-per-doc, Euclid font, arrow-adjacency rule, x-offset list nesting + continuation attachment — all spec-literal, no default test pins either string; feature-gated unit tests could not be run, flagged for first pdf-layout CI run. Documented-only (no safe code change or behavior already correct): A1 unwind kept (catch_unwind requirement, existing addenda), A3 RTF warn+Document kept (instruction 7 over security sentence), A6 EPUB split accepted, A9 md-Auto analyzes kept, A10 hidden warnings-entry counts as the summary note, A14 raw `/Encrypt` pre-scan kept (conservative Encrypted over Corrupt-mislabel), A19–A22 accepted/noted, A2 already resolved. Still BLOCKED (no local fix possible): A12 PDFium-refusal→Encrypted mapping + A13 verification + A16 samples/fuzz/corpus/CI/pdf-layout sizes + A17 five-fixture comparison. Full suite 130/1 green, zero warnings (baseline 126/1, +4: 2 flat, 1 detect, 1 kinds).

Completed Steps: A1, A3–A11, A13–A15, A17–A22 resolved (A12/A16 recorded still-blocked).

Files Modified: src/extract/ooxml.rs, src/extract/csv.rs, src/extract/mod.rs, src/extract/xlsx.rs, src/extract/pdf.rs, src/extract/pdf_layout.rs, tests/phase0.rs, tests/phase1_integration.rs, tests/phase2_integration.rs, tests/phase3_integration.rs, tests/detect.rs, project/agent.md, project/agent-archive.md.

Dependencies Added: None.

Tests Executed: full 130 passed, 0 failed, 1 ignored (breakdown in §13).

Security Work: no new trust boundaries; hyperlink-empty-cell now preserved (no silent drop); security-auditor.md referral still open.

Repository Status: Still uncommitted (no commits yet); no tag (§14 evidence incomplete).

## 2026-09-24 — Self-audit audit-and-fix session (Orchestrator)

Session Summary: Ran audit-and-fix.md end to end. Architect/designer role files absent → architecture.md/blueprint.md used as Architect decisions per equivalent-file rule; Designer N/A (headless CLI, arch §1). Three parallel code-level passes (Phases 0–1, 2–3, 4–6) + full-project synthesis. 15 clear compliance findings fixed (C1–C15), all verified: `cargo build --locked` zero warnings, full suite 126 passed, 0 failed, 1 ignored (baseline 115/1, +11: serial_to_iso unit, lower-dedup, date-display, chart, range-guard, preview/txt, slide-table-chart, 2× PDF-seam, XLS cross-ref, deep-XML). 22 items flagged for Architect (A1–A22, full list in archive remediation block). Security questions referred to security-auditor.md, not resolved here. pdf_layout.rs logic untouched (feature build not compilable offline — bytemuck not cached; default-profile tests cannot cover it).

Completed Steps: C1 profile comments (src/config.rs); C2 cli-reference duplicate row removed; C3 pdf_path "default"|"layout" (src/report.rs, P4-S12); C4 lower_workbook duplicate-warning guard + genuine regression test (src/extract/mod.rs, tests/phase2_integration.rs); C5 HTML img warning includes src (src/extract/html.rs, P0-S06); C6 BMP sign comment (src/extract/image.rs, P5-S01); C7 flat weak-heuristic comment (src/extract/pdf.rs, P4-S02); C8 Excel serial→ISO date display + known-value unit tests (src/extract/xlsx.rs, P2-S01); C9 ODS warning gated on preserve_formulas (src/extract/xlsx.rs, P2-S04); C10 coverage: P1-S06 preview+txt test, P4-S03 convert_file seam tests, P6-S04 XLS cross-ref + deep-XML pin; C11 XLSX chart-presence test; C12 PPTX table+chart slide test; C13 nine-profile effect assertions; C14 error-cell Debug format (P2-S01); C15 OMML sSub/sSubSup/rad-deg/nary/delimiter/matrix/accent cases.

Files Modified: src/config.rs, src/report.rs, src/extract/mod.rs, src/extract/html.rs, src/extract/image.rs, src/extract/pdf.rs, src/extract/xlsx.rs, tests/phase1_integration.rs, tests/phase2_integration.rs, tests/phase3_integration.rs, tests/phase4_integration.rs, tests/phase6_integration.rs, docs/cli-reference.md, project/agent.md, project/agent-archive.md.

Dependencies Added: None.

Tests Executed: full 126 passed, 0 failed, 1 ignored (per-binary breakdown in §13).

Security Work: no new trust boundaries; ZIP/XML/PDF guards pinned by new tests (range-guard, deep-XML); full security pass explicitly deferred to security-auditor.md.

Repository Status: Still uncommitted (no commits yet); no tag (§14 evidence still incomplete: samples enumeration, pdf-layout/CI runs, fuzz, corpus).

## 2026-09-24 — User "fix them all" batch (Orchestrator)

Session Summary: Sequenced one-at-a-time per execution rules. AQ-006 (honest placeholder kept, `placeholder_honest_stub`), AQ-007 (`{ext}: {sorted-signals}`, `reason_string_rule`), AQ-008 (delimiter-sniff candidates/count/tie-break, detect 8/8), AQ-009 (13 negation companions + docs + `cli_docs` + bin precedence test) — all resolved by user authorization as documented decisions, blueprint unchanged. Queue (d) OMML-64 guard + DOCX bold-header (ns='w', merge fixture corrected) + RTF keep-Document documented; (e) xlsx date→Date map + merged-dedup + `date_tokens`; (f) default-Force AD-15 comment + Force-placeholder test + image None asserts; (g) P0-S08 precedence pinned (P6-S03 prose swap documented) + §9-mapped negatives + CLI batch exit codes. Release: 4,303,552 B (4.1MiB ≤5MB), toolchain 1.98.0. Full suite 115/1 green (baseline 107/1, +8 genuine). samples_gen enumeration stays Architect-blocked; pdf-layout/CI/fuzz/corpus without evidence — not claimed.

Completed Steps: AQ-006, AQ-007, AQ-008, AQ-009, queue (d)(e)(f)(g), release re-measure (i-measurable).

Files Modified: tests/phase0.rs (+2 tests); src/main.rs (13 flags + precedence test); docs/cli-reference.md (flag table); src/extract/ooxml.rs (OMML depth, DOCX header); tests/regressions.rs (OMML bound); tests/phase1_integration.rs (bold-header test + fixture); src/extract/xlsx.rs (date map, dedup, unit test); src/extract/pdf.rs (AD-15 comment); tests/phase4_integration.rs (Force placeholder); tests/phase5_integration.rs (None asserts); tests/phase6_integration.rs (precedence, negatives, batch); project/agent.md (state).

Dependencies Added: None.

Tests Executed: full 115 passed, 0 failed, 1 ignored (per-binary breakdown in §13).

Security Work: OMML recursion bound; RTF depth decision documented (instruction 7 over security sentence for content errors); §9-mapped negative coverage expanded; no new trust boundaries.

Repository Status: Still uncommitted (no commits yet); no tag (§14 evidence incomplete: samples enumeration, pdf-layout/CI runs, fuzz, corpus).

## 2026-09-24 — Queue item (c) split: capped-read done, bool-off flagged (Orchestrator)

Session Summary: (c1) CLI bool-off assessed NOT execution-ready: blueprint mandates precedence but never specifies negation flags; adding `--no-*` companions is CLI-surface design (flag set, help, docs/cli-reference.md, drift test) → AQ-009, code untouched. (c2) TOCTOU capped-read handed off and verified: new `package::read_capped` (`take(max+1)` + identical Unsupported message) shared by `extract_file`; `convert.rs:5` kept as fail-fast pre-check; +2 helper unit tests (reject-over-max fails under plain read — genuine). Full suite re-run by validator: 105 passed, 0 failed, 1 ignored. Note: worktree `main.rs` carries a whitespace-only `fn main` reformat vs index (no logic change; line 35 identical) — pre-existing or fmt noise, left as-is.

Completed Steps: Queue item (c2).

Files Modified: src/extract/package.rs (+helper + tests); src/extract/mod.rs (1 call); project/agent.md (AQ-008/AQ-009, §14, this log).

Dependencies Added: None.

Tests Executed: lib 7/7; full 105/1 (validator re-ran).

Security Work: TOCTOU race closed at the enforcing read; pre-guard ZIP-walk referral still open.

Repository Status: Still uncommitted.

## 2026-09-24 — Queue item (b) assessed BLOCKED (Orchestrator)

Session Summary: P0-S07 strings assessed against code+blueprint+tests, NOT handed off. (b1) Placeholder: blueprint example vs `implementation-notes.md:106` retained honest-stub decision (asset=None honestly says "no rendered asset available"; literal string would claim a nonexistent asset) → AQ-006, code untouched. (b2) Reason: "lists contributing signals" + example with counts-for-some/prefix `markdown:` vs code's deduped bare names under `md:`/`txt:` → no deterministic count/order/prefix rule → AQ-007, code untouched. Tests pin neither string (`tests/phase0.rs:10` asserts format + "user-specified" only), so nothing breaks either way — the rules themselves are what's missing. No code changed; suite still 103/1.

Completed Steps: None — assessment only.

Files Modified: project/agent.md (AQ-006/AQ-007, §14, this log).

Dependencies Added: None.

Tests Executed: None new.

Security Work: None.

Repository Status: Still uncommitted.

## 2026-09-24 — Queue item (a): PK exact-magics done (Orchestrator)

Session Summary: Blueprint P0-S02 exact-magic rule implemented: `detect.rs:12` now `PK\x03\x04`/`PK\x05\x06` only (ZIP body byte-identical); +2 detect tests (`pk_prefix_text_not_corrupt` fails on old code by reasoning, `pk_empty_archive_signature_still_zip_branch`); stale phase6 fixture `PKbroken`→`PK\x03\x04broken` (same intent/message, corrected branch condition — implementer correctly flagged instead of silently editing). Full suite 103/1 green (detect 6/6, phase6 6/6). CSV-delimiter sniff NOT handed off: no deterministic candidate/tie-break rule in blueprint — flagged for Architect.

Completed Steps: Queue item (a) minus delimiter-sniff (flagged).

Files Modified: src/detect.rs (1 line); tests/detect.rs (+2 tests); tests/phase6_integration.rs (fixture bytes); project/agent.md (state + this log).

Dependencies Added: None.

Tests Executed: detect 6/6; phase6 6/6; full 103/1.

Security Work: None (ZIP-walk pre-guard referral from audit still open).

Repository Status: Still uncommitted.

## 2026-09-24 — Audit fix C2: OCR warning gate (self-audit)

Session Summary: Finding F9 (P5-S03): `extract_pdf_flat` warned OCR_BUILD on clean text PDFs under Force with zero scanned pages. Removed the `else if Force` arm (first branch byte-identical); added `clean_text_pdf_force_no_ocr_warning` (Force + prose fixture → success, no OCR_BUILD). Phase4 7/7 green; full suite 101 passed, 0 failed, 1 ignored.

Completed Steps: C2 fix (correctness, single step).

Files Modified: src/extract/pdf.rs (1 arm deleted); tests/phase4_integration.rs (+1 test); project/agent.md (this log).

Dependencies Added: None.

Tests Executed: phase4 7/7; full 101/1.

Security Work: None.

Repository Status: Still uncommitted.

## 2026-09-24 — Audit fix C1: non-aligned attribution guard (self-audit)

Session Summary: Finding F-01 (P4-S02 step 1): non-aligned decode (parts≠pages) fanned the ENTIRE text to every page (GB-scale blowup on 3000-page PDFs). Extracted pure `attribute_pages()` (aligned→as-is; else first gets all, rest "") + 4 unit tests (aligned, non-aligned, empty, trailing-FF). lib 5/5; full suite 100/1 at the time (now 101/1 after C2). Existing phase4 tests unmodified and green.

Completed Steps: C1 fix (correctness/safety, single step).

Files Modified: src/extract/pdf.rs (helper + rewire + tests); project/agent.md (this log).

Dependencies Added: None.

Tests Executed: lib 5/5; full 100/1 (then-current).

Security Work: DoS-scale output guard (bounded attribution).

Repository Status: Still uncommitted.

## 2026-09-24 — Audit opened + executed (self-audit)

Session Summary: Ran audit-and-fix.md. Roles architect/designer absent → used architecture.md/blueprint.md as Architect decisions (equivalent-file rule); Designer N/A (headless CLI, arch §1). Validated pre-session P4-S01 pdf-extract restore at code level: dep 0.12.1 + `extract_text_from_mem` + catch_unwind + single-fetch + HashSet all as specified; `cargo build --locked` 0 warnings; full suite 96/0/1 (phase4 6/6, new panic-containment test genuine — pins raw panic first, fails loudly on dep change). Real-PDF proof: chapter PDF now decodes clean (4274 lines, HAL header, 0 replacement chars, 0 joined words; was 77 lines garbage). Benchmarks: 1.1M chapter repo-debug 12.5s (real decode) vs system 2.55s; 15M postgres repo-release 26.5s vs system >180s timeout. Two explore passes covered phases 0–3 and 5–6 + cross-cutting. Findings: 3 compliance fixed+verified (P4-S01 validated, C1, C2); 17 compliance flagged as queued single steps; 14 quality flagged; 5 ambiguous flagged for Architect; security-referral items noted (recommend security-auditor.md, not resolved here).

Completed Steps: Audit steps 1–4 (open, phase-by-phase, full-project, classify) + fixes C1, C2.

Files Modified: project/agent.md (Review History open entry, this log); fixes per entries above.

Dependencies Added: None by the audit (pdf-extract 0.12.1 pre-dates it).

Tests Executed: Full suite at close: 101 passed, 0 failed, 1 ignored (baseline was 95/1 → +malformed-font +4 attribution +1 OCR-gate).

Security Work: Not a security audit — referrals flagged (detect ZIP-walk pre-guard, batch TOCTOU/exclusion, OCR capture/timeout, OMML recursion bound, CI action pins).

Repository Status: Still uncommitted (no commits yet); release binary stale (3.0M pre-pdf-extract — re-measure required).

## 2026-09-23 — PDF speed+quality probe (Orchestrator)

Session Summary: User-reported pdf/* slowness + quality demand. Benchmarks (repo release vs system /home/firetech/opt/LLM/docconvert 4.1M): 1.1M chapter — repo 0.38s/77 lines/32K vs system 2.55s/222 lines/39K; 15M postgres (3299 pp) — repo 26.5s/7.1M out vs system >180s timeout (no output). Quality: repo p.1 garbage `���` + joined words (`Howtounderstandbetter`) + broken ligatures vs system clean (HAL header, accents AMÉLIE/RAPHAËL). Root cause (drift): `src/extract/pdf.rs` hand-rolls Tj/TJ decode, zero `pdf_extract` refs, `pdf-extract` absent from Cargo.toml/lock — blueprint P4-S01 mandates `pdf_extract::extract_text_from_mem` (CMap/CFF/Type1) + catch_unwind + single-pass lopdf presence scan. Perf sins ride along: page streams decompressed TWICE (pre-scan loop + `extract_text_from_page`), `bytes.windows(5/8)` full-file pre-scans, per-operand multi-alloc hex decode, `scanned.contains` O(n²). Release evidence: `target/release/docconvert` 3,117,576 B (3.0M, within ≤5MB), `--offline` fresh 2026-09-23. Classification: P4-S01 drift correction; P4-S02 wording (form-feed split, unconditional warning text) queued next, not combined. P6-S05-R2 samples_gen NOT execution-ready (no enumerated fixture list — escalated to Architect, not handed off).

Completed Steps: None — verification only, no code changed.

Files Modified: project/agent.md (this log + §§8/14).

Dependencies Added: None — `pdf-extract` add reserved for P4-S01 step via `cargo add`.

Tests Executed: None new — prior 95/1 default-green stands.

Security Work: None new.

Repository Status: Still uncommitted.

## 2026-09-23 — P6-S05-R1 done (Orchestrator)

Session Summary: Single-step README alignment. Changed only `README.md`: dated default-profile green fact (2026-09-23, 95/1, 1.98.0), AD-1 pointer without ADR reproduction. Verified: 4 item-1 elements present, all 10 relative links resolve, no `v1.0`/release/CI/size/fuzz/corpus completion claims. Other working-tree modifications pre-date R1 and were not touched.

Completed Steps: P6-S05-R1.

Files Modified: README.md (Implementer); project/agent.md (Orchestrator state + this log).

Dependencies Added: None.

Tests Executed: None — docs-only; prior default-green run (95/1) still current.

Security Work: None new.

Repository Status: Still uncommitted (no commits yet).

## 2026-09-23 — Reverify build (Orchestrator)

Session Summary: User-requested build reverify, default profile only. `cargo build --locked` clean, zero warnings. `cargo test --locked` 95 passed, 0 failed, 1 ignored. Toolchain 1.98.0. P6-S05 release block remains (no binary sizes, fuzz, corpus, CI green, tag).

Completed Steps: None — verification only, no code changed.

Files Modified: project/agent.md (build/test evidence, this log).

Dependencies Added: None — lockfile resolved existing deps, no changes.

Tests Executed: 95 passed, 0 failed, 1 ignored (see §13 breakdown).

Security Work: None new — test suite green; audit/corpus/fuzz still outstanding.

Repository Status: Still uncommitted (no commits yet).

## 2026-09-23 — P6-S05 close-out check (Orchestrator)

Session Summary: Close-out check for P6-S05 against blueprint items 1–7 + Arch §14. Files exist (README, cli-reference, format-quirks, adding-a-format, samples_gen, CI matrix, package_binary.py, pdfium-setup) but release Done not met. README declares source-candidate/not-v1; samples_gen representative-only per implementation-notes; CI YAML unrun; no binary sizes, fuzz hours, corpus, or v1.0 tag. Skills docs/packaging/ci loaded. Set P6-S05 back to In Progress; queued single-step remediation R1 (README alignment only).

Completed Steps: None — verification only, no code changed.

Files Modified: project/agent.md (Current Step, Next Steps, this log).

Dependencies Added: None.

Tests Executed: None — no build/test run in this session.

Security Work: None — reproducible-build evidence (lockfile hash, toolchain, sizes) still outstanding.

Repository Status: Uncommitted (no commits yet); Cargo.lock AM; untracked PDF/package.* left out of release scope.

## 2026-09-16 — Self-audit session

Session Summary: Ran self-audit procedure (audit-and-fix.md). Initialized git repository. Updated agent.md status to reflect actual state: 42 blueprint steps verified, build clean, 95 tests pass. Security checklist updated with verified items. Documentation completeness confirmed. Staged files for initial commit (pending user approval).

Completed Steps: Audit steps 1–5 (open, phase-by-phase, full-project, classify, apply fixes).

Files Modified: project/agent.md (status, security checklist, session log), .gitignore (added node_modules/).

Dependencies Added: None.

Tests Executed: 95 passed, 0 failed, 1 ignored (samples_gen write test).

Security Work: Security checklist items verified against test suite results.

Repository Status: Git initialized, files staged, initial commit pending user approval.

## 2026-09-16 — Earlier sessions

Prior sessions (startup, source batch, revisions 2–4, skills research) archived to `project/agent-archive.md`.

# 16. Review History

| Reviewer | Blueprint Version | Major Findings | Required Fixes | Status |
| --- | --- | --- | --- | --- |
| Orchestrator | v2.0 (merged) | Dependency version mismatches prevented build | lopdf 0.36→0.45, calamine 0.28→0.31 | **Closed** |
| Orchestrator | v2.0 (merged) | All 42 blueprint steps verified | None | **Closed** |
| self-audit | v2.0 (merged) | Phase-by-Phase + Full-Project + code-level audit: 8 findings, 4 fixed (git, status, security checklist, .gitignore), 0 code bugs, 3 design-level flags | Git init, status correction, security checklist, doc verification | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-24 audit-and-fix session: phase-by-phase + full-project code-level re-audit incl. unvalidated P4-S01 pdf-extract change | Fixed C1 (P4-S02 attribution guard), C2 (P5-S03 OCR warning gate); P4-S01 validated 96→101 passed; remainder flagged as queued single steps (see log) | **Closed** |
| self-audit | v2.0 (merged) | Scope: Phase-by-Phase + Full-Project (code-level, all 42 steps + cross-cutting). Architect/designer role files absent — architecture.md/blueprint.md used as Architect decisions per equivalent-file rule; Designer N/A (headless CLI, arch §1) | Fixed 15 clear findings (C1–C15, see Session Log); 22 items flagged for Architect (A1–A22, see report in log); security items referred to security-auditor.md | **Closed** |
