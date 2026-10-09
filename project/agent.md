# Project Agent

> **Project:** docconvert
> **Version:** 1.0
> **Maintained By:** Orchestrator & Implementer
> **Created From:** `project/agent.template.md`
> **Status:** In Progress — P6-S05 close-out: fuzz §14 item met 2026-10-09 (3/3 targets ≥24 CPU-h, zero new findings); commit-remainder, reviews, tag pending
> **Created:** 2026-09-16
> **Last Updated:** 2026-10-09

# Current Status

Current Phase: 6 — Batch, Hardening, Packaging

Current Stage: Complete

Current Step: P6-S05 — Samples, README, packaging

Current Objective: All phases complete. Project verified.

Overall Progress: P6-S05 In Progress. CI 10/10 user-reported green. AQ-012 FIXED (text-wins + zero-yield placeholders; Database 9KB/0-text → 3.47MB text). AQ-013 FIXED 2026-10-04 (vendored pdf-extract/CMap hardening; 16/16 retained pdf artifacts contained, verified 2026-10-09, suite 142/0/1). FUZZ GATE MET 2026-10-09 (csv 24.00h, rtf 24.01h, pdf 24.01h post-patch, zero new findings). Release Done still blocked — commit-remainder, reviews, tag pending.

Repository Status
- Current Branch: main (tracks origin/main, behind local by 1)
- Latest Commit: dca3ebc (AQ-011/AQ-012/link/guide batch, committed 2026-09-25, NOT pushed — push not requested)
- Working Tree: committed clean except pre-existing non-batch modifications (pdfium-setup.md, agent-archive.md, pdf_layout.rs) + user untracked files, all left untouched
- Last Successful Build: 2026-09-25 — `cargo build --locked` clean (zero warnings), toolchain 1.98.0.

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
| quick-xml | crates.io | 0.41.0 | Yes | Installed |
| clap | crates.io | 4.x | Yes | Installed |
| serde + serde_json | crates.io | 1.x | Yes | Installed |
| flate2 | crates.io | 1.x | Yes | Installed |
| walkdir | crates.io | 2.x | Yes | Installed |
| base64 | crates.io | 0.22.x | Yes | Installed |
| csv | crates.io | 1.x | Yes | Installed |
| encoding_rs | crates.io | 0.8.x | Yes | Installed |
| logos | crates.io | 0.15.x | Yes | Installed |
| calamine | crates.io | 0.36.1 | Yes | Installed |
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

Next Step: (h) DONE 2026-09-25 by user-authorized documented decision, blueprint unchanged — see Session Log. Then: initial commit + architecture/security reviews + tag (no tag until §14 evidenced).

Relevant Skills: docs, packaging, CI (`skills/docs.md`, `skills/packaging.md`, `skills/ci.md` loaded for close-out).

Acceptance Criteria: NOT MET — §14 items 1–2, 4–5, 7 without full evidence; fuzz hours MET 2026-10-09 (csv 24.00h, rtf 24.01h, pdf 24.01h post-patch, zero new findings); `samples_gen` covers representative only (enumeration still Architect-blocked, not claimed); CI YAML not run (both-profile runs unverified locally; pdf-layout needs native PDFium). Release evidence 2026-09-24: default release binary 4,303,552 bytes (4.1MiB, within ≤5MB; was 3.0M pre-pdf-extract) — `cargo build --locked --release --offline`, toolchain 1.98.0. Full suite 115 passed, 0 failed, 1 ignored (baseline 107/1 → +8: placeholder, reason, CLI precedence, OMML bound, DOCX header, date_tokens, scanned-Force, CLI batch).

Status: In Progress — 2026-09-24 remaining-issues batch: shared-Pdfium fix (multi-bind bug), A12 mapping, samples 25/25, both release sizes, perf evidence, CI SHA pins, fuzz harness + smoke runs; default 130/1 + layout 140/1 green. Still open: real-doc corpus, CI execution, commit/tag (staged), security-auditor referral (fuzz gate met 2026-10-09). 2026-09-25 reverify (default profile only): `cargo build --locked` zero warnings; `cargo test --locked` 130 passed, 0 failed, 1 ignored. 130-vs-131 DELTA RESOLVED 2026-09-25: `cargo test --locked -- --list` shows exactly 131 tests total (130 runnable + 1 ignored `write_samples`); the "131 passed" claim was loose shorthand for 131 total — no test missing, no code changed. (`rg` finds 144 `#[test]` attrs repo-wide; the balance is `cfg(feature = "pdf-layout")`-gated.)

# 9. Known Issues

See docs/implementation-notes.md for the detailed current list. Important unresolved issues:
- **[High, security audit 2026-09-25] quick-xml 0.38.4: RUSTSEC-2026-0194 + RUSTSEC-2026-0195 — RESOLVED same session: direct dep → 0.41 + calamine 0.31→0.36.1 (whose quick-xml ^0.41 unifies the tree; 0.38.4 gone from lock); suite 139/0/1 green with zero P2 changes (used API surface stable across the jump).**
- **[High, security audit 2026-09-25] ttf-parser 0.25.1 unmaintained (transitive via lopdf 0.42 ← pdf-extract 0.12.1, itself latest — no upstream fix available); warning-severity, documented residual, revisit on pdf-extract update.**
- **[Blocker, fuzz 2026-10-04 — RESOLVED 2026-10-09] pdf target was blocked at ~1.05 CPU-h (3772s, 22,678 runs): 15 libFuzzer crash artifacts in `fuzz/artifacts/fuzz_pdf/` (11x `pdf-extract 0.12.1 lib.rs:177:52 maybe_deref expect("missing object reference")` / ObjectNotFound; 3x `adobe-cmap-parser 0.4.1 lib.rs:169:31 failed to parse: Mismatch`; 1x `pdf-extract lib.rs:204:69 expect("wrong type")`). All three sites sit inside production `decode_whole`'s `catch_unwind` (`src/extract/pdf.rs:27`), but cargo-fuzz compiles with `-Cpanic=abort` so `catch_unwind` cannot contain them under fuzz (known rust-fuzz behavior) — production (panic=unwind, `Cargo.toml:42`) converts these to `Corrupt`, fuzz reports deadly-signal exit 77. FIXED 2026-10-04 (see AQ-013): vendored patch, 16/16 retained artifacts contained (verified 2026-10-09, zero signal deaths), pdf gate MET 2026-10-09 with a fresh post-patch 24h run: 86427s (24.01 CPU-h), 792,655 runs, corpus 1028, zero new crash/oom/timeout/leak via `scripts/fuzz_batch.sh --targets pdf`.**
- No build/test/API compatibility evidence or Cargo.lock.
- Streaming XML rewrite is source-inspected but uncompiled; API/ownership/runtime behavior and performance need user verification.
- Local images, navigation warning aggregation, basic table spans/wrapping and PPTX object order have source changes and new regressions. Complex Markdown grammar, advanced RTF/ODF/number-format cases and full fixture coverage remain pending.
- Not all blueprint-required fixture variants have test coverage.
- Output staging rolls back ordinary errors in source; power-loss atomicity, race-proof sandboxing and platform hard-link support are not verified.
- No dependency/security audit or corpus/CI evidence. Fuzz COMPLETE 2026-10-09: csv 24.00 CPU-h (86403s, 12,195,781 runs, corpus 5506, 0 crash/oom/timeout — slow-units only), rtf 24.01 CPU-h (86447s, 34,393,191 runs, corpus 31353, 0 crash/oom/timeout — slow-units only), pdf 24.01 CPU-h post-patch (86427s, 792,655 runs, corpus 1028, zero NEW crash/oom/timeout/leak across the counted window; 16 stale pre-patch crash artifacts retained in fuzz/artifacts/fuzz_pdf/, all dated ≤2026-10-04 and each contained — see 2026-10-09 log) per `fuzz/.batch-state/` + `TARGET DONE` log lines;

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

## AQ-011 — Asset-namespace length bound (long input filenames)

Blueprint Step: P0-S08 (`convert.rs` staging; "filename = stem + extension" with no length rule).

Question: `src/convert.rs:92` builds the per-file asset namespace as the raw stem for `[A-Za-z0-9_-]`-only stems, else `~` + full hex of the stem bytes — unbounded. A 162-char/164-byte real-world stem (corpus EPUB, strace-proven) yields a 329-byte single path component; `mkdir` fails ENAMETOOLONG (NAME_MAX 255) and the conversion dies with `I/O error: invalid filename`, exit 1. Any stem with non-alphanumeric chars past ~127 bytes is a guaranteed failure. Wanted: truncate (how many bytes, keep readability?), hash-suffix (which hash, collision handling?), or declared-limitation with a clear error naming the limit? Code untouched pending ruling — current behavior is honest (no partial output) but trips on ordinary long ebook filenames.

Status: Resolved 2026-09-25 by user "fix them" authorization — documented decision, blueprint unchanged: new `asset_namespace()` helper (`src/convert.rs`): readable stems pass through, other stems hex-encoded as before, either form capped at 200 bytes; overlong names fall back to `long-<32 readable chars>-<16-hex stem hash>` (deterministic, unique, always ≤54 bytes) + warning "long input filename shortened in asset paths" (pushed before report creation so it surfaces in reports). Pinned by 4 lib unit tests (passthrough/hex/bound/determinism) + e2e `long_stem_shortens_asset_namespace` (150-char stem with image asset → converts, shortened namespace, warning, asset present). Proven on the corpus repro: the 162-char-stem EPUB that died ENAMETOOLONG now exits 0 with md + report + `assets/long-HowtoBecomeanExpertSoftwareEngin-515fffd239d5d28e/image-001.jpg`. Full suite 135/0/1, zero warnings.

## AQ-012 — Empty-yield PDF pages are neither text nor placeholder (silent gap)

Blueprint Steps: P4-S01 (scanned detection), P4-S02 (NeedsOcr placeholders).

Question: Scanned detection flags NeedsOcr only when a page's content stream lacks text-showing operators. sam/ run proves 11 real scanned books (up to 1376 pages, 0 text lines in every output) yield ~1 flagged page each, while hundreds of sibling pages decode to zero text and emit a bare `Boundary(Page)` with no placeholder, no warning, and a report claiming "1 page(s) need OCR". Blank-by-design pages vs image-only pages need different treatment, and the discriminating signal plus the counting rule are unspecified — P4-S02 covers only already-flagged pages. Wanted: extend NeedsOcr to zero-yield non-trivial pages (placeholder + honest count), or declare bare-boundary emission acceptable with blueprint wording? Code untouched pending ruling — current behavior understates loss, which §0.1 forbids.

Status: Resolved 2026-09-25 by user "fix it" authorization — documented decision, blueprint unchanged, two-layer fix in `extract_pdf_flat` (`src/extract/pdf.rs`): (1) zero-yield substantial pages join the scanned set (placeholder + honest count) when attribution is aligned or the whole decode is empty — blank (trivial-stream) pages stay bare; (2) decoded text always wins over the scanned flag — the diag proved why: 478pp novel decoded 808,965 real chars with zero formfeeds, all attributed to page 1, which was operator-absent, so the old branch discarded the entire blob (9KB/0-text output). Pinned by `zero_yield_substantial_page_gets_placeholder` + `blank_page_stays_bare`. Proven: Database (1376pp) went 9KB/0-text/1-scan → 3.47MB real text, 0 placeholders, OCR warning correctly gone, 0 FFFD. Full suite 139/0/1, zero warnings; release 4,310,304 B.

## AQ-013 — PDF fuzz crashes: upstream fix/replace (pdf-extract/adobe-cmap-parser)

Blueprint Steps: P4-S01 (dependency + catch_unwind containment), P6-S05 (§14 fuzz gate).

Question: 15 pdf fuzz crashes (11x pdf-extract 0.12.1 `maybe_deref` expect ObjectNotFound lib.rs:177:52; 3x adobe-cmap-parser 0.4.1 expect lib.rs:169:31 CMap Mismatch; 1x pdf-extract expect("wrong type") lib.rs:204:69) sit inside `decode_whole` catch_unwind (`src/extract/pdf.rs:27`), uncatchable under cargo-fuzz `-Cpanic=abort`. Production (panic=unwind) returns `Corrupt`; fuzz reports deadly-signal exit 77.

Status: User ruled 2026-10-04 "Upstream fix/replace". FIXED same session as user-authorized P4-S01 drift correction, blueprint unchanged: vendored `patches/pdf-extract` (0.12.1) + `patches/adobe-cmap-parser` (0.4.1) with hostile-input hardening only (same versions, graceful-None/skip instead of expect/todo!/assert — dangling refs, malformed CMaps, unsupported Type0 encodings, hostile Tf/Do font/XObject refs, fontless text spans), wired via `[patch.crates-io]` in `Cargo.toml` + `fuzz/Cargo.toml` (both lockfiles updated). Proven: all 15 retained pdf crash artifacts decode without panic (14 OK + 1 graceful Corrupt → recheck 15/15 OK after final edits), 120s post-patch fuzz smoke 1171 runs/0 crashes, full suite 142/0/1 green (phase4 14/14 incl. new `dangling_tounicode_never_panics` tripwire + pre-existing `malformed_font_panic_is_contained` still green), 15 artifacts seeded into `fuzz/corpus/fuzz_pdf/` as regression seeds. Residual: unproven expect sites remain in the vendored code (content-stream operands, W-array bounds, MediaBox, Subtype dispatch) — fuzz will prove or not; pdf 24h gate MET 2026-10-09: 86427s (24.01 CPU-h), 792,655 runs, corpus 1028, zero new crash/oom/timeout/leak (16 stale pre-patch artifacts retained, 16/16 contained — see 2026-10-09 log). No commit, no tag.

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

Latest Test Run: 2026-10-09 — default 142 passed, 0 failed, 1 ignored (post-fuzz-gate re-verify; `pdf-extract` vendored lib emits unused-var warnings, cosmetic, patch code only). Prior: 2026-09-25 — default 141 passed (139 + parent_or_dot unit + bare-name e2e), 0 failed, 1 ignored, zero warnings, toolchain 1.98.0. Release 4,310,304 B (≤5MB, pre-bare-name-fix build). Prior evidence (not re-run today): `pdf-layout` profile 144 tests listed 2026-09-25 via `cargo test --locked --offline --features pdf-layout -- --list` (131 default + 13 layout-gated; pre-shadow-fix claim 140/1 is superseded — 3 shadow/hijack tests added since; execution still needs native PDFium, absent). Release evidence: default 4,307,952 B (4.11MiB ≤5MB); layout 4,188,216 B (3.99MiB) sha `248688cf…` (see docs/pdfium-setup.md). Fuzz smoke (nightly + cargo-fuzz 0.13.2, ASan): rtf 247,187 runs/91s, pdf 561,111/91s, csv 39,956/91s — zero crashes; §14 24h/target now met 2026-10-09 (see gate evidence above). Perf: 100 mixed files batch exit 0 in 18.3s wall (≪5min gate). CI YAML: 10 combos valid, actions pinned to SHAs (checkout 11bd596a, upload-artifact ea165f8d); workflow never executed (needs vars + runners). Corpus (11 real PDFs): 11/11 convertible — 8 default-clean (0 U+FFFD) + 3 via layout (Active/app-comb/Automate → md; cover doubling fixed, fragments poppler-confirmed genuine); 0 crashes — see Session Log. 2026-09-25 CI-equivalent local (linux-x86_64 default only): `cargo tree --locked` zero pdfium entries (pdf-extract 0.12.1 + lopdf 0.45/0.42); `cargo build --locked --release` 4,307,952 B within ≤5MB; `package_binary.py default` archive sha `7ba7d9f7…`, lock sha `4573af18…`; cli_docs drift test green (in 130/0/1 suite). Remote 10-combo matrix NOT executed (needs GitHub runners + RUST_TOOLCHAIN/PDFIUM_* vars); pdf-layout NOT re-run (native lib absent — chromium/ dir gone, repo-root .so known-incompatible). Fuzz smoke 2026-09-25 (nightly + cargo-fuzz 0.13.2, ASan, 90s/target): rtf 156,466 runs, pdf 476,309, csv 29,073 — zero crashes, no crash/oom/timeout artifacts; §14 24 CPU-hours/target MET 2026-10-09 (csv 24.00h, rtf 24.01h, pdf 24.01h post-patch — supersedes the 90s-smoke figures).

Coverage: Not measured. All test files executed successfully.

§9-row-to-test mapping (P6-S04 DoD, recorded 2026-09-25; blueprint names `tests/negative_suite.rs` but the suite lives in `tests/regressions.rs` + `tests/phase6_integration.rs` — CLOSED 2026-09-25 by user decision: keep current files, mapping below is the record, blueprint unchanged):
| §9 row | Test(s) |
| --- | --- |
| Malformed/weaponized ZIP | `negative_entrypoints_exact_errors` (truncated.pdf/bad.docx → Corrupt), `zip_case_collisions_are_rejected`, `empty_and_truncated`, `pk_empty_archive_signature_still_zip_branch` |
| Malformed XML | `deeply_nested_xml_bounded`, `xml_strict_rejects_mismatched_nesting`, `omml_deep_nesting_stays_bounded`, `xml_stream_order_and_cdata` |
| PDF parser bombs | `encrypted_pdf_exact_error`, `encrypted_pdf_convert_file_fails_without_partial_output`, fuzz_pdf smoke 476k runs zero crashes |
| Path traversal | `markdown_remote_or_traversing_images_remain_links`, `markdown_local_images_are_embedded_with_safe_paths`, `report_symlink_rejected_before_replacing_output` |
| Command injection (OCR) | `fake_engine_argv_and_cleanup` (argv array; `eng; echo NOT_A_SHELL` passed literally), `missing_engine_auto_preserves_image_force_errors` |
| Unbounded memory | `negative_entrypoints_exact_errors` size-guard case (`file exceeds --max-size`) |
| Output overwrite | `source_overwrite_stays_refused_after_renderer_changes`, `staged_overwrite_replaces_outputs_and_cleans_backups`, `report_conflict_does_not_write_partial_output`, `same_stem_collision_rejected_before_writes_race` |
| Batch DoS | `strict_one_worker_stops_queue`, `recursive_outputs_mirror_tree_and_isolate_errors`, `cli_batch_exit_codes_and_progress` |
| pdfium FFI (opt-in) | `missing_pdfium_is_actionable`, `layout_encrypted_reports_encrypted` (feature-gated) |
| N/A (auth/secrets/replay) | No tests by design — local single-user tool, no network (see Security Checklist) |

Known Failures: None.

# 14. Next Steps

1. DONE 2026-09-24: P4-S01 validated; C1 attribution guard; C2 OCR-warning gate. Suite 101/1.
2. NEXT single steps, in order (one at a time, each with regression suite): (a) DONE + AQ-008 resolved (delimiter-sniff candidates `,`/`;`/TAB/`|`, first-line count, tie/zero→default; detect 8/8). (b) DONE — AQ-006 resolved (honest `no rendered asset available` kept, `placeholder_honest_stub`) + AQ-007 resolved (`{ext}: {sorted-signals}`, `reason_string_rule`, phase0 14/14). (c) DONE — capped-read + AQ-009 resolved (13 `--no-*`/`--formulas`/`--comments`/`--strip-headers-footers` companions, docs synced, `cli_docs` + bin precedence test green). (d) DONE 2026-09-24 — OMML depth-64 guard added (`omml::go` depth param, `omml_deep_nesting_stays_bounded`); DOCX bold-header rule implemented for ns='w' only (`row_bold`, `docx_table_bold_header_rule`, merge fixture corrected to bold); RTF depth KEPT as warning+Document per P1-S05 instruction 7 + `rtf_malformed_battery` (security `Corrupt` sentence documented as superseded for content errors). Suite 112/1. (e) DONE 2026-09-24 — P2 date-format→Date map (`is_date_format`, `%` excluded, brackets/literals stripped; numeric+date-fmt → `CellKind::Date`) + merged-dedup in xlsx/ods fidelity; `date_tokens` unit test + phase2 6/6 green. EXTENDED 2026-09-24 audit (C8): numeric+date-fmt now also converts display serial→ISO 8601 (`serial_to_iso`, known-value unit tests + `xlsx_number_format_date_converts_display`); error cells use Debug format per P2-S01. (f) DONE 2026-09-24 — default-Force documented AD-15 (`pdf.rs`, any mode → placeholder + OCR_BUILD); `scanned_force_default_still_placeholder` (placeholder + Page boundary + warning); image empty/1-byte → None asserts; phase4 8/8 + phase5 4/4 green. EXTENDED 2026-09-24 audit: +2 convert_file seam tests (scanned-ok, encrypted-no-partial); phase4 10/10. (g) DONE 2026-09-24 — precedence test pins P0-S08 order CLI > config > profile (documents P6-S03 prose swap of middle two); negative suite expanded with §9-mapped encrypted/OLE2/unknown/mismatch-report cases; CLI batch exit-code test (mixed→1 with isolation, all-valid→0); phase6 7/7 green. EXTENDED 2026-09-24 audit: nine-profile effect assertions, XLS-not-rejection cross-ref, deep-XML pin; phase6 9/9. ALSO DONE 2026-09-24 audit: C4 lower-dedup (+test), C3 pdf_path default|layout, C5 img src in warning, C11 chart test, C12 PPTX table+chart test, C15 OMML coverage. (h) OPEN — samples_gen every-fixture needs Architect enumeration (representative-only; not claimed complete). (i) DONE 2026-09-24 — default release 4,303,552 B (4.1MiB ≤5MB), toolchain 1.98.0, suite 126/1 after audit fixes; pdf-layout build + CI runs + fuzz + corpus still without evidence.
3. Architect calls flagged: xml.rs scope vs arch table; layout-Force semantics; shallow scanned detection depth; precedence-test 3-value design; unwind-vs-abort doc update; dead variants (Zip/Other/export_charts/duplicate_of); OCR timeout/sandbox; CI SHA pins; security-auditor.md referral.
4. Then: initial commit + architecture/security reviews + tag (no tag until §14 evidenced).

# 15. Session Log

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

# 16. Review History

| Reviewer | Blueprint Version | Major Findings | Required Fixes | Status |
| --- | --- | --- | --- | --- |
| Orchestrator | v2.0 (merged) | Dependency version mismatches prevented build | lopdf 0.36→0.45, calamine 0.28→0.31 | **Closed** |
| Orchestrator | v2.0 (merged) | All 42 blueprint steps verified | None | **Closed** |
| self-audit | v2.0 (merged) | Phase-by-Phase + Full-Project + code-level audit: 8 findings, 4 fixed (git, status, security checklist, .gitignore), 0 code bugs, 3 design-level flags | Git init, status correction, security checklist, doc verification | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-24 audit-and-fix session: phase-by-phase + full-project code-level re-audit incl. unvalidated P4-S01 pdf-extract change | Fixed C1 (P4-S02 attribution guard), C2 (P5-S03 OCR warning gate); P4-S01 validated 96→101 passed; remainder flagged as queued single steps (see log) | **Closed** |
| self-audit | v2.0 (merged) | Scope: Phase-by-Phase + Full-Project (code-level, all 42 steps + cross-cutting). Architect/designer role files absent — architecture.md/blueprint.md used as Architect decisions per equivalent-file rule; Designer N/A (headless CLI, arch §1) | Fixed 15 clear findings (C1–C15, see Session Log); 22 items flagged for Architect (A1–A22, see report in log); security items referred to security-auditor.md | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-25 audit-and-fix session: third full pass; focus on post-2026-09-24 changes (AQ-011/012, link fallback, ocr allow, CI/fuzz/batch-test scripts, docs) + regression sweep | Fixed F1 (fuzz run-output gitignore); no new compliance/quality findings; standing referrals unchanged | **Closed** |
| self-audit | v2.0 (merged) | 2026-09-25 audit-and-fix session: fourth full pass; focus on post-third-audit changes (dep bumps quick-xml/calamine, bare-name fix, security/release reports) + regression sweep | TBD (see Session Log) | **Open** |
