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
