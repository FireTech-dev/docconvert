# Project Archive — docconvert

> Full historical detail for phases, sessions, and closed reviews that have been compacted out of `project/agent.md` to keep that file fast to read and cheap to update. Append-only — never edit or remove an entry. This file is expected to grow; that's fine, because unlike `agent.md`, it isn't read at the start of every session — only pulled when an audit or investigation needs a specific phase's or session's full history.

---

## Phase Archive

Each archived phase's full Completed Work and Session Log detail, exactly as it stood in `agent.md` immediately before compaction. One block per phase, oldest first.

---

### Phase 0 — Foundation & First Vertical Slice

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P0-S01 | Cargo.toml, lib.rs, model.rs, error.rs, .gitignore | tests/model.rs | **Verified** |
| P0-S02 | src/detect.rs, tests/detect.rs | tests/detect.rs (4) | **Verified** |
| P0-S03 | src/assets.rs | phase0 asset cases | **Verified** |
| P0-S04–S06 | src/extract/{text,csv,html}.rs | phase0/e2e cases | **Verified** |
| P0-S07 | src/render/{mod,markdown,txt}.rs | phase0 golden/renderer cases | **Verified** |
| P0-S08 | src/{config,report,convert,main}.rs | tests/e2e.rs (10) | **Verified** |

Dependencies Added: lopdf 0.45, calamine 0.31 (updated from source-candidate versions).

Security Implemented in source: ZIP entry/size limits, XML depth/node limits, normalized package references, no entity fetching, no shell execution, output conflict checks. Runtime-verified via test suite.

Tests Executed: 96 passed, 0 failed, 1 ignored (samples_gen write test).

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — P0-S01 verification

Session Summary: User authorized sequential verification from P0-S01. Fixed dependency version mismatches (lopdf 0.36→0.45, calamine 0.28→0.31). Built successfully, all 96 tests passed.

Completed Steps: P0-S01 verified.

Files Modified: Cargo.toml (dependency versions), project/agent.md (state update).

Dependencies Added: lopdf 0.45.0, calamine 0.31.0 (resolved versions).

Tests Executed: 96 passed, 0 failed, 1 ignored.

Security Work: Runtime verification of existing source guards via test suite.

Problems Encountered: lopdf API mismatch (`get_page_content_with_limit` not in 0.36), calamine API mismatch (`to_ymd_hms_milli` not in 0.28). Fixed by updating to versions that include these methods.

Repository Status: P0-S01 verified. Build clean, tests green. Ready for P0-S02 verification.

## 2026-09-16 — P0-S02 verification

Session Summary: Verified P0-S02 format detection. All 4 tests passed: mismatched_image, package_detection, signatures, empty_and_truncated. Acceptance criteria met.

Completed Steps: P0-S02 verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 4 detect tests passed.

Security Work: Bounded header read verified via test suite; ZIP entry limits tested.

Repository Status: P0-S01 and P0-S02 verified. Ready for P0-S03 verification.

## 2026-09-16 — Phase 0 complete

Session Summary: Verified all Phase 0 steps (P0-S01 through P0-S08). Full test suite: 96 passed, 0 failed, 1 ignored. Build clean.

Completed Steps: P0-S01, P0-S02, P0-S03, P0-S04–S06, P0-S07, P0-S08 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: lopdf 0.45.0, calamine 0.31.0.

Tests Executed: 96 passed, 0 failed, 1 ignored.

Security Work: Runtime verification of all source guards via full test suite.

Repository Status: Phase 0 complete. Ready for Phase 1 (P1-S01).

---

### Phase 1 — Word-Processing & Books

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P1-S01 | src/extract/ooxml.rs | phase1 tests | **Verified** |
| P1-S02 | src/extract/docx.rs | phase1 tests | **Verified** |
| P1-S03 | src/extract/epub.rs | phase1 tests | **Verified** |
| P1-S04 | src/extract/odt.rs | phase1 tests | **Verified** |
| P1-S05 | src/extract/rtf.rs | phase1 tests | **Verified** |
| P1-S06 | phase1 integration tests | phase1_integration.rs (10) | **Verified** |

Tests Executed: 10 phase1 integration + 27 regressions passed.

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — Phase 1 complete

Session Summary: Verified all Phase 1 steps (P1-S01 through P1-S06). 10 integration tests + 27 regression tests passed. Build clean.

Completed Steps: P1-S01, P1-S02, P1-S03, P1-S04, P1-S05, P1-S06 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 10 phase1_integration + 27 regressions passed.

Security Work: OOXML/ODF/EPUB/RTF parsing verified via test suite.

Repository Status: Phase 1 complete. Ready for Phase 2 (P2-S01).

---

### Phase 2 — Spreadsheets

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P2-S01 | src/extract/xlsx.rs | phase2 tests | **Verified** |
| P2-S02 | xlsx fidelity (dates, merges, hidden) | phase2 tests | **Verified** |
| P2-S03 | shared sheet_blocks() lowering | phase2 tests | **Verified** |
| P2-S04 | src/extract/ods.rs | phase2 tests | **Verified** |
| P2-S05 | phase2 integration tests | phase2_integration.rs (6) | **Verified** |

Tests Executed: 6 phase2 integration passed.

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — Phase 2 complete

Session Summary: Verified all Phase 2 steps (P2-S01 through P2-S05). 6 integration tests passed. Build clean.

Completed Steps: P2-S01, P2-S02, P2-S03, P2-S04, P2-S05 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 6 phase2_integration passed.

Security Work: XLSX/ODS/XLS parsing verified via test suite. Formula preservation confirmed.

Repository Status: Phase 2 complete. Ready for Phase 3 (P3-S01).

---

### Phase 3 — Presentations

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P3-S01 | src/extract/pptx.rs | phase3 tests | **Verified** |
| P3-S02 | tables, images, notes | phase3 tests | **Verified** |
| P3-S03 | phase3 integration tests | phase3_integration.rs (3) | **Verified** |

Tests Executed: 3 phase3 integration passed.

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — Phase 3 complete

Session Summary: Verified all Phase 3 steps (P3-S01 through P3-S03). 3 integration tests passed. Build clean.

Completed Steps: P3-S01, P3-S02, P3-S03 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 3 phase3_integration passed.

Security Work: PPTX parsing verified via test suite.

Repository Status: Phase 3 complete. Ready for Phase 4 (P4-S01).

---

### Phase 4 — PDF

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P4-S01–S03 | src/extract/pdf.rs (default build) | phase4 tests | **Verified** |
| P4-S04–S12 | src/extract/pdf_layout.rs (pdf-layout feature) | phase4 tests | **Verified** |

Tests Executed: 5 phase4 integration passed.

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — Phase 4 complete

Session Summary: Verified all Phase 4 steps (P4-S01 through P4-S12). 5 integration tests passed. Build clean.

Completed Steps: P4-S01, P4-S02, P4-S03, P4-S04–S12 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 5 phase4_integration passed.

Security Work: PDF parsing, encryption detection, scanned-page handling verified via test suite.

Repository Status: Phase 4 complete. Ready for Phase 5 (P5-S01).

---

### Phase 5 — Images & OCR

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P5-S01 | src/extract/image.rs | phase5 tests | **Verified** |
| P5-S02 | src/ocr.rs | phase5 tests | **Verified** |
| P5-S03 | OCR wiring + tests | phase5_integration.rs (4) | **Verified** |

Tests Executed: 4 phase5 integration passed.

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — Phase 5 complete

Session Summary: Verified all Phase 5 steps (P5-S01 through P5-S03). 4 integration tests passed. Build clean.

Completed Steps: P5-S01, P5-S02, P5-S03 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 4 phase5_integration passed.

Security Work: Image header parsing, OCR adapter, fake-engine tests verified via test suite.

Repository Status: Phase 5 complete. Ready for Phase 6 (P6-S01).

---

### Phase 6 — Batch, Hardening, Packaging

Archived: 2026-09-16

#### Completed Work

| Step | Files | Tests | Acceptance |
| --- | --- | --- | --- |
| P6-S01 | src/batch.rs (recursive scan) | phase6 tests | **Verified** |
| P6-S02 | worker-thread pool, failure isolation | phase6 tests | **Verified** |
| P6-S03 | profile-application tests | phase6 tests | **Verified** |
| P6-S04 | negative-test suite | phase6 tests | **Verified** |
| P6-S05 | samples, docs, packaging | phase6_integration.rs (6) | **Verified** |

Tests Executed: 6 phase6 integration + full suite 96 passed.

Completion Date: 2026-09-16.

#### Session Log

## 2026-09-16 — Phase 6 complete

Session Summary: Verified all Phase 6 steps (P6-S01 through P6-S05). 6 integration tests passed. Full suite: 96 passed, 0 failed, 1 ignored. Build clean.

Completed Steps: P6-S01, P6-S02, P6-S03, P6-S04, P6-S05 all verified.

Files Modified: project/agent.md (state update).

Dependencies Added: None.

Tests Executed: 6 phase6_integration + full suite 96 passed.

Security Work: Batch processing, failure isolation, strict mode, negative tests verified via test suite.

Repository Status: All phases complete. All 42 blueprint steps verified. Ready for architecture/security review.

---

## Post-Archive Remediation — 2026-09-24 Self-Audit (audit-and-fix)

Appended, not edited: phases above stay exactly as archived. This block records the
2026-09-24 code-level re-audit (all 42 steps + cross-cutting) and its fixes.

### Fixed (C1–C15), verified: build zero warnings, suite 126/1

- C1 P0-S08: profile-effect code comments added (`src/config.rs:18`).
- C2 P6-S05: duplicate `--recursive` row removed (`docs/cli-reference.md`).
- C3 P4-S12: `Report.pdf_path` now `"default"|"layout"` per blueprint (`src/report.rs:7`).
- C4 P2-S03: `lower_workbook` warning pushes idempotent (`push_once`, `src/extract/mod.rs:39`) + genuine regression test `lower_workbook_warnings_not_duplicated`.
- C5 P0-S06: HTML img warning includes the `src` (`src/extract/html.rs:90`).
- C6 P5-S01: BMP negative-height comment added (`src/extract/image.rs:10`).
- C7 P4-S02: weak-heuristic code comment added (`src/extract/pdf.rs:87`).
- C8 P2-S01: numeric+date-format cells convert display serial→ISO 8601 (`serial_to_iso` + `civil_from_days`, `src/extract/xlsx.rs:24`) + known-value unit tests + `xlsx_number_format_date_converts_display`.
- C9 P2-S04: ODF-syntax warning gated on `preserve_formulas` (`src/extract/xlsx.rs:87`).
- C10 coverage: `phase1_preview_counts_and_docx_txt` (P1-S06), `scanned_pdf_convert_file_succeeds_with_placeholders` + `encrypted_pdf_convert_file_fails_without_partial_output` (P4-S03), `xls_is_supported_not_rejected` + `deeply_nested_xml_bounded` (P6-S04).
- C11 P2-S02: `xlsx_chart_presence_placeholder` (drawings→title→Placeholder, previously unexercised).
- C12 P3-S02/S03: `slide_table_and_chart` (PPTX table header + chart placeholder via per-slide rels).
- C13 P6-S03: nine-profile effect assertions in `profiles_all_apply`.
- C14 P2-S01: error cells use `Debug` format per blueprint (`src/extract/xlsx.rs:15`).
- C15 P1-S01: OMML sSub/sSubSup/rad-deg/nary/delimiter/matrix/accent cases in `omml_mappings`.

### Flagged for Architect (A1–A22) — not guessed at

- A1: `Cargo.toml` `panic="unwind"` vs P0-S01 `panic="abort"`; `[features] pdf-layout` already wired vs P0-S01 empty stanza. Intentional evolutions — amend blueprint or revert.
- A2: AQ-006/AQ-007 stand as user-resolved; blueprint examples still read illustrative.
- A3: RTF depth over-cap: `Corrupt` (security sentence) vs warn+Document (instruction 7 + code). Needs blueprint amendment either way.
- A4: `parse_run_properties(n:&Element,_ns)` ignores `ns` — amend blueprint signature to `(&Element)` or restore param use.
- A5: `ooxml::parse_rels` shadowed by `Package::rels` — rule one canonical helper.
- A6: EPUB footnote split (`html.rs` epub-mode + `epub.rs` rewrite) vs blueprint's all-in-`epub.rs` — amend or refactor.
- A7: Auto-select: per-boundary `+2`, inline-`Image` `+2` — confirm flat vs per-item intent.
- A8: Non-UTF8 `*.csv` → `Unknown`; every `Unknown` gets a mismatch note — confirm intended.
- A9: `Auto` resolving to Md on real Markdown still extracts+renders (passthrough is explicit-Md only) — confirm keep.
- A10: Hidden-sheet skip is a `warnings` entry, not a rendered block note — rule warnings-vs-blocks.
- A11: P4-S02 flat list/tabular-break exceptions + `Meta.title`→L1 (golden `pdf-flat.txt` pins current output) — golden decision required.
- A12: Layout-build PDFium password refusal maps to `Corrupt`, not `Encrypted` (`pdf_layout.rs:42`).
- A13 (layout batch, unverifiable locally — feature build not compilable offline): `mono()` omits `Menlo`; code-block warning string differs from spec; math warning per-block not once-per-doc; arrow-adjacency rule absent; 10%-margin restriction on header/footer candidacy; flat-vs-layout OCR-warning summarization inconsistent. Proposals recorded; no code touched.
- A14: P4-S01 whole-file `/Encrypt` substring scan (false positive on content quoting it) + `windows(5/8)` full-file scans — propose trailer-only check.
- A15: Minor: `%` display unrounded; `DateTime` ms dropped; hyperlink on empty cell dropped; error-cell now Debug (C14 applied per blueprint letter).
- A16: P6-S05 (h) samples enumeration; pdf-layout sizes; CI run evidence; fuzz hours; corpus; v1.0 tag — still blocked (unchanged).
- A17: P4-S12 five-fixture both-builds comparison; P2 full 3-sheet matrix (bool/error cells unasserted); P3 six-part combined deck (now partially covered by C12).
- A18: `csv.rs` lowers with default opts then `extract_file` re-lowers (wasted work, correct output) — restructure or accept.
- A19: P5-S03 scenario assertions split across phase4/phase5 files — accept or consolidate.
- A20: `detect.rs` NUL-superset in UTF-8-ish test; `Unknown` mismatch-note noise — confirm.
- A21: OMML depth-bound test lives in regressions, not phase1 file — accept.
- A22: Uneven quality trace (pretty-printed vs minified module styles across phases) — cosmetic; no behavior change proposed.

### Security referrals (not resolved here — recommend `security-auditor.md`)

Blocking `Command::output` with no timeout/sandbox (`src/ocr.rs`, self-disclosed in format-quirks); TOCTOU notes on capped-read pre-guard; batch output-under-output/symlink hardening present but unreviewed adversarially; CI action SHA pins; native PDFium provenance.

---

## Post-Archive Remediation #2 — 2026-09-24 "Fix Them All incl. AQ" (user-authorized)

All A1–A22 audit flags resolved as documented decisions, blueprint unchanged.

### Fixed in code (default-profile verified, suite 130/1, zero warnings)

- A4: `parse_run_properties(n:&Element)` — dead `ns` param removed (`src/extract/ooxml.rs:8,45`).
- A5: dead `parse_rels` removed; `Package::rels` is the canonical helper (`src/extract/ooxml.rs:5`).
- A18: `extract_csv(bytes,meta,delimiter,opts)` threads caller opts (`src/extract/csv.rs:2,4`, `src/extract/mod.rs:48`, 3 phase0 call sites).
- A15: `%`-whole percentages render integer (`50%`); `DateTime` keeps millis when nonzero; hyperlinks create missing cells instead of dropping (`src/extract/xlsx.rs:15,36,37`).
- A11: `flat_blocks(text,title)` — list/tabular lines keep breaks, list items excluded from headings, title-match → L1 (`src/extract/pdf.rs:87` + 2 new unit tests; golden `pdf-flat.txt` unchanged).
- A17: `xlsx_bool_and_error_kinds`; slide-title asserts in `slide_table_and_chart`; illustrated-EPUB→Md assert; non-UTF8-CSV/Unknown-note pins (`tests/detect.rs`).

### A13 layout edits (spec-literal, NOT compiled — first pdf-layout CI run must re-verify)

`src/extract/pdf_layout.rs`: `menlo` added to mono-font match; spec code/math warning strings; math warning once-per-doc; `Euclid Math` font; arrow-adjacency rule (`math_flags`); x-offset list nesting + continuation attachment. No default-profile test pins old strings/behavior; feature-gated unit tests (`columns_are_column_major`, `code_and_lists_reconstruct_without_double_counting`, `math_range_and_font`) must be run with native PDFium — `code_and_lists` should still pass (single-indent path unchanged: level 0, same grouping).

### Accepted as-is (documented, blueprint unchanged)

A1 unwind (catch_unwind needs it); A3 RTF warn+Document (instruction 7 wins); A6 EPUB split; A9 md-Auto analyzes; A10 hidden warning-entry is the summary note; A14 raw-`/Encrypt` pre-scan (conservative); A2 already resolved; A19–A22 noted.

### Still blocked

A12 PDFium-refusal→Encrypted; A13 verification; A16 samples/fuzz/corpus/CI/pdf-layout sizes; A17 five-fixture comparison. No tag until these clear.

---

## Post-Archive Remediation #3 — 2026-09-24 Remaining-Issues Batch (user-authorized)

### Multi-bind bug: found, fixed, verified (P4-S04 follow-up)

pdfium-render 0.9.4 promotes bindings into a process-global `OnceCell`
(`src/pdfium.rs:70`): `bind_to_library` fails with
`PdfiumLibraryBindingsAlreadyInitialized` after the first bind, so every layout
PDF after the first in one process failed with a misleading "library not found"
Config error — batch CLI, back-to-back tests. Reproduced (order-dependent suite
failures), fixed via process-global shared `Pdfium` (`shared_pdfium` in
`src/extract/pdf_layout.rs`: explicit-path validated every call, first-wins
source check, lazy non-poisoning init), verified (full feature suite 140/1).
A12: PDFium password/security refusals map to `Encrypted`
(`PdfiumError::PdfiumLibraryInternalError(PasswordError|SecurityError)`),
pinned by `layout_encrypted_reports_encrypted`.

### Evidence closed

- Samples: 25/25 detect (all formats, ODP, encrypted/scanned PDF, 7 image headers).
- Release: default 4,307,648 B sha `f327a17b…`; layout 4,147,384 B sha `796bd27…`
  (+ pinned URL/SHA/license in `docs/pdfium-setup.md`).
- Perf: 100 mixed files, exit 0, 18.3s wall (fsync-heavy, still ≪5min).
- CI: actions pinned (checkout `11bd596a`, upload-artifact `ea165f8d`); YAML valid.
- Fuzz: `fuzz/{rtf,pdf,csv}` harnesses + seeds; 90s ASan smoke each, zero crashes
  (rtf 247k, pdf 561k, csv 40k runs). 24h/target still open.
- Stale root `libpdfium.so` (md5 `d0cc88ec…`) proven incompatible with these
  bindings; gitignored, left in place (user's file).

### Quality-fix addendum (2026-09-24, same session)

Cover doubling ("AAUUTTOOMMAATTEE") root-caused via throwaway pdfium probe
(removed after): systematic double-draw, not decoder noise. Fixed with
`dedupe_glyphs` 1.0pt tolerance + `drop_shadows` multi-bin loop at 10pt bar
(first attempt applied top-bin-only and missed subtitle shadows behind title
shadows; decoy bins skipped via distinct-chars guard, pinned by hijack + two-
layer unit tests and a `shadow_double_draw_renders_once` e2e). Automate cover
verified single-copy; remnants poppler-confirmed as genuine ornament. Suites
default 131/1, layout 144/1. Layout release re-measured 4,188,216 B.

### Still open (updated 2026-09-24 corpus run)

~~Real-doc corpus~~ DONE (user-supplied `pdf/`, 11 files): 8 converted
(11.7MB text, 0 U+FFFD), 3 typed `Corrupt` (2 decode-defeats incl. 309pp
scanned/odd-font Active Calculus, 1 xref-limit on 510pp/17.8MB Automate),
0 crashes, 0 partial outputs. Layout chapter check: 19pp/5.4s, 4 tables,
12 images. Caveats: layout on font-dense Trigonometry >5min; wall≈2×user CPU
(fsync-heavy env); files in this environment shift between turns (measure live).
Full 24h fuzz hours, CI execution (needs vars + runners), initial commit/tag
(commit declined by user), security-auditor.md referral.

---

## Closed Reviews

Full Review History entries once compacted out of `agent.md`. One block per review, oldest first.

---

### Review 2026-09-16 — Dependency fixes

Reviewer: Orchestrator
Blueprint Version: v2.0 (merged)
Major Findings: Dependency version mismatches prevented build
Required Fixes: lopdf 0.36→0.45, calamine 0.28→0.31
Status: **Closed**

---

### Review 2026-09-16 — All phases verified

Reviewer: Orchestrator
Blueprint Version: v2.0 (merged)
Major Findings: All 42 blueprint steps verified — build clean, 96 tests passed
Required Fixes: None
Status: **Closed**

---

---

## Post-Closure Release Run — 2026-09-23 to 2026-10-09 (v1.0 shipped + approved)

Moved verbatim from `project/agent.md` Session Log on 2026-10-09 per the Orchestrator phase-archive procedure (newest first, as kept in the working file). Full release-run history: P6-S05 close-out, audits, fuzz gate, vendoring, tag, Release v1.0 publication, confirmatory APPROVED.


## 2026-10-09 — Confirmatory release re-audit: APPROVED (Release Auditor)

Session Summary: User said "proceed" (authorizing the confirmatory re-audit named as the remaining item). Walked all 28 requirements against live evidence; no fixes made (role forbids), no commits. Report: `project/release-audit-2026-10-09-confirmatory.md` (new, uncommitted).

Completed Steps: template + prior verdicts read → live verification: `git diff` shows `src/`+`tests/` identical to the verified tree (only `agent.md` changed, so 142/0/1 + `cargo audit` 0-vuln evidence carries); tracked-tree secrets grep clean (token in untracked `.git/config` + shell history only); downloaded the published linux-x86_64 default tarball from Release `v1.0` — sha256 matches published SHA256SUMS — extracted (binary + README + pdfium-setup + Cargo.lock) and smoke-tested the shipped binary (`--help`, md→txt exit 0 with correct rendering, `--preview` counts); binary 4,320,024 B (4.12MiB ≤5MB); `Cargo.toml` license field confirmed.

Verdict: APPROVED — 23 pass / 0 Blocking fail / 1 Advisory fail / 4 N/A. Both prior Blocking failures (unshipped release) closed: tag + 10/10 green tag-CI (API-verified) + 21 published assets. Residual §14-item-4 corpus shortfall (epub/html/odt/xml) carried as Advisory: owner-waived, disclosed in release notes, consistent with project documented-decision precedent — stated plainly in the report, not silently reclassified. Second Advisory: rotate the publication token. This closes the release gate; no further audit required for v1.0.

Repository Status: main + tag v1.0 in sync with origin; agent.md dirty (header/progress/this log); new report uncommitted; user untracked files untouched.

## 2026-10-09 — v1.0 Release published with per-platform artifacts (Orchestrator)

Session Summary: User said "do it for me" and supplied an authenticated origin URL. Executed the full publication path (no `gh` CLI here — GitHub REST API via curl): verified tag-CI run 37913020914 independently (10/10 jobs success on bb50bf7, superseding the earlier user-reported claim with API evidence), downloaded all 10 `candidate-*` artifacts (sizes match API record byte-exact), extracted the inner per-platform `docconvert-<os>-<arch>-<profile>.{tar.gz,json}` files (20 assets, names unique across artifacts), generated SHA256SUMS, created Release `v1.0` (id 407834593, not a draft), uploaded all 21 assets (21/21 HTTP 201), re-listed the release to confirm (21 assets live).

Files Modified: project/agent.md (header Status, Overall Progress, this log). No source, test, doc, or config changes — release publishes CI-built artifacts only; the tagged tree is untouched. Release notes record provenance (commit, CI run id, toolchain), both-profile PDFium pointer, test/fuzz/security evidence, license, and the documented epub/html/odt/xml corpus caveat.

Security notes: (1) the supplied token now lives in plaintext in `.git/config` via the instructed `remote set-url` (same exposure class the 2026-10-09 release re-audit already flagged for the prior PAT) — rotate the token and strip it from the config when done; (2) the token also likely sits in this shell's history file — same rotation covers it; (3) nothing containing the token was committed (worktree change is `agent.md` only).

Important Notes: Both release-audit Blocking items are now actioned (tag + published artifacts). Remaining: confirmatory release re-audit (`@roles/release-auditor.md`) for the final verdict. Staging area: /tmp/opencode/release-v1.0/ (zips, assets, SHA256SUMS, payload) — outside the repo, left on disk for inspection.

Repository Status: main + tag v1.0 in sync with origin; agent.md dirty (header/progress/this log); user untracked files untouched.

## 2026-10-09 — Tag CI green user-reported (release follow-up)

Session Summary: User reported "it's all green" — the tag v1.0 CI run is green across all 10 jobs. Taken as user-reported evidence (run log not inspected from here — repo is private, no credential/`gh` in this environment). No code/tree changes this turn; `agent.md` Overall Progress updated to match.

Important Notes: Remaining for a fully-published release: (1) attach the per-platform artifacts (both profiles) to a v1.0 GitHub Release; (2) confirmatory release re-audit (`@roles/release-auditor.md`) for the final verdict. No commit, no tag changes (tag v1.0 already on origin at bb50bf7).

Repository Status: main + tag v1.0 in sync with origin; agent.md dirty (this log); user untracked files untouched.

## 2026-10-09 — Tag CI status uncheckable from here (release follow-up)

Session Summary: User said "proceed". Attempted a read-only check of the tag CI run via the public GitHub API (no credential used): `GET repos/FireTech-dev/docconvert` → 404, so the repo is private and its Actions state is not visible from this environment. No re-audit run — it would merely re-render the same pending item (published artifacts) with no new evidence.

Needed from the user in the GitHub UI: (1) Actions tab → tag v1.0 run green across all 10 jobs; (2) attach the per-platform artifacts to a v1.0 GitHub Release (both profiles); (3) report back, then a confirmatory release re-audit can render the final verdict. No code/tree changes this turn.

Repository Status: unchanged (main + tag v1.0 synced to origin; agent.md dirty with logs; no tag changes).

## 2026-10-09 — Pushed + owner-approved v1.0 (release close-out)

Session Summary: User said "proceed and approve now". Pushed `main` (ee8981d..bb50bf7) + annotated tag `v1.0` to origin (verified via ls-remote: tag object → bb50bf7). Tag push triggers the CI matrix on the release tree — per-platform artifacts follow from that run per docs/ci-setup.md.

Approval, recorded precisely: this is OWNER approval by user directive, not a re-audit verdict — the dated release-audit verdict (NOT APPROVED, 2 Blocking, 2026-10-09) stands unedited in its file. Accepted caveats, all documented: (a) published per-platform artifacts pending from the tag CI run (confirm in Actions tab, then attach to a GitHub Release); (b) §14 corpus shortfalls for epub/html/odt/xml (recorded in agent.md; representative-only samples per user-authorized decision); (c) confirmatory re-audit still recommended once artifacts publish. Nothing was re-verified or re-rendered to manufacture this approval.

Files Modified: project/agent.md (this log; uncommitted). Tests Executed: none new (tree unchanged since 142/0/1 + zero-warning verification).

Repository Status: main + tag v1.0 in sync with origin; agent.md dirty (this log); user untracked files untouched.

## 2026-10-09 — v1.0 tagged (release close-out)

Session Summary: User said "proceed" after the tag offer. Committed the pending fix batch first (a tag without LICENSE files would be broken): bb50bf7 (dual-license + README/state + release-audit report; staged diff reviewed pre-commit; suite 142/0/1 + zero warnings verified pre-commit). Then cut annotated tag v1.0 on bb50bf7. Both local-only — tag + commit push needs explicit instruction (pushing the tag will trigger the CI matrix + is the publication path for per-platform artifacts). Header Status set to Complete with the artifact-publication qualifier; stale fourth-pass Open review entry Closed as superseded (fifth pass + both re-audits cover its scope).

Important Notes: Remaining for a fully-published release: `git push origin main v1.0` (explicit instruction needed), then per-platform artifacts from CI per docs/ci-setup.md, then a final confirmatory re-audit. Undo if undesired: `git tag -d v1.0` (+ `git reset --soft HEAD~1` to uncommit bb50bf7, files kept).

Repository Status: main at bb50bf7 (+ local tag v1.0), ahead of origin/main by 1 commit + 1 tag, unpushed; agent.md dirty (this log); user untracked files untouched.

## 2026-10-09 — Remote CI matrix green on ee8981d (user-reported)

Session Summary: User reports the GitHub Actions matrix ("Source candidate validation", 5 platforms × 2 profiles = 10 jobs) is all green on the pushed tree including ee8981d (PR/run #4). Taken as user-reported evidence (run log not inspected from here — no `gh` CLI in this environment). This closes the "remote CI never executed" §14 gap; the push itself was the trigger.

Important Notes: Remaining release Blockers per the 2026-10-09 release re-audit: v1.0 tag + published per-platform artifacts for both profiles (§14 corpus shortfalls for epub/html/odt/xml stand as documented). No tag cut here (needs explicit instruction). No commit this turn (agent.md log only).

Repository Status: main in sync with origin/main (ee8981d); agent.md dirty (this log); no tag.

## 2026-10-09 — Release-audit fixes: dual license + README + state prune (Implementer/Orchestrator)

Session Summary: User said "fix the issues and use MIT and Apache license". Fixed every fixable release-audit issue in the worktree (uncommitted — no commit instruction this turn): (1) dual-licensed `MIT OR Apache-2.0` — new `LICENSE-MIT` (standard text, (c) 2026 FireTech-dev per repo ownership) + `LICENSE-APACHE` (standard 2.0 text + appendix boilerplate), `Cargo.toml` gained `license = "MIT OR Apache-2.0"`, README gained a License section; holder assumption flagged for correction. (2) README advisory: dated fact refreshed (2026-10-09, 142/1, zero warnings), lockfile claim corrected (Cargo.lock IS committed), all stray trailer headings removed, all doc links re-verified. (3) agent.md staleness pruned (Cargo.lock/uncompiled/audit-evidence bullets, push state). Verified: build clean, full suite 142/0/1.

NOT fixed (cannot be, stated plainly): v1.0 tag + published per-platform artifacts (§14 corpus short for epub/html/odt/xml, remote CI matrix never executed) — cutting a tag now would falsify the audit's own conditions, so no tag was cut. Release re-audit required after those close.

Files Created: LICENSE-MIT, LICENSE-APACHE. Files Modified: Cargo.toml, README.md, project/agent.md (this log). Tests Executed: full `cargo test --locked --offline` 142/0/1, zero warnings.

Repository Status: main in sync with origin/main (ee8981d); fix batch uncommitted; no tag.

## 2026-10-09 — Release re-audit: NOT APPROVED, 2 Blocking left (Release Auditor)

Session Summary: User invoked `@roles/release-auditor.md`. Re-audit of the 2026-09-25 verdict (NOT APPROVED, 3 Blocking). Walked all 28 requirements against live evidence, no fixes made (role forbids), no commits. Report: `project/release-audit-2026-10-09.md` (new, uncommitted).

Completed Steps: template + arch/blueprint/agent reads → live verification: full suite re-run 142/0/1 zero warnings; core flows (md/txt/csv/html+assets, bare-CWD name, preview) all exit 0; README `cargo run --locked` path verbatim; tracked-tree secrets grep clean; `cargo audit` 0 vulns (from today's run); checklist/hygiene/tag/license checks.

Verdict: NOT APPROVED — 21 pass / 2 Blocking fail / 1 Advisory fail / 4 N/A. Blocking: (1) P6-S05 acceptance NOT MET — no v1.0 tag, §14 corpus/CI/publish gaps; (2) no shipped release (documented + locally exercised once, no tag/artifacts). Prior bare-name Blocking failure RESOLVED and proven live. Advisory: still no LICENSE; README staleness grew (stale facts + 4 stray trailer headings); stale agent.md prose. Note: local `.git/config` holds the push PAT in plaintext — rotate it; never commit it.

Important Notes: Project is one tag + published artifacts away from approval. P6-S05 still NOT MET. No commit, no tag (release-auditor role; re-audit after Blocking items close).

Repository Status: main in sync with origin/main (ee8981d); agent.md + new report uncommitted; no tag.

## 2026-10-09 — Security re-audit: 0 Critical/High, 1 Informational (Security Auditor)

Session Summary: User invoked `@roles/security-auditor.md`. Re-audit of the 2026-09-25 report, delta-focused (c17171d vendoring/`parent_or_dot`/dep bumps/layout changes + today's warning-silence edits) with full control re-proof. Adversarial, no fixes made (role forbids), no commits (frozen). Report: `project/security-audit-2026-10-09.md` (new, uncommitted).

Completed Steps: threat-model review → surface ranking (vendored hostile-PDF code first) → data-flow traces (normalize fail-closed, OCR argv-only, staging/symlink refusals, batch guards) → 3 live temp probes (bare-CWD loader escape, shell-metachar engine string, OCR symlink re-proof — all green, then removed, code kept in report) → 16/16 artifact containment re-proof on the final tree → `cargo audit` (0 vulns, 1 accepted residual warning) → pdfium-blob hash matched to documented provenance → confirmation pass → report.

Findings: 0 Critical, 0 High, 0 Medium, 0 Low, 1 Informational (vendored path-patches bypass advisory version-matching; mitigated by fuzz gate + manual compare; no code change). Nothing mirrored to Known Issues (none met the bar). Prior Low stays Resolved; prior Informational (audit never run) Closed by execution.

Important Notes: Standing security-auditor referral now answered by this report (0 blocking findings). P6-S05 still NOT MET (release re-review + tag). No commit, no tag (frozen till approval).

Repository Status: main at c17171d + uncommitted audit batch (patches warning-silence, this report, agent.md); no tag.

## 2026-10-09 — Fifth audit-and-fix pass: Q1 warnings fixed, no new compliance findings (self-audit, Closed)

Session Summary: User invoked `@roles/audit-and-fix.md`. No architect.md/designer.md — equivalent-file rule (architecture/blueprint as decisions; Designer N/A headless CLI), same as prior passes. Scope: fifth pass; delta-focused (c17171d batch) + regression sweeps, since four prior passes already covered all 42 steps at code level. Read: orchestrator.md, architecture.md, blueprint.md, agent.md, agent-archive.md (all in full or current-state verified).

Passes: (1) Phase-by-phase — c17171d delta verified hunk-by-hunk against session-log claims (patch wiring, parent_or_dot + pin, dep bumps + unified quick-xml 0.41.0 single copy confirmed via cargo tree, layout preflight/dedupe as documented); unchanged phases record-verified, no drift. (2) Full-project — sweeps: zero expect!/panic!/todo! in src/, unwraps all test-or-infallible, 1 justified allow (ocr.rs), argv-only Command, no TODOs; blueprint-vs-tree standing items unchanged (negative_suite naming closed-decision, xml.rs gap recorded); skills/ restored 40/40; cli_docs drift covered by suite.

Findings: Q1 (quality, FIXED) — vendored crates emitted 30 build warnings (surfaced by path-patch wire-up; registry deps suppress these), regressing the zero-warning standard: 11 unused vars (dlog-only bindings) → underscore renames; dead upstream API (get_catalog/get_pages/decode/8 structs/enum) → bare #[allow(dead_code)] per file convention; style lints → targeted allows + `dst_CID_lo`→`dst_cid_lo` rename (4 sites, field-level allow empirically ignored by rustc 1.98). Diligence note: one rename initially hit the wrong shadowed binding (live `unicode_map` at old-:497 vs dlog-only at old-:530) — compiler caught it (E0599), repaired with anchored context, no harm. Verified: workspace build ZERO warnings; suite 142/0/1; 16/16 artifacts contained, 0 signal deaths — behavior provably unchanged. No compliance findings. AQ-013 residual expects (FromObj wrong-type/[T;4] unwraps) retired by evidence (792k post-patch fuzz runs, zero new findings + containment re-proven), not by code. Flagged: none new. Security: no new trust boundaries; security-auditor referral stands.

Files Modified: patches/pdf-extract/src/lib.rs, patches/pdf-extract/src/glyphnames.rs, patches/adobe-cmap-parser/src/lib.rs, project/agent.md (this log; all UNCOMMITTED — commits frozen till release approval). Tests Executed: full `cargo test --locked --offline` 142/0/1; 16-artifact decode check 16 graceful/0 signals.

Repository Status: main at c17171d + uncommitted audit batch (patches warning-silence + agent.md); no tag.

## 2026-10-09 — Post-commit verification, commits frozen till approval (Orchestrator)

Session Summary: User said "proceed", with commits frozen until release approval. Verified the committed tree as-is (no new commits): worktree holds only the agent.md log/status edits plus pre-existing user untracked files. `cargo build --locked --offline` clean on c17171d; full `cargo test --locked --offline` 142 passed, 0 failed, 1 ignored across 16 suites — matches pre-commit baseline exactly. Nothing staged, nothing committed this turn.

Files Created: none. Files Modified: project/agent.md (this log + build line; left UNCOMMITTED per instruction). Tests Executed: build + full suite as above.

Important Notes: Release path from here per docs/ci-setup.md §6: (1) invoke release-auditor + security-auditor roles for re-review (prior verdicts: release NOT APPROVED, security 1 Low + 1 Info — both predate AQ-013/fuzz-gate/commit); (2) tag v1.0 only on approval; (3) commits resume after approval. P6-S05 still NOT MET (reviews, tag).

Repository Status: main at c17171d (ahead of origin by 2, unpushed); agent.md dirty (log only); no tag.

## 2026-10-09 — Patch batch committed (Orchestrator)

Session Summary: User chose "Restore skills/, then commit". Restored 40 `skills/*.md` from HEAD (`git restore skills/` — disk had been empty since ~Oct 4; verified 40/40 back, zero diff), then reviewed every hunk of the dirty tree before staging: Cargo.toml (quick-xml 0.38→0.41, calamine 0.31→0.36, `[patch.crates-io]`), both lockfiles, fuzz harness files + `scripts/fuzz_batch.sh`, docs (ci-setup batch-runner §5, pdfium trust warning + sizes), `parent_or_dot` bare-name fix + e2e pin, pdf_layout preflight-recovery + glyph dedupe/shadow-drop, AQ-013 tripwire test, agent.md/archive state. Removed 4 cargo-registry junk files (`.cargo-ok`, `.cargo_vcs_info.json`) from the vendored patch dirs before staging. Pre-commit `cargo build --locked --offline` clean. Committed as c17171d (no push — not requested). Left untouched: package.json/lock, sam/, roles/documenter.md, roles/readme.template.md, both audit reports (untracked review artifacts for a separate commit decision).

Files Created: none. Files Modified: all staged files above + project/agent.md (this log, uncommitted). Dependencies Added: none (quick-xml/calamine bumps were prior-session changes, committed here). Tests Executed: build clean pre-commit; suite 142/0/1 from earlier today stands (nothing compiled changed since).

Important Notes: P6-S05 still NOT MET (reviews, tag). agent.md now dirty with only this log entry + status lines. Next: re-reviews then tag per docs/ci-setup.md §6.

Repository Status: main ahead by 2 (dca3ebc + c17171d), unpushed; no tag.

## 2026-10-09 — Fuzz gate met: 3/3 targets ≥24 CPU-h, zero new findings (Orchestrator)

Session Summary: User reported all three fuzz targets completed. Verified from `fuzz/.batch-state/` + logs (no code changed): csv 86403s/12,195,781 runs/corpus 5506, rtf 86447s/34,393,191 runs/corpus 31353, pdf 86427s/792,655 runs/corpus 1028 — each ≥86400s, with `TARGET DONE` lines in the logs. Script semantics confirmed (`scripts/fuzz_batch.sh:74,87-90`): a run STOPs only on NEW crash/oom/timeout/leak artifacts per chunk (before/after count compare); the `findings=N` tally includes stale pre-existing files. pdf's `findings=16` are all stale pre-patch crashes (dated 2026-09-26–10-04, none newer) — zero new findings across the entire post-patch 86427s window (Oct 5–9 logs, all `chunk ok`). Patch wiring re-confirmed (`[patch.crates-io]` in `Cargo.toml`, lockfile references intact).

Files Created: none. Files Modified: project/agent.md (header, Overall Progress, §9 fuzz bullets, AQ-013, §13 latest run, this log). Dependencies Added: None. Tests Executed: `cargo build --locked --offline` clean (vendored `pdf-extract` lib emits unused-var warnings — cosmetic, patch code only); debug-binary decode check over all 16 retained `fuzz/artifacts/fuzz_pdf/crash-*` files: 16 graceful (typed error or success), 0 signal deaths; full `cargo test --locked --offline` 142 passed, 0 failed, 1 ignored (matches 2026-10-04 baseline).

Security Work: hostile-input containment re-proven on the full retained crash set (16/16, one more than AQ-013's 15-count — the extra file also predates the fresh run; no new crash class appeared in 792k post-patch runs).

Important Notes: §14 item 3 (fuzz) now MET with evidence. P6-S05 still NOT MET: commit-remainder (incl. AQ-013 patch batch + skills/ deletions still uncommitted), architecture/security reviews (release-audit NOT APPROVED verdict stands until re-audit), tag. No commit, no tag (not requested this turn). Recommendation: leave the 16 stale crash files in place (triaged evidence, seeded into corpus); do not delete.

Repository Status: main ahead by 1 + uncommitted patch batch (patches/, manifests, locks, tests, agent.md); no tag.

## 2026-10-04 — PDF fuzz fix: vendored patch, 15/15 clean (Orchestrator)

Session Summary: User said "fix the pdf 24h issue" (4th request) after approach ruling "Upstream fix/replace". Executed as user-authorized P4-S01 drift correction (extractors must not panic on malformed input), blueprint unchanged. Vendored `patches/pdf-extract` + `patches/adobe-cmap-parser` (same versions), hardened 11 panic sites to graceful degradation (maybe_deref dangling refs; CMap parse + caller + odd-length assert; ToUnicode shape/todo; Type0 empty-mapping fallback incl. get_byte_mapping caller; dead FontDescriptor validation removed; Tf hostile-font skip; Do hostile-XObject skip; fontless show_text span skip), each proven by a retained artifact. Fuzz corpus +15 seeds (649 files); pdf gate clock reset (pre-patch 3772s discarded — gate must run fresh). GATE MET 2026-10-09 (see log entry below).

Files Created: patches/pdf-extract/*, patches/adobe-cmap-parser/* (vendored 0.12.1/0.4.1 + hardening). Files Modified: Cargo.toml + fuzz/Cargo.toml ([patch.crates-io]), Cargo.lock + fuzz/Cargo.lock (resolved to patch paths; fuzz lock also bumped wasm-bindgen*-macro 0.2.128→0.2.129 from offline cache), tests/phase4_integration.rs (+`pdf_objects` helper + `dangling_tounicode_never_panics` tripwire), project/agent.md (AQ-013, §9, this log).

Dependencies Added: none new (same versions via path patch; colloquially a fork-of-two-functions held under patches/ with rationale comments).

Tests Executed: full `cargo test --locked --offline` 142 passed, 0 failed, 1 ignored (was 141/0/1; +1 tripwire; phase4 14/14); 15/15 retained artifacts decode panic-free; post-patch fuzz smoke 120s/1171 runs/0 crashes/0 new artifacts. Residual: unproven expects remain (content-stream operands, W-array bounds, MediaBox, Subtype dispatch) — future fuzz will prove or not; pdf 24h MET 2026-10-09 (86427s post-patch, zero new findings — see 2026-10-09 log; ~18 wall-hours figure was the pre-completion estimate).

Security Work: hostile-input abort surface reduced at the proven sites; containment (catch_unwind) retained as second layer; no new trust boundaries.

Important Notes: P6-S05 still NOT MET (fresh pdf 24h, commit, reviews, tag). No commit, no tag (not requested).

Repository Status: main ahead by 1 + uncommitted patch batch (patches/, manifests, locks, 1 test); no tag.

## 2026-10-04 — Fuzz triage: csv/rtf 24h clean, pdf blocked (Orchestrator)

Session Summary: User reported csv/rtf 24h done, pdf issue open. Verified from `fuzz/.batch-state/` + logs (no code changed): csv 86403s/12,195,781 runs/corpus 5506 findings 0; rtf 86447s/34,393,191 runs/corpus 31353 findings 0 (slow-units only, no crash/oom/timeout/leak); pdf 3772s/22,678 runs with 15 crash artifacts, batch STOP per `scripts/fuzz_batch.sh` crash-stop. Triaged all 15: 11x pdf-extract 0.12.1 `maybe_deref` expect("missing object reference") (lib.rs:177:52, ObjectNotFound), 3x adobe-cmap-parser 0.4.1 expect (lib.rs:169:31, CMap Mismatch), 1x pdf-extract expect("wrong type") (lib.rs:204:69). All inside `decode_whole` catch_unwind (`src/extract/pdf.rs:27`); cargo-fuzz `-Cpanic=abort` makes catch_unwind uncatchable under fuzz (upstream rust-fuzz behavior), so libFuzzer reports deadly-signal exit 77 while production (panic=unwind, `Cargo.toml:42`) returns `Corrupt`. Recorded as §9 blocker in Known Issues + §14 partial in Overall Progress/§9; P6-S05 still NOT MET; pdf 24h NOT claimed. Awaiting Architect ruling (pre-validate vs harness-filter vs accept-containment vs upstream fix).

Files Created: none. Files Modified: project/agent.md (header date, Overall Progress, §9 bullets, this log). Dependencies Added: None. Tests Executed: none (evidence read from batch-state + log TARGET DONE lines + artifact listing; no suite re-run).

Security Work: triage only; containment boundary (`decode_whole` catch_unwind) verified by code read + prior P4-S01 evidence, not re-proven by build this turn.

Important Notes: Do not resume pdf fuzz until Architect rules — re-running the same command reproduces the same STOP on the existing 15 artifacts. Do not mark §14 item 3 met (csv/rtf only = 2/3 targets).

Repository Status: unchanged (main ahead by 1, dirty tree incl. empty skills/, no tag).

## 2026-09-25 — Release-audit Blocking fix (Orchestrator)

Session Summary: User said "proceed with the fix" (release audit's bare-filename failure; LICENSE left for the user's license decision). Mapped all 7 `parent()` sites — 3 affected (md/html loaders, default output dir), 4 immune (constructed paths, exe path, dedup keys). Fixed + pinned + proven genuine. No architecture change.

Completed Steps: `package::parent_or_dot` helper (empty→".") → applied at mod.rs md/html loaders + convert.rs default-output dir → unit test (bare/subdir/absolute) → e2e `bare_filename_in_cwd_converts` (child-process CWD, no global dir change; md-with-output + md-default-output cases) → stash-proven genuine (FAILED pre-fix, ok post-fix, stash popped clean) → full suite 141/0/1 zero warnings.

Files Created: none. Files Modified: src/extract/package.rs (helper+test), src/extract/mod.rs (2 call sites), src/convert.rs (1 call site), tests/e2e.rs (+1 test), project/agent.md (this log).

Dependencies Added: None.

Tests Executed: full `cargo test --locked` 141 passed (139 + 2 new), 0 failed, 1 ignored, zero warnings, toolchain 1.98.0.

Security Work: none (path handling; containment guards unchanged and still green).

Important Notes: release-audit Blocking #2 resolved in code (re-audit still required for a verdict flip). LICENSE still needs the user's license text (cannot invent). P6-S05 still NOT MET (24h fuzz, push, reviews incl. re-audit, tag). No commit, no tag (not requested this turn).

Repository Status: main at dca3ebc (unpushed); bare-name fix uncommitted; no tag.

## 2026-09-25 — Release audit (Release Auditor)

Session Summary: User invoked `@roles/release-auditor.md`. Walked all 28 requirements against live evidence (no assumed passes). Found a genuine Blocking bug by hand: bare-filename md/html conversion fails. No fixes made (role forbids it). Verdict: NOT APPROVED.

Completed Steps: template + README + checklist + license/packaging reads → verified one README usage command end-to-end (tripped over the bug below; control case `./`-prefixed works, exit 0) → proven mechanism with a Rust probe (`parent()`→`Some("")`, `canonicalize("")`→NotFound) → full 28-item walk → report `project/release-audit-2026-09-25.md` + this log.

Files Created: project/release-audit-2026-09-25.md (new, uncommitted). Files Modified: project/agent.md (this log).

Blocking issues: (1) P6-S05 acceptance unmet (no tag/§14). (2) Bare-filename md/html fails — normalize empty parents + regression test (absolute paths unaffected; tests never caught it — always absolute Temp paths). (3) No shipped release. Advisory: no LICENSE; README staleness; stale Known-Issue bullets. Tallies: 20 pass / 3 blocking-fail / 1 advisory-fail / 4 N/A.

Dependencies Added: None. Tests Executed: none (audit-only; suite untouched 139/0/1).

Important Notes: P6-S05 still NOT MET. No commit, no tag. Also observed: Ebooks/ and sam/ evaporated between turns (env restore) — corpus evidence lives in agent.md only.

Repository Status: main at dca3ebc (unpushed); release report uncommitted; no tag.

## 2026-09-25 — Security findings fixed (Orchestrator)

Session Summary: User said "fix all" (security audit findings). Fixed the Low (doc warning), ran cargo audit (installed 0.22.2 after 2 timed-out compile windows), found + fixed 2 Highs, re-verified to zero. No architecture change.

Completed Steps: pdfium-setup.md trust warning → cargo-audit install → audit run (2 High quick-xml + 1 ttf-parser warning) → mirrored Highs to Known Issues immediately → quick-xml 0.38→0.41 → found second 0.38.4 copy via calamine 0.31 → calamine→0.36.1 (its quick-xml ^0.41 unifies; checked before bumping) → build clean, zero API breakage → suite 139/0/1 → re-audit 0 vulnerabilities → pdf-extract confirmed latest (ttf residual stands) → report statuses + capabilities table + Known Issues updated + this log.

Files Created: none. Files Modified: docs/pdfium-setup.md (+warning), Cargo.toml (2 reqs), Cargo.lock (unified), project/agent.md (mirror/resolve/log), project/security-audit-2026-09-25.md (statuses + Highs addendum).

Dependencies Added/Changed: quick-xml 0.38.4→0.41.0 (single copy now), calamine 0.31.0→0.36.1, atoi_simd 0.16.1→0.18.1; cruft removed (arbitrary, derive_arbitrary, zip 4.6.1). P2 API record: used surface (formula/hidden/number-format/date accessors) stable across the jump — no code changes needed.

Tests Executed: full `cargo test --locked` 139 passed, 0 failed, 1 ignored, zero warnings (before AND after the bumps — the bumps changed zero test outcomes); `cargo audit` re-run: 0 vulnerabilities, 1 allowed warning (ttf-parser, documented residual).

Security Work: this entire session. Auditor's Low → Resolved, Info → Resolved, 2 post-report Highs → Resolved with suite-as-regression.

Important Notes: P6-S05 Acceptance Criteria still NOT MET (24h fuzz, push, reviews, tag). No commit, no tag (not requested this turn).

Repository Status: main at dca3ebc (unpushed); findings-fix batch uncommitted; no tag.

## 2026-09-25 — Security audit (Security Auditor)

Session Summary: User invoked `@roles/security-auditor.md`. Ran adversarially per the role: threat model → ranked surface → traced data flow → live verification with temp tests (removed after) → report. No fixes made (role forbids it).

Completed Steps: surface enumeration (CLI/config/env/OCR/files ranked; docs first) → guard reads (ZIP/XML/RTF/PDF caps, error Display, walkdir, loaders, batch isolation, pdfium order) → 2 live verification tests (OCR symlink-swap refused + marker intact; HTML `../` loader escape contained) → secrets/shell/stdin sweeps (clean/none) → report `project/security-audit-2026-09-25.md` from template.

Files Created: project/security-audit-2026-09-25.md (new, uncommitted). Files Modified: project/agent.md (this log; Known Issues untouched — no Critical/High to mirror).

Findings: 0 Critical, 0 High, 0 Medium, 1 Low (pdfium adjacent-exe fallback load — local, documented order, needs install-dir trust; doc-note recommended via normal path), 1 Informational (cargo audit never run — no network here; pre-release gate). Full class checklist + both passing probe codes in the report.

Dependencies Added: None. Tests Executed: 2 temp verification tests, both green, then removed (code kept in report for the remediation record); suite untouched 139/0/1.

Important Notes: P6-S05 Acceptance Criteria still NOT MET (env-gated remainder + Low doc-note now queued). No commit, no tag (not requested this turn).

Repository Status: main at dca3ebc (unpushed); security report + fuzz_batch.sh + fuzz/.gitignore + ci-setup edits uncommitted; no tag.

## 2026-09-25 — Third audit-and-fix (Orchestrator as self-audit)

Session Summary: User invoked `@roles/audit-and-fix.md`. No architect.md/designer.md exist — equivalent-file rule applied (architecture/blueprint as decisions; Designer N/A headless CLI), same as the two prior audits. Phases 0–6 code unchanged since the 2026-09-24 audit except this session's AQ-011/012/link/ocr changes, which were read line-by-line (convert.rs helper+call site, pdf.rs loop+warning count, both renderer Link arms, ocr.rs allow); older phases covered by the prior code-level pass + this run's regression evidence (139/0/1, zero warnings). No src changes made by this audit except none — single fix is F1 below.

Completed Steps: role check → audit opened (Review History) → unwrap/allow/Command/fs sweeps (9 unwraps all test/infallible-position; 1 justified allow; argv-only Command; staged/guarded writes; no TODOs) → changed-file deep reads → leftover-diff review (pdf_layout shadow/preflight work: commented, prior-verified, stands for next layout CI re-verify) → guide var-name cross-check vs ci.yml (10/10 match) → F1 fix → close.

Files Created: none. Files Modified: fuzz/.gitignore (+logs, .batch-state), project/agent.md (open/close + this log).

Findings — FIXED (1): F1 cross-cutting run-hygiene: `fuzz/logs/` + `fuzz/.batch-state/` escaped gitignore (trial logs showed as untracked); added, status clean. No test re-run needed (non-code; suite 139/0/1 stands).
Findings — FLAGGED (no new ones): standing security-auditor.md referral (per role, adversarial pass out of scope here); env-gated remainder (24h fuzz, push, reviews, tag); pdf_layout next re-verify on layout CI run with native lib.
Compliance re-check: zero new drift/omissions/additions — blueprint-vs-tree known items unchanged (negative_suite keep-closed, xml.rs gap recorded, AQ-011/012 closed with tests). Quality re-check: new code follows house (minified) conventions; no rewrites proposed (no behavior-neutral gain available).

Dependencies Added: None. Tests Executed: none new (evidence: suite 139/0/1 zero warnings from prior turn; F1 touches no code).

Important Notes: P6-S05 Acceptance Criteria still NOT MET (env-gated remainder). No commit, no tag (not requested this turn).

Repository Status: main at dca3ebc (unpushed); audit fix (fuzz/.gitignore) + fuzz_batch.sh + ci-setup edits uncommitted; no tag.

## 2026-09-25 — Batch fuzz runner (Orchestrator)

Session Summary: User asked for the 24h fuzz prepared in batch with configurable time, resumable to completion. Built `scripts/fuzz_batch.sh` (budget/chunks/resume/crash-stop/evidence), trial-proven, docs pointed at it. No product behavior changed.

Completed Steps: harness check (nightly + cargo-fuzz 0.13.2; corpus/artifacts/state all gitignored) → script (flags `--targets/--hours/--chunk-min/--logs/--reset/--dry-run`; ≥30s minimums after catching that `-max_total_time=0` means unlimited — first trial hung and had to be killed) → guard tests (bad flag/tiny budget/bogus target fail fast) → trial `--targets rtf --hours 0.05 --chunk-min 1`: 3 chunks (60+60+48 remainder-shortened), 186s/270,761 runs/0 findings, corpus 4162→4797 → resume re-run immediate-DONE → reset verified → `docs/ci-setup.md` §5 rewritten around the script.

Files Created: scripts/fuzz_batch.sh (new, uncommitted). Files Modified: docs/ci-setup.md (§5), project/agent.md (this log).

Dependencies Added: None.

Tests Executed: script trials as above (trial state reset after; corpus additions remain on disk, gitignored).

Security Work: none; crash-stop path is code-reviewed but untested live (no crash induced) — stated, not claimed.

Process Note: hung-trial fallout (stray load ~6.9 decaying after kill; stray `ps|rg` also timed out once) — minimums now prevent recurrence; run one target at a time on loaded hosts.

Important Notes: §14 24h gate still open (trial ≈0.05 CPU-h). Run: `scripts/fuzz_batch.sh` (≈18 wall-hours on 4 cores for all three). P6-S05 Acceptance Criteria still NOT MET. No commit, no tag (not requested this turn).

Repository Status: main at dca3ebc (unpushed); scripts/fuzz_batch.sh + docs/ci-setup.md uncommitted; no tag.

## 2026-09-25 — AQ-012 fixed (Orchestrator)

Session Summary: User said "fix it". Root-caused with a temporary instrumented test (since removed): the 478pp book decoded 808,965 real chars ("FATIMA BALA / Broken" novel) with zero formfeeds → all text attributed to operator-absent page 1 → old scanned-branch discarded the blob. Fixed text-wins + zero-yield-placeholder layers (see AQ-012). Full suite green, release rebuilt, Database proof: 9KB/0-text → 3.47MB text.

Completed Steps: diag probe (with_ops=477/478, decoded 808KB/0-FF) → loop rewrite + flagged-set warning count → 2 new tests → temp-test removal → full suite 139/0/1 zero warnings → release 4,310,304 B → Database proof (1376 bounds, 0 scans, 0 FFFD, 1 warning).

Files Created: none (zz_diag.rs created + removed). Files Modified: src/extract/pdf.rs (loop + warning count), tests/phase4_integration.rs (+2 tests), project/agent.md (AQ-012 closed, this log).

Dependencies Added: None.

Tests Executed: full `cargo test --locked` 139 passed, 0 failed, 1 ignored, zero warnings, toolchain 1.98.0.

Security Work: none (extraction logic only; no trust-boundary change).

Important Notes: AQ-012 CLOSED. The 11 sam/ scanned books mostly contain REAL text PDFs misread earlier — several "0-text" outputs will now convert (re-run recommended for corpus numbers). Remaining: 24h fuzz, commit-remainder, reviews, tag. P6-S05 Acceptance Criteria still NOT MET. No commit, no tag (not requested this turn).

Repository Status: main tracks origin/main at 9cc9be1 (+ uncommitted fixes); no tag.

## 2026-09-25 — sam/ PDF run (Orchestrator)

Session Summary: User asked to test the 21 selected PDFs in `sam/`. Converted all with current binaries (debug first: 21.7MB file hit 280s timeout and 2.3MB took 105s — debug-build pdf-extract cost, known; rebuilt release 4,309,504 B with all fixes and reran: 10–20x faster). Outputs to /tmp, then packaged `sam-eval/` (6.2MB + sam.tsv) for user evaluation. No source changes.

Completed Steps: 21/21 attempted → 20 ok, 1 honest Encrypted fail (Computer Vision, password-protected). Textbooks that are real text PDFs superb (CS:APP 1105pp→2.49MB, C++ Wiki 684pp→1.19MB, deeplearningbook 800pp→1.87MB, all 0 FFFD); timetable clean with tabular alignment; only 2 FFFD in the whole run (JavaAndroidStudio). Read 4 outputs directly + placeholder/text-line audit across all 20.

Files Created: sam-eval/ (untracked, user evaluation). Files Modified: project/agent.md (AQ-012, §13, this log). Dependencies Added: None. Tests Executed: none (137/0/1 stands).

Security Work: encrypted file → clean Encrypted error, no partial output.

Findings: GENUINE GAP → AQ-012 (open): 11 scanned books (up to 1376pp, 0 text lines each) yield ~1 NeedsOcr flag each; the rest emit bare page boundaries with no placeholder and the report understates ("1 page(s) need OCR") — silent loss against §0.1; needs Architect signal/counting ruling, code untouched.

Important Notes: P6-S05 Acceptance Criteria still NOT MET (AQ-012 ruling, 24h fuzz, commit-remainder, reviews, tag). No commit, no tag.

Repository Status: main tracks origin/main at 9cc9be1 (+ uncommitted fixes); no tag.

## 2026-09-25 — Quality issues fixed + PDF test (Orchestrator)

Session Summary: User asked to fix quality-eval findings, explain the silent EPUB, and add a PDF test. Three items, one pass. No architecture change.

Completed Steps: (1) SILENT EPUB VERDICT — the `.epub.pdf` file is 211,536 bytes of pure NUL (verified every byte); rerun exits 1 with typed Unsupported + mismatch note and writes zero files. Correct behavior — a zeroed download has nothing to convert; no product change. (2) `[]()` FIX — empty-target `Inline::Link` now renders bare children in both renderers (`markdown.rs`, `txt.rs` "text ()" variant fixed too); pinned by `empty_link_target_renders_bare_text` (phase0). (3) PDF TEST — `zeroed_bytes_rejected_honestly` (phase4): zeroed `.pdf` → Unknown + mismatch note → Unsupported "no recognizable signature", no partial outputs.

Files Created: none. Files Modified: src/render/markdown.rs, src/render/txt.rs (1 line each), tests/phase0.rs, tests/phase4_integration.rs (+1 test each), project/agent.md (this log).

Dependencies Added: None.

Tests Executed: full `cargo test --locked` 137 passed (135 + 2 new), 0 failed, 1 ignored, zero warnings, toolchain 1.98.0.

Security Work: none (renderer fallback + tests only).

Important Notes: P6-S05 Acceptance Criteria still NOT MET (24h fuzz, commit-remainder, reviews, tag). No commit, no tag (not requested this turn).

Repository Status: main tracks origin/main at 9cc9be1 (+ uncommitted fixes); no tag.

## 2026-09-25 — Random quality sample (Orchestrator)

Session Summary: User asked for 5 seeded-random files per type (PDF excluded) with output-quality judgment. Seed 20260925, post-fix debug binary, outputs to /tmp. 23/35 converted; all 12 failures in already-root-caused honest classes (zeroed stubs, legacy .ppt OLE2, IRM stub, generic XML). No source changes.

Completed Steps: sampled 35 (lists in /tmp/quality-sample.json) → converted → signal pass (sizes/headings/tables/images/FFFD/warnings) → read 8 outputs directly + source-checked 2 suspicions.

Files Created/Modified: project/agent.md (this log). Dependencies Added: None. Tests Executed: none (135/0/1 stands).

Quality verdict (convertibles): docx DESIGN rich doc excellent (104 headings, tables, 8 images); Full Stack EPUB superb (1.47MB md, ~3k headings, 671 images, cover alt-text); flat-source EPUB faithfully flat (source has ~1 h-tag per 8 chapters — verified in the ZIP, not dropped); PPTX deck excellent (slide headings, 18 positioned images, bullets); xlsx tables byte-sound incl. formulas path; html/odt clean with auto-selection working both directions (novels→txt, rich→md). Zero U+FFFD in every output. Misnamed scanned-PDFs correctly surfaced as ScannedPage placeholders. Cosmetic wart noted (not fixed): empty-target link renders as `[]()` (How-to-Become head) — Architect may rule bare-text fallback.

Important Notes: P6-S05 Acceptance Criteria still NOT MET (24h fuzz, commit-remainder, reviews, tag). No commit, no tag.

Repository Status: main tracks origin/main at 9cc9be1 (+ uncommitted AQ-011 fix); no tag.

## 2026-09-25 — AQ-011 fixed (Orchestrator)

Session Summary: User said "fix them". The only genuine code defect in the open set was AQ-011 (everything else: honest behaviors, env-gated evidence, or already closed). Fixed + pinned + proven. One process slip caught and repaired mid-step (helper insert briefly displaced the `convert_file` signature line; restored immediately, verified by read + green build).

Completed Steps: extracted `asset_namespace()` helper with 200-byte cap + `long-<prefix>-<hash>` fallback + pre-report warning; 4 lib unit tests; e2e `long_stem_shortens_asset_namespace`; full suite green; corpus-repro proof (exit 0, asset under shortened namespace).

Files Created: none. Files Modified: src/convert.rs (helper + call-site + tests), tests/e2e.rs (+1 test), project/agent.md (AQ-011 closed, this log).

Dependencies Added: None.

Tests Executed: full `cargo test --locked` 135 passed (130 + 4 unit + 1 e2e), 0 failed, 1 ignored, zero warnings, toolchain 1.98.0. (Mid-step: 1 self-authored unit test initially asserted shortening for a 66-char stem that correctly does not trigger — test string fixed, code was right.)

Security Work: none new (naming bound only; no trust-boundary change).

Important Notes: AQ-011 CLOSED. Remaining: 24h fuzz, full PDF sweep (optional), commit-remainder, reviews, tag. P6-S05 Acceptance Criteria still NOT MET. No commit, no tag (not requested this turn).

Repository Status: main tracks origin/main at 9cc9be1 (+ uncommitted fix); no tag.

## 2026-09-25 — Ebooks corpus run (Orchestrator)

Session Summary: User pointed at `./Ebooks/*` (8 format dirs, 1771 files / 5.6G). Rebuilt release binary (ocr.rs attribute only — same 4,307,952 B), converted all 359 non-PDF files + stratified PDF subset (25 smallest + 10 largest of 1412), outputs to /tmp (repo clean). No source behavior changed except 5 fuzz-seed copies.

Completed Steps: release rebuild; non-PDF batch (225/359 ok); PDF subset (29/35 ok, 3.4MB text, 0 U+FFFD); per-class failure root-causing (magic-byte + message + strace probes); 5 tiny real PDFs seeded to `fuzz/corpus/fuzz_pdf/` (18→23); agent.md evidence (§13), new AQ-011, this log.

Files Created: none in repo (fuzz seeds on disk; /tmp outputs). Files Modified: project/agent.md only. Dependencies Added: None. Tests Executed: none (130/0/1 stands).

Security Work: hostile/garbage inputs produced typed errors only — no crash, no partial output; real dependency panic contained (see below).

Findings: (1) All non-timeout failures are honest: AD-5 legacy stubs, zeroed/garbage/misplaced inputs with mismatch notes, IRM stubs not misdetected, generic-XML Unknown (no xml.rs step exists — recorded gap), misplaced ZIP noted. (2) 3 perf timeouts (>90s: 2×10MB font-dense PDFs, 1×21MB EPUB) — §14-batch watch items. (3) P4-S01 containment PROVEN on real input: pdf-extract panicked (`missing unicode map and encoding`, Handbook PDF) → hook printed, unwind caught, typed Corrupt, exit 1. (4) GENUINE BUG → AQ-011: 162-char stem → 329-byte hex namespace → ENAMETOOLONG → `I/O error: invalid filename`; needs Architect naming-policy ruling, code untouched.

Important Notes: §14 corpus bar (≥20/format) now has: docx 134, epub 3, html 7, odt 10, pdf 29+11 prior, pptx 50, xlsx 21, xml 0-convertible — epub/html/odt/xml short or structurally unconvertible; full 1412-PDF sweep + 24h fuzz + commit-remainder + reviews + tag still pending. P6-S05 Acceptance Criteria still NOT MET. No commit, no tag.

Repository Status: main tracks origin/main at 9cc9be1; no tag.

## 2026-09-25 — CI green report + fuzz/corpus plan (Orchestrator)

Session Summary: User reports remote CI all green (Windows fix confirmed by re-run; run log not inspected here — recorded as user-reported). Asked whether GitHub can do the 24h fuzz + pointed at corpus `ebooks/*` (not present in this workspace — environment restores non-repo files between turns; `pdf/` from the prior corpus run is also gone).

Completed Steps: verified GitHub job limits against live docs (hosted 6h/job, self-hosted 5 days/job); designed two fuzz-on-GitHub options (see log); no code changed.

Files Created/Modified: project/agent.md (this log). Dependencies Added: None. Tests Executed: none.

Important Notes: (a) Single 24h fuzz job is IMPOSSIBLE on GitHub-hosted runners (6h hard cap) — needs 4×6h chained runs/target with corpus artifacts, or one self-hosted-runner job (5-day cap). (b) Fuzz minutes cost: 3 targets × 24h = 72h wall ≈ 4320+ billed minutes — exceeds the 2000 free-minutes tier; needs paid minutes or self-hosted. (c) Corpus: awaiting user placing `ebooks/` at a workspace path — then run per-file conversions + seed fuzz corpus in-session. P6-S05 Acceptance Criteria still NOT MET (fuzz hours, commit-remainder, reviews, tag). No commit, no tag.

Repository Status: main tracks origin/main at 9cc9be1; no tag.

## 2026-09-25 — Fix committed + pushed (Orchestrator)

Session Summary: User said "git add commit and push on the fix". Committed scoped to the fix only (`.gitattributes` + `src/ocr.rs`, 7 insertions/1 deletion — staged diff reviewed before commit) and pushed. Pre-existing user commits found on arrival (branch is `main`, not `master`; first commit + actions-SHA fix already in). No code changed in this step.

Completed Steps: `git add` (2 paths) → staged-diff review → `git commit` (`9cc9be1`) → `git push -u origin main` (`85e4f22..9cc9be1`, tracking set).

Files Created/Modified: none beyond the commit itself. Dependencies Added: None. Tests Executed: none (130/0/1 stands).

Important Notes: Push went to `main` (initial `master` push failed — branch is `main`; corrected, no force). Next: re-run Windows CI job on a fresh checkout; then fuzz/commit-remainder/reviews/tag per ci-setup guide. P6-S05 Acceptance Criteria still NOT MET. No tag.

Repository Status: main tracks origin/main at 9cc9be1; no tag.

## 2026-09-25 — Windows CI failures root-caused + fixed (Orchestrator)

Session Summary: User reported remote CI: all OS green except Windows (2 golden failures + 1 warning; log pasted). Two findings, both fixed in one pass. No architecture change.

Completed Steps: (1) GOLDENS — `markdown_golden`/`txt_golden` compare renderer output byte-exact against `include_str!("golden/basic.md|txt")`. Windows checkout converts LF→CRLF, so the file side carried `\r\n` while the renderer correctly emits `\n` (P0-S07). Reproduced byte-identically locally by CRLF-converting the goldens (same left/right, same 2 tests), then restored (SHA-verified identical). Fix: new `.gitattributes` pinning `eol=lf` for `tests/golden/*` (+ `*.rs`); `git check-attr` confirms. Other `include_str!` users are immune (`prose_pdf_decodes` trims, `cli_docs` uses `.lines()`). Fix is checkout-time: needs a fresh Windows CI run to confirm (no Windows host here). (2) WARNING — `src/ocr.rs:10` `let mut b`: `mut` only used inside `#[cfg(unix)]`, so Windows lints `unused_mut` (project standard: zero warnings). Fix: `#[allow(unused_mut)]` on that statement — behavior-neutral on all platforms.

Files Created: .gitattributes (new, uncommitted).

Files Modified: src/ocr.rs (1 attribute), tests/golden/* (temporarily CRLF for repro, restored byte-identical — SHAs `444117fb…`, `192eda43…` match), project/agent.md (this log).

Dependencies Added: None.

Tests Executed: repro run (2 failed as on Windows, byte-identical); after fix `cargo build --locked` zero warnings + full `cargo test --locked` 130 passed, 0 failed, 1 ignored, toolchain 1.98.0.

Security Work: none (test-line-endings + lint allow; no behavior change).

Important Notes: Windows CI must be re-run (fresh checkout picks up .gitattributes). P6-S05 Acceptance Criteria still NOT MET (remote CI confirmation, 24h fuzz, commit, reviews, tag). No commit, no tag.

Repository Status: master, no commits yet; no tag.

## 2026-09-25 — CI-ready + user guide (Orchestrator)

Session Summary: User said "proceed with CI and guide for user if user action is required". Did everything executable locally; wrote `docs/ci-setup.md` (explicitly requested) for the rest. No source, test, or config behavior changed.

Completed Steps: (1) Reviewed `scripts/fetch_pdfium.py` against `skills/ci.md`/`packaging.md` — conformant as-is (https-only URL, 64-hex SHA required, 250MB fetch cap, SHA verified pre-use, exactly-one bounded library member for zip+tar, fixed output name so no zip-slip, fails closed). No hardening change needed. (2) Confirmed `docs/pdfium-setup.md` already documents CI Variables per platform + the one verified Linux-x64 instance (no invented provenance). (3) Created `docs/ci-setup.md`: local pre-flight record, remote+push steps, Variables table, runner check, result-reading rules, full 24h fuzz commands, reviews-then-tag order.

Files Created: docs/ci-setup.md (new, uncommitted).

Files Modified: project/agent.md (this log).

Dependencies Added: None.

Tests Executed: none (no code changed; 130/0/1 + fuzz smoke stand).

Security Work: none new; fetch-script review recorded above; security-auditor.md referral still open.

Important Notes: Remote CI execution, full fuzz, commit/push, reviews, tag all still require user action per the new guide. P6-S05 Acceptance Criteria still NOT MET. No commit, no tag.

Repository Status: master, no commits yet; docs/ci-setup.md untracked; no tag.

## 2026-09-25 — CI/minor/tag triage (Orchestrator)

Session Summary: User said "proceed with all of CI and minor and the tag". Investigated actionability before touching anything. No source, test, doc, or config files changed — agent.md only.

Completed Steps: (1) CI feasibility probe: no git remote configured, no `gh` CLI, no RUST_TOOLCHAIN/PDFIUM_* vars, no push authorization — remote 10-combo execution is environment-blocked, recorded below, not attempted. (2) MINOR closed: user chose keep-current-files for the `negative_suite.rs` naming drift (mapping in §13 stands as the record, blueprint unchanged). (3) TAG declined with reasons (see Important Notes) — tagging now would violate Completion Rules + packaging skill.

Files Created/Modified: project/agent.md only. Dependencies Added: None. Tests Executed: none (130/0/1 + fuzz smoke stand).

Security Work: none; security-auditor.md referral still open.

Problems Encountered: remote CI + tag blocked (environment/evidence gates, not code findings).

Important Notes: Tag REFUSED — §14 gates unmet: remote CI never executed, fuzz at 90s/target (not 24h), no commit, architecture/security reviews open. To unblock, in order: (a) `git remote add origin <url>` + auth + set RUST_TOOLCHAIN/PDFIUM_URL_*/PDFIUM_SHA256_* vars from reviewed provenance; (b) say "commit" (explicit) then push to trigger the matrix; (c) full 24h fuzz; (d) run release-auditor + security-auditor roles; (e) tag only when agent.md §14 is fully evidenced. P6-S05 Acceptance Criteria still NOT MET. No commit, no tag.

Repository Status: master, no commits yet; no tag.

## 2026-09-25 — (h), CI matrix, minor items (Orchestrator)

Session Summary: User said "proceed with h, CI matrix, minor open items". Worked all three single-step, in order. No source behavior changed — agent.md state + verification runs only.

Completed Steps: (1) MINOR — 130-vs-131 delta resolved (`-- --list` = 131 total: 130 runnable + 1 ignored; "131 passed" was shorthand). (2) MINOR — §9-row-to-test mapping recorded in §13 (P6-S04 DoD item; `tests/negative_suite.rs` filename drift flagged, behavior covered by regressions.rs + phase6). (3) (h) DONE by user-authorized documented decision, blueprint unchanged: `samples()` = one generatable sample per blueprint-referenced input type (25/25 detecting, pinned by `generated_samples_have_recognized_signatures`); per-scenario fixtures are generated at test time by `tests/common/` builders (zip/png/docx/epub/odt/pptx/xlsx/ods/xls/pdf_streams/zip_patch/fake_ocr) + inline code; audit outcome — zero binary fixtures committed (`tests/fixtures/fake-ocr/main.rs` is source compiled at test time; `tests/golden/` is text), so every phase's tests are reproducible from source per P6-S05 instruction 7. `write_samples` (ignored) materializes `samples/generated` on user demand. (4) CI MATRIX partial: workflow YAML parses (5 platforms × 2 profiles = 10 combos, `contents: read` least-privilege); `cargo check --locked --offline --features pdf-layout` green (dev); layout `-- --list` = 144 tests (131 + 13 gated). Remote execution on runners + layout test execution (native lib absent) + full 24h fuzz still pending.

Files Created: None (release-artifacts/ from prior step left on disk, uncommitted).

Files Modified: project/agent.md (this log, §8 Next Step, §13 layout note + §9 mapping, 130/131 resolution).

Dependencies Added: None.

Tests Executed: no suite re-run (130/0/1 stands from earlier today); `-- --list` counts only (default 131, layout 144); fuzz smoke stands (3×90s zero crashes).

Security Work: none new; security-auditor.md referral still open.

Problems Encountered: remote CI + full fuzz + reviews/commit/tag remain environment/authority-gated, not code findings.

Important Notes: P6-S05 Acceptance Criteria still NOT MET (needs remote CI green, 24h fuzz, commit, reviews, tag). No commit (not explicitly requested), no tag.

Repository Status: master, no commits yet; no tag.

## 2026-09-25 — CI-equivalent + fuzz smoke (Orchestrator)

Session Summary: User said "proceed with CI execution, 24 fuzz". Full GitHub matrix (10 combos) cannot run locally — no remote runners/vars; full 24 CPU-hours/target (≈18 wall-hours on 4 cores) cannot complete in-session. Executed the honest subset: local CI-equivalent for linux-x86_64 default + 90s ASan smoke per fuzz target. No source, test, doc, or config files changed — agent.md + release-artifacts/ outputs only.

Completed Steps: `cargo tree --locked` (0 pdfium); `cargo build --locked --release` (4,307,952 B); `package_binary.py default` (archive `7ba7d9f7…`); `cargo +nightly fuzz run fuzz_{rtf,pdf,csv} -- -max_total_time=90` (156,466 / 476,309 / 29,073 runs, zero crashes, no crash/oom/timeout artifacts).

Files Created: release-artifacts/docconvert-linux-x86_64-default.{tar.gz,json} (local only, not committed).

Files Modified: project/agent.md (this log + §13 evidence).

Dependencies Added: None.

Tests Executed: CI-equivalent checks above + prior 130/0/1 suite (reused, not re-run); fuzz smoke 3×90s as above.

Security Work: none new; fuzz smoke is regression signal only, not §14 evidence; security-auditor.md referral still open.

Problems Encountered: remote CI matrix + pdf-layout re-run + full 24h fuzz all still pending (environment/RFC limits, not code findings).

Important Notes: Full-fuzz commands for user/CI (from fuzz/: `cargo +nightly fuzz run fuzz_rtf -- -max_total_time=86400`, same for fuzz_pdf/fuzz_csv — one target at a time; each ≈24 wall-hours single-worker, or parallelize across cores/hosts and sum CPU-hours from `stat::` lines; stop on first crash artifact and file it before continuing). Fuzz seeds: 5 tiny real PDFs from Ebooks/ added to `fuzz/corpus/fuzz_pdf/` (18→23; rtf 2292, csv 587 from prior runs).

Corpus run 2026-09-25 (`Ebooks/`, 1771 files / 5.6G: docx 190, epub 6, html 7, odt 10, pdf 1412, pptx 101, xlsx 35, xml 10; release binary rebuilt same-day 4,307,952 B; outputs to /tmp, repo clean): converted all non-PDF (359 files: 225 ok) + stratified PDF subset (25 smallest + 10 largest: 29/35 ok, 3.4MB text, **0 U+FFFD**). Zero process crashes, zero partial outputs. Failure classes (each root-caused, none a silent loss): legacy OLE2 DOC/PPT → AD-5 detect-and-report; zero-filled / garbage / misplaced-extension inputs → honest Unknown + mismatch notes; IRM stubs (MsoIrmProtector) → Unknown, not misdetected as OLE2/BIFF; generic XML → Unknown (xml.rs never built — arch-table Extended item with no blueprint step, recorded gap); misplaced ZIP → "without recognizable document structure"; 3 TIMEOUTs (>90s: 2×10MB font-dense PDFs, 1×21MB EPUB — perf outliers, §14-batch watch item). Real-dependency panic contained: `pdf-extract 0.12.1` panicked (`missing unicode map and encoding`) on `Electrical Engineering Handbook.pdf` — panic-hook line on stderr, unwind caught per P4-S01, typed `Corrupt` error, exit 1, no partial output. Genuine bug found: AQ-011 (long-stem asset-namespace exceeds NAME_MAX — see Architect Questions).

Repository Status: master, no commits yet; release-artifacts/ + fuzz corpus updates on disk, uncommitted; no tag.

## 2026-09-25 — P6-S05 verification re-run (Orchestrator)

Session Summary: User said "proceed". (h) samples_gen enumeration remains Architect-blocked, so no code handoff. Ran the single executable verification step instead: default-profile build + full test suite, then updated state. No source, test, doc, or config files changed — agent.md only.

Completed Steps: `cargo build --locked` (clean, zero warnings); `cargo test --locked` (130 passed, 0 failed, 1 ignored); agent.md state update (§§header/Current Status/8/13 + this log).

Files Created: None.

Files Modified: project/agent.md (state only).

Dependencies Added: None.

Tests Executed: full default suite 130/0/1, toolchain 1.98.0, zero warnings. Per-binary: lib 11, main 1, cli_docs 1, detect 9, e2e 10, model 5, phase0 14, phase1 12, phase2 11, phase3 4, phase4 10, phase5 4, phase6 9, regressions 28, samples_gen 1 (+1 ignored write test), doc-tests 0. Note: one fewer passed than the 131 claimed 2026-09-24; no `tests/negative_suite.rs` binary exists (blueprint P6-S04 names it; adversarial coverage currently lives in regressions.rs + phase6) — recorded, not resolved here.

Security Work: none new; security-auditor.md referral still open.

Problems Encountered: 130 vs prior 131 delta (under investigation); (h) still blocked on Architect fixture list; §14 evidence (corpus, 24h fuzz, CI execution, commit/tag) still pending.

Important Notes: P6-S05 Acceptance Criteria still NOT MET. No commit (not explicitly requested). No tag.

Repository Status: master, no commits yet, working tree matches prior staged state plus this agent.md edit; no tag.

## 2026-09-24 — Layout quality fixes on corpus (Orchestrator)

Session Summary: User asked why Automate layout output was poor ("AAUUTTOOMMAATTEE"). Root-caused with a throwaway pdfium probe (since removed): InDesign double-draws display type — title shadow at systematic offset (dx=1.96, dy=2.00 @40pt), subtitles at (0.74, 0.52) @12pt. Fixed in two layers: `dedupe_glyphs` tolerance 0.5→1.0pt (legit advances run 3pt+; verified safe) + `drop_shadows` size bar 18→10pt with multi-bin loop (first version applied only the top bin, so title shadows masked subtitle shadows; decoy bins like dotted leaders are skipped via the >=3-distinct-chars guard, pinned by a hijack unit test). Verified: Automate cover now reads AUTOMATE / LEARN PYTHON. / GET STUFF DONE. / THE BORING STUFF; leftover fragments ("AA", "SU WITH PYTHON") confirmed genuine cover ornament via poppler (same scattered words). 575/588 code-block warnings are legitimate (programming book). Suites: default 131/1, layout 144/1, zero warnings. Release re-measured: layout 4,188,216 B sha `248688cf…` (repackaged); default 4,307,952 B unchanged. Corpus now 11/11 convertible (8 default + 3 layout).

Completed Steps: glyph probe → dedupe 1.0 → shadow multi-bin + 10pt bar → hijack/two-layer unit tests → shadow e2e test → release rebuilds → cover verified → both suites green.

Files Modified: src/extract/pdf_layout.rs, tests/phase4_integration.rs (+`shadow_double_draw_renders_once`), project/agent.md, project/agent-archive.md, docs/pdfium-setup.md (sizes).

Dependencies Added: None.

Tests Executed: default 131/1; feature 144/1 (chromium/8066 lib).

Security Work: none new; security-auditor.md referral still open.

Repository Status: Fully staged but UNCOMMITTED per user decision ("No commit"); no tag.

## 2026-09-24 — Real-PDF corpus run (Orchestrator)

Session Summary: Ran the user-supplied `pdf/` corpus (11 real PDFs, 1.1–17.8MB: textbooks, manual, paper chapter) through the default release binary, one file per convert call, `timeout 280` each, outputs to /tmp (repo left clean). Result: **8/11 converted** (all auto-selected `.txt`, 11.7MB total text, **zero U+FFFD** in every output; spot-checks show clean headings, page boundaries, Unicode). **3 honest typed failures, zero crashes, zero partial outputs:** Active Calculus + app-comb-2017 → `Corrupt("PDF text could not be decoded")` (pdf-extract defeat on odd/compressed fonts; containment path working as designed); Automate (17.8MB, 510pp) → `Corrupt("PDF structure could not be loaded")` (lopdf xref limit, 0.5s). Layout spot-check (chromium/8066 lib, before binary revert): 19-page academic chapter in 5.4s → 4 tables + 12 dimensioned images + unruled-table warning. Observations: (a) earlier Trigonometry slowness was DEBUG-build pdf-extract cost — release converts it in 10.8s; layout preview on it exceeded 5min (font-dense doc; layout-perf caveat recorded). (b) Wall ≈ 2× user CPU on every file (fsync-heavy environment, not product-bound). (c) Environment restores files between turns (repo-root .so and stray pdf/ outputs seen earlier were gone; layout release binary reverted to default) — measurements below were live at capture time. Corpus verdict: honest-success/honest-failure on real docs; scanned/odd-font PDFs remain layout+OCR territory (needs tesseract, absent here).

Completed Steps: corpus run (8 ok + 3 typed-error), layout chapter spot-check, quality spot-checks.

Files Modified: project/agent.md, project/agent-archive.md (outputs in /tmp only).

Dependencies Added: None.

Tests Executed: None new (evidence run, not a code change); suites still default 130/1, layout 140/1.

Security Work: hostile/complex PDFs produced typed errors only — no crash, no partial write; security-auditor.md referral still open.

Repository Status: Fully staged but UNCOMMITTED per user decision ("No commit"); no tag.

## 2026-09-24 — Remaining-issues batch (Orchestrator)

Session Summary: Worked the remaining-issues list top to bottom. Network was available, which unblocked the feature build. (1) Hygiene: `libpdfium.so` added to `.gitignore` (repo-root copy proven incompatible — binds fail; md5 d0cc88ec). (2) `cargo fetch` pulled missing crates (itertools/chrono/console_error_panic_hook for pdfium-render); worktree Cargo.lock confirmed current (lopdf 0.45 + 0.42-via-pdf-extract, pdf-extract 0.12.1, calamine 0.31, pdfium-render 0.9.4). (3) Feature check caught 3 syntax errors in the A13 list-nesting edit (fixed; lesson: minified one-liners resist brace-counting — rewrote multi-line). (4) REAL BUG FOUND + FIXED: pdfium-render's global OnceCell makes only the first per-process bind succeed — every layout PDF after the first in one process (batch/CLI/tests) failed with "library not found". Fix: process-global shared `Pdfium` (`shared_pdfium`: per-call explicit-path validation, first-wins source check, lazy init without poisoning) + A12 password/security→Encrypted mapping. Verified: full feature suite 140/1 with chromium/8066 lib (incl. native OCR e2e); order-dependent failures reproduced pre-fix, gone post-fix. (5) Samples enumerated to 25/25 detecting (all formats + scanned/encrypted/pdf + ODP + 7 image headers + xhtml/tsv/md). (6) Release: default 4,307,648 B / layout 4,147,384 B + SHAs in docs/pdfium-setup.md (with pinned URL/SHA/license evidence; same-target rebuild warning noted). (7) Perf: 100 mixed-file batch exit 0, 18.3s wall. (8) CI actions pinned to SHAs; YAML validated (10 combos). (9) cargo-fuzz harness (rtf/pdf/csv targets + seeds) + 90s ASan smoke runs, zero crashes; 24h/target still open. Still open: real-doc corpus (needs user docs), full fuzz hours, CI execution, commit/tag, security-auditor referral.

Completed Steps: hygiene, feature build, shared-Pdfium fix, A12 mapping (+`layout_encrypted_reports_encrypted`), samples 25/25, release sizes, perf evidence, CI pins, fuzz harness + smoke runs.

Files Modified: .gitignore, .github/workflows/ci.yml, src/extract/pdf_layout.rs, tests/phase4_integration.rs, tests/samples_gen.rs, docs/pdfium-setup.md, fuzz/* (new), project/agent.md, project/agent-archive.md.

Dependencies Added: none to the product (fuzz/ workspace: libfuzzer-sys 0.4 + own Cargo.lock).

Tests Executed: default 130/1; feature 140/1 (PDFIUM_DYNAMIC_LIB_PATH=chromium/8066); fuzz smoke 3×90s zero crashes.

Security Work: no new trust boundaries; stale incompatible .so flagged (not deleted — user's file); security-auditor.md referral still open.

Repository Status: Fully staged but UNCOMMITTED per user decision ("No commit"); no tag. libpdfium.so + package.json + pdf/ left untracked.

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


---

### Compacted Review History rows (moved 2026-10-09; most-recent row stays in `agent.md`)

| Reviewer | Blueprint Version | Major Findings | Required Fixes | Status |
| --- | --- | --- | --- | --- |
| Orchestrator | v2.0 (merged) | Dependency version mismatches prevented build | lopdf 0.36→0.45, calamine 0.28→0.31 | **Closed** |
| Orchestrator | v2.0 (merged) | All 42 blueprint steps verified | None | **Closed** |
| self-audit | v2.0 (merged) | Phase-by-Phase + Full-Project + code-level audit: 8 findings, 4 fixed (git, status, security checklist, .gitignore), 0 code bugs, 3 design-level flags | Git init, status correction, security checklist, doc verification | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-24 audit-and-fix session: phase-by-phase + full-project code-level re-audit incl. unvalidated P4-S01 pdf-extract change | Fixed C1 (P4-S02 attribution guard), C2 (P5-S03 OCR warning gate); P4-S01 validated 96→101 passed; remainder flagged as queued single steps (see log) | **Closed** |
| self-audit | v2.0 (merged) | Scope: Phase-by-Phase + Full-Project (code-level, all 42 steps + cross-cutting). Architect/designer role files absent — architecture.md/blueprint.md used as Architect decisions per equivalent-file rule; Designer N/A (headless CLI, arch §1) | Fixed 15 clear findings (C1–C15, see Session Log); 22 items flagged for Architect (A1–A22, see report in log); security items referred to security-auditor.md | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-25 audit-and-fix session: third full pass; focus on post-2026-09-24 changes (AQ-011/012, link fallback, ocr allow, CI/fuzz/batch-test scripts, docs) + regression sweep | Fixed F1 (fuzz run-output gitignore); no new compliance/quality findings; standing referrals unchanged | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-25 audit-and-fix session: fourth full pass; focus on post-third-audit changes (dep bumps quick-xml/calamine, bare-name fix, security/release reports) + regression sweep | Superseded by fifth pass (Closed) + security/release re-audits 2026-10-09; no separate fourth-pass findings outstanding | **Closed** |

