# Implementation notes — Phases 0–6 source candidate, revision 3

## Current source batch

User authorized Phases 4–6, retaining both PDF profiles and prioritizing optional 4.B.
No application, Rust toolchain, dependency installation, build, formatter, tests or
fixture generation was executed by the assistant. Published API documentation and
file contents were inspected. All runtime acceptance remains open.

New source: default PDF decoding and scanned detection; feature-gated PDFium metrics,
layout heuristics, rendered image assets and raster OCR; seven image header readers;
Tesseract-protocol adapter; recursive structure-preserving batch worker queue;
profile/CLI/error tests; representative fixture-generation tests; docs/CI/package
helper source. Refer to format-quirks.md and architecture-addenda.md for exact limits.

### Open Phase 4–6 gaps (not waived)

- Full layout fidelity and required P4-S12 before/after goldens/corpus are incomplete.
  Several code paths are heuristic and only some have isolated tests. Exact table,
  caption/column, font, image, list/code and math behavior needs native verification.
- The decoder panic test injects a panic; no pathological-font fixture proves an
  actual dependency panic. Native faults/OOM/stack aborts are not contained.
- Default scanned detection is operator-presence based, not recursive Form-XObject
  analysis. Limits after dependency loading/decoding cannot prove bounded allocation.
- OCR blocking capture has no timeout or bounded stdout/stderr; error-path cleanup
  tests do not yet exhaust every spawn/read/write failure. Empty OCR may retain a
  placeholder; confidence stays None by design.
- Negative catalogue and all profile end-to-end cases are not exhaustive; samples
  generator covers representative formats, not every blueprint fixture. Real scanned
  image PDF and fully encrypted cryptographic fixtures remain release-corpus work.
- CI source exists but was not run; it requires a committed lockfile, exact toolchain,
  reviewed native libraries/hashes and available target runners. No binary-size,
  dependency audit, fuzz-hour, license audit, v1.0 tag or release evidence exists.
- Plain Rust default/layout API compatibility remains unverified. Published signatures
  were inspected for pdfium-render 0.9.4, lopdf 0.45.0, pdf-extract 0.12.0; these are
  not resolved versions. pdf-extract currently documents a different transitive lopdf
  requirement; duplicate dependency versions and the size budget need measurement.

### Preserved Phase 0–3 revision-2 findings

The earlier record below remains applicable unless the new sections explicitly
supersede it. Source availability is not proof that any phase is complete.

## Historical revision-2 source record

## Authorization and evidence

The user selected **Finish Phase 0–3 gaps**, retaining source-only restrictions. No dependency installation/resolution, lockfile generation, Rust/Cargo invocation, compilation, application execution or tests were performed. There is no successful runtime result to report, and no phase is marked acceptance-complete.

Original uploaded architecture, blueprint, role instructions and templates are unchanged. Previous source-batch history remains in project/agent.md.

## Changes made in this revision

### Streaming XML — previous architecture discrepancy addressed in source

`extract/package.rs` no longer defines a recursive `Node` tree or retains child-node containers. It now uses:

- A quick-xml streaming validation/normalization pass with at most 64 open tag names.
- One shared, bounded normalized byte buffer per XML part.
- Immutable `Element` views with a tag name, attributes and byte range, not parent/child pointers or cached subtrees.
- Pull iterators for content, children and descendants, which parse and discard events as consumed.
- Explicit metadata maps only where the extraction contract needs them, such as style IDs, relationship IDs and EPUB manifests.

HTML recovery uses streaming writes for void tags, unmatched closing tags and EOF closures. XML nesting mismatches in strict package parts are rejected rather than silently corrected. The source still reads bounded parts into memory, as required by the byte-buffer extractor APIs; “no DOM” does not mean there is no source byte buffer. Iterators may rescan bounded byte ranges, so time/memory behavior requires user measurements. This change has not been compiled or reviewed by an independent Architect.

### Fidelity and data preservation

- Updated every XML consumer to the streaming-view API.
- EPUB footnotes are extracted in EPUB-aware HTML traversal, not by deleting tree nodes. Links to notes in other spine chapters are resolved against their actual package paths.
- HTML handles inline images at their inline position and reports the exact aggregated `skipped N navigation/interface sections` warning. Named entity preprocessing is one-pass.
- Markdown has a safe injected local-image loader, literal fallback for non-punctuation escapes, multi-backtick code spans, hard breaks and mixed nested list grouping/continuations.
- PPTX picture, text, table and chart objects are processed in spTree order; group shapes are traversed. Titles retain their prescribed top-of-slide position. Notes use notes-part relationships.
- Shared table rendering expands omitted vertical-merge continuations so later columns do not shift left. DrawingML rowSpan handling no longer double-increments declared rowspans.
- Markdown table cells escape pipes even inside inline code. TXT cells wrap at the 32-character cap rather than discarding characters with an ellipsis.
- The tts profile's minimal-placeholder setting now affects TXT rendering, retaining available alt/caption text.
- ODS repeated cell/row formulas, comments, hyperlinks and merge metadata are expanded under the preflight cell-count bound; external styles.xml table visibility is considered. Embedded objects are identified as charts only when their package content indicates charts.
- RTF hex pictures are finalized at implicit EOF closes, and nested picture groups contribute bytes without duplicating assets. Field-instruction text is accumulated before hyperlink extraction.
- Malformed existing optional DOCX XML parts produce a warning.

### Detection and output safety

ZIP detection now inspects central-directory names and a maximum 256-byte mimetype payload, rather than decompressing the whole package during detection. Actual extraction retains entry count, per-entry, total decompression and XML complexity limits. Case-colliding archive names are rejected.

Output/report/asset writes are staged before publication. Ordinary commit failures trigger rollback; existing outputs are backed up before replacement. Staging files are removed by cleanup guards. Publication uses same-filesystem hard links to avoid overwriting an unexpected newly created destination. Filesystems without hard-link support fail instead of silently falling back to a weaker write path.

This is **not** an atomic multi-file transaction across power loss, process termination or a hostile concurrent filesystem actor. If rollback fails, `.docconvert-backup-*` files are retained and the error requests recovery. If backup cleanup fails after a successful commit, an error reports that outputs were committed. Initial tests should still use copies and a fresh output directory.

Explicit Markdown-to-Markdown conversion bypasses Markdown parsing and copies bytes unchanged. Source-relative links remain unchanged by design; a report warning tells users to keep referenced files available beside the copy. Asset reorganization is not silently applied to passthrough Markdown.

## Still pending — not a claim of full Phase 0–3 completion

1. **All builds and tests remain unrun.** Rust type/API compatibility, fixture validity and expected output must be checked by the user. The wildcard manifest still needs current-stable resolution and a committed Cargo.lock.
2. **Full fixture coverage is not complete.** The new regression target adds streaming, cross-chapter notes, picture/table/chart order, repeated ODS metadata, cell wrapping, output staging and OMML cases. It is not a substitute for every phase-exit fixture and a real-world corpus.
3. **Markdown edge grammar remains partial.** Nested/overlapping emphasis delimiters, fully general link destinations and CommonMark combinations beyond the specified simple parser need further work and goldens. Passthrough relative-reference portability is explicitly warned, not rewritten.
4. **RTF edge fidelity remains partial.** Binary picture payloads, advanced list/table controls, unusual multi-token escaped field instructions and multi-byte fallback subtleties are not fully implemented. The blueprint's contradictory deep-group handling still uses warning + text fallback rather than claiming both Corrupt and successful Document behavior.
5. **Spreadsheet/native-version gaps remain.** Calamine controls legacy XLS decoding and shared formula recovery; XLS merge/comment/link/chart gaps are warned. Complete number-format fidelity, inherited ODF visibility/style chains, formulas in repeated empty cells and all drawing variants need more fixtures/review.
6. **Other format corners remain.** Nested HTML tables/complex browser recovery, DOCX numbering overrides and nested non-core structures, ODT image/style cases, and pathological merge layouts are not fully covered.
7. **Runtime/security/release evidence is absent.** No independent audit, timing measurement, power-loss recovery test, platform matrix, fuzzing or corpus evaluation occurred. Symlink checks are not a race-proof sandbox. Do not use source inspection as proof of security.

## Retained implementation decisions

- All CDM types implement Serialize; applicable enums are Copy and defaults are explicit.
- `text_len` counts Unicode scalars in body text and footnotes, not metadata, asset bytes or duplicate workbook data. `walk_inlines_mut` is postorder; replacement children are not revisited.
- Config precedence: defaults → profile → config file → explicitly supplied CLI options, following P0-S08 and architecture §5 rather than the contradictory P6-S03 example.
- Asset names are scoped as `assets/<stem>/image-NNN.ext`; unsafe stems use a reversible hex namespace.
- Source overwrite is refused even with `--overwrite`. Chart placeholders without assets honestly say no rendered asset is available.
- Bounds: 10,000 ZIP entries, 64 MiB per decompressed member/XML source, 200 MiB decompressed package total, 64 XML nesting levels, 500,000 XML elements; spreadsheet expanded-cell preflight bound is 2,000,000.
- All declared dependencies appear in the supplied register. None was installed, resolved or audited by the assistant.

## Dependency documentation evidence retained from revision 1

Calamine published documentation showed **0.36.1**, not a locally resolved version:

- https://docs.rs/calamine/0.36.1/calamine/trait.Reader.html
- https://docs.rs/calamine/0.36.1/calamine/fn.open_workbook_auto_from_rs.html
- https://docs.rs/calamine/0.36.1/calamine/enum.Data.html
- https://docs.rs/calamine/0.36.1/calamine/struct.ExcelDateTime.html

The implementation uses the documented reader/formula/value/date APIs. Actual resolved versions and a lockfile hash must come from the user. See RUN_TEST_GUIDE.md.
