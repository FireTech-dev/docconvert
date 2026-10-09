# Project Agent

> **Project:** docconvert
> **Version:** 1.0
> **Maintained By:** Orchestrator & Implementer
> **Created From:** `project/agent.template.md`
> **Status:** Complete — v1.0 tagged + pushed + RELEASED 2026-10-09 (release `v1.0`, id 407834593: 10 platform tarballs + 10 manifests + SHA256SUMS from tag-CI run 37913020914); owner-approved with documented caveats (below); confirmatory release re-audit APPROVED 2026-10-09 (23 pass / 0 Blocking / 1 Advisory / 4 N/A)
> **Created:** 2026-09-16
> **Last Updated:** 2026-10-09

# Current Status

Current Phase: 6 — Batch, Hardening, Packaging

Current Stage: Complete

Current Step: P6-S05 — Samples, README, packaging

Current Objective: All phases complete. Project verified.

Overall Progress: Project Complete (header). Tag-CI 10/10 GREEN user-reported 2026-10-09. AQ-012 FIXED (text-wins + zero-yield placeholders; Database 9KB/0-text → 3.47MB text). AQ-013 FIXED 2026-10-04 (vendored pdf-extract/CMap hardening; 16/16 retained pdf artifacts contained, verified 2026-10-09, suite 142/0/1). FUZZ GATE MET 2026-10-09 (csv 24.00h, rtf 24.01h, pdf 24.01h post-patch, zero new findings). v1.0 TAGGED+PUSHED 2026-10-09 (annotated tag on bb50bf7; main + tag in sync with origin). v1.0 RELEASE PUBLISHED 2026-10-09 (release id 407834593: 10 platform tarballs + 10 build manifests + SHA256SUMS, sourced byte-identical from tag-CI run 37913020914, 10/10 green). Security re-audit clean; confirmatory release re-audit APPROVED 2026-10-09 (`project/release-audit-2026-10-09-confirmatory.md`: 23 pass / 0 Blocking fail / 1 Advisory (owner-waived corpus caveat) / 4 N/A).

Repository Status
- Current Branch: main (in sync with origin/main at bb50bf7; tag v1.0 pushed)
- Latest Commit: bb50bf7 (dual-license + README/state batch, committed + pushed 2026-10-09)
- Working Tree: committed clean except this log entry + user untracked files (package.json/lock, sam/, roles/documenter.md, roles/readme.template.md, 2026-09-25 audit reports), all left untouched
- Last Successful Build: 2026-10-09 — `cargo build --locked --offline` clean on committed tree c17171d (vendored pdf-extract lib warnings only, cosmetic), toolchain 1.98.0.

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
| Post-closure | Release run 2026-09-23 → 2026-10-09: close-out, audits, 24h fuzz gate, vendored hardening, tag + Release v1.0 (21 assets), confirmatory APPROVED | 2026-10-09 | Closed |

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
- Cross-platform build/API compatibility evidence still pending (local linux-x86_64_default only).
- Streaming XML rewrite compiled, tested, suite-pinned, and committed; platform-port verification still pending.
- Local images, navigation warning aggregation, basic table spans/wrapping and PPTX object order have source changes and new regressions. Complex Markdown grammar, advanced RTF/ODF/number-format cases and full fixture coverage remain pending.
- Not all blueprint-required fixture variants have test coverage.
- Output staging rolls back ordinary errors in source; power-loss atomicity, race-proof sandboxing and platform hard-link support are not verified.
- Dependency (`cargo audit` 0 vulnerabilities) and security (re-audit 2026-10-09, 0 blocking) evidence recorded; corpus/CI gaps remain. Fuzz COMPLETE 2026-10-09: csv 24.00 CPU-h (86403s, 12,195,781 runs, corpus 5506, 0 crash/oom/timeout — slow-units only), rtf 24.01 CPU-h (86447s, 34,393,191 runs, corpus 31353, 0 crash/oom/timeout — slow-units only), pdf 24.01 CPU-h post-patch (86427s, 792,655 runs, corpus 1028, zero NEW crash/oom/timeout/leak across the counted window; 16 stale pre-patch crash artifacts retained in fuzz/artifacts/fuzz_pdf/, all dated ≤2026-10-04 and each contained — see 2026-10-09 log) per `fuzz/.batch-state/` + `TARGET DONE` log lines;

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

Session Log archived to `project/agent-archive.md` ("Post-Closure Release Run" section) on 2026-10-09 at v1.0 approval; this section starts fresh.

## 2026-10-09 — Codebase cleanup + state commit (Orchestrator)

Session Summary: User said "cleanup the codebase dir", then selected all four scopes when asked: /tmp staging, cargo target/, fuzz run state, commit pending state. Removed: `/tmp/opencode/release-v1.0` + `/tmp/opencode/audit-v1.0` (~25MB staging), root `target/` (2.4G), `fuzz/target/` (1.1G), `fuzz/logs/*` (49M) — ~3.6G total, all regenerable. Deliberately kept (stated before acting): `fuzz/artifacts/` (16 retained crash files carry a standing do-not-delete order as triaged evidence) and `fuzz/corpus/` (regression seeds whose Ebooks/ sources are gone from the workspace). User files (`sam/`, `package.json`, roles additions) untouched. Staged + committed `project/agent.md`, `project/agent-archive.md`, `project/release-audit-2026-10-09-confirmatory.md` (state-only; code tree already verified at `bb50bf7`, no build re-run needed). Left uncommitted per prior deferrals: 2026-09-25 audit reports, user files. No push (not instructed).

Repository Status: committed state ahead of origin/main; tag v1.0 + Release published; confirmatory APPROVED.

## 2026-10-09 — Final archival: session log + review history compacted (Orchestrator)

Session Summary: User said "archive". Applied the Orchestrator phase-archive procedure as the v1.0 close-out: moved all 54 Session Log entries (2026-09-16 → 2026-10-09) verbatim into `project/agent-archive.md` under "Post-Closure Release Run", and compacted Review History (7 Closed rows moved to archive "Closed Reviews"; most-recent fifth-pass row kept). Added the post-closure summary row under Archived Phases. No source, test, doc, or config changes.

Repository Status: main + tag v1.0 in sync with origin; Release v1.0 published (21 assets); confirmatory re-audit APPROVED; agent.md + confirmatory report uncommitted; user untracked files untouched.


# 16. Review History

| Reviewer | Blueprint Version | Major Findings | Required Fixes | Status |
| --- | --- | --- | --- | --- |
| self-audit | v2.0 (merged) | 2026-10-09 audit-and-fix session: fifth pass; focus on post-fourth-audit delta (c17171d: vendored pdf-extract/CMap hardening, parent_or_dot, dep bumps, fuzz gate) + regression sweeps | Fixed Q1 (vendored-crate warnings 30→0, behavior-neutral; suite 142/0/1 + 16/16 containment re-proven). No new compliance findings; residual vendored expects retired by 24h fuzz evidence. Standing referrals unchanged | **Closed** |
