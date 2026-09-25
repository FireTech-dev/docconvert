# Project skills — researched reference pack

Researched/written **2026-09-16** using the supplied Skill Writer role and exact
section order from `project/skill.template.md`. This is reference material, not an
application implementation, acceptance report or claim of universally “best” practice.

## Coverage

**34 skills:** 32 distinct detailed-blueprint topics plus ZIP container anatomy and
property-based/golden testing explicitly named in architecture §12. Aliases in the
roadmap/component map are consolidated rather than duplicated. No speculative
skills or design-system skill were added to this headless CLI project.

Read the mapped files before implementing a step. `none new` inherits prior shared
skills; it does not mean no expertise is needed. All checklists remain unchecked:
writing guidance does not prove the source conforms to it.

## Skill library

| Skill | File | Referenced steps |
| --- | --- | --- |
| Rust core | [rust-core.md](rust-core.md) | P0-S01, P0-S02, P0-S03 |
| File-format signatures | [file-format-signatures.md](file-format-signatures.md) | P0-S02 |
| ZIP container anatomy | [zip-container-anatomy.md](zip-container-anatomy.md) | P1-S01, P1-S02, P1-S03, P1-S04, P2-S01, P2-S02, P2-S04, P3-S01, P3-S02 |
| Parsing | [parsing.md](parsing.md) | P0-S04, P0-S05 |
| Spreadsheet data modeling | [spreadsheet-data-modeling.md](spreadsheet-data-modeling.md) | P0-S05, P2-S01, P2-S02, P2-S04 |
| HTML/XHTML semantics | [html-semantics.md](html-semantics.md) | P0-S06, P1-S03 |
| Streaming XML | [streaming-xml.md](streaming-xml.md) | P0-S06 |
| Markdown/ASCII layout | [markdown-ascii-layout.md](markdown-ascii-layout.md) | P0-S07 |
| CLI/config design | [cli-design.md](cli-design.md) | P0-S08 |
| Orchestration | [orchestration.md](orchestration.md) | P0-S08 |
| OOXML shared packaging and helpers | [ooxml.md](ooxml.md) | P1-S01 |
| OOXML WordprocessingML | [ooxml-wordprocessingml.md](ooxml-wordprocessingml.md) | P1-S02 |
| EPUB/OPF packaging | [epub-opf-packaging.md](epub-opf-packaging.md) | P1-S03 |
| ODF (OpenDocument XML) | [odf.md](odf.md) | P1-S04, P2-S04 |
| RTF control-word grammar | [rtf-control-word-grammar.md](rtf-control-word-grammar.md) | P1-S05 |
| OOXML SpreadsheetML | [ooxml-spreadsheetml.md](ooxml-spreadsheetml.md) | P2-S02 |
| OOXML PresentationML | [ooxml-presentationml.md](ooxml-presentationml.md) | P3-S01, P3-S02 |
| Integration testing | [integration-testing.md](integration-testing.md) | P1-S06, P2-S05, P3-S03, P4-S03, P4-S12, P5-S03, P6-S03 |
| Property-based / golden testing | [property-golden-testing.md](property-golden-testing.md) | P1-S06, P2-S05, P3-S03, P4-S03, P4-S12, P6-S04, P6-S05 |
| PDF internals | [pdf-internals.md](pdf-internals.md) | P4-S01, P4-S04 |
| Text heuristics | [text-heuristics.md](text-heuristics.md) | P4-S02 |
| FFI and native-library boundaries | [ffi.md](ffi.md) | P4-S04 |
| PDF layout/typography | [pdf-layout-typography.md](pdf-layout-typography.md) | P4-S05, P4-S06, P4-S07, P4-S08, P4-S09, P4-S10, P4-S11 |
| Unicode | [unicode.md](unicode.md) | P4-S11 |
| Image header formats | [image-header-formats.md](image-header-formats.md) | P5-S01 |
| Process spawning | [process-spawning.md](process-spawning.md) | P5-S02 |
| Temp-file handling | [temp-file-handling.md](temp-file-handling.md) | P5-S02 |
| Batch processing | [batch-processing.md](batch-processing.md) | P6-S01, P6-S02 |
| Filesystem traversal | [filesystem-traversal.md](filesystem-traversal.md) | P6-S01 |
| Concurrency | [concurrency.md](concurrency.md) | P6-S02 |
| Security testing | [security-testing.md](security-testing.md) | P6-S04 |
| Documentation | [docs.md](docs.md) | P6-S05 |
| Packaging | [packaging.md](packaging.md) | P6-S05 |
| Continuous integration | [ci.md](ci.md) | P6-S05 |

## Exact detailed-step mapping

| Step | Original Relevant Skills | Read |
| --- | --- | --- |
| P0-S01 | Rust core | [rust-core](rust-core.md) |
| P0-S02 | Rust core, file-format signatures | [rust-core](rust-core.md), [file-format-signatures](file-format-signatures.md) |
| P0-S03 | Rust core | [rust-core](rust-core.md) |
| P0-S04 | parsing | [parsing](parsing.md) |
| P0-S05 | parsing, spreadsheet data modeling | [parsing](parsing.md), [spreadsheet-data-modeling](spreadsheet-data-modeling.md) |
| P0-S06 | HTML semantics, streaming XML | [html-semantics](html-semantics.md), [streaming-xml](streaming-xml.md) |
| P0-S07 | Markdown/ASCII layout | [markdown-ascii-layout](markdown-ascii-layout.md) |
| P0-S08 | CLI design, orchestration | [cli-design](cli-design.md), [orchestration](orchestration.md) |
| P1-S01 | OOXML | [ooxml](ooxml.md) |
| P1-S02 | OOXML WordprocessingML | [ooxml-wordprocessingml](ooxml-wordprocessingml.md) |
| P1-S03 | EPUB/OPF packaging, HTML semantics (reused) | [epub-opf-packaging](epub-opf-packaging.md), [html-semantics](html-semantics.md) |
| P1-S04 | ODF | [odf](odf.md) |
| P1-S05 | RTF control-word grammar | [rtf-control-word-grammar](rtf-control-word-grammar.md) |
| P1-S06 | integration testing | [integration-testing](integration-testing.md) |
| P2-S01 | spreadsheet data modeling | [spreadsheet-data-modeling](spreadsheet-data-modeling.md) |
| P2-S02 | OOXML SpreadsheetML (fallback paths only), spreadsheet data modeling | [ooxml-spreadsheetml](ooxml-spreadsheetml.md), [spreadsheet-data-modeling](spreadsheet-data-modeling.md) |
| P2-S03 | none new | [spreadsheet-data-modeling](spreadsheet-data-modeling.md) |
| P2-S04 | ODF, spreadsheet data modeling | [odf](odf.md), [spreadsheet-data-modeling](spreadsheet-data-modeling.md) |
| P2-S05 | integration testing | [integration-testing](integration-testing.md) |
| P3-S01 | OOXML PresentationML | [ooxml-presentationml](ooxml-presentationml.md) |
| P3-S02 | OOXML PresentationML | [ooxml-presentationml](ooxml-presentationml.md) |
| P3-S03 | integration testing | [integration-testing](integration-testing.md) |
| P4-S01 | PDF internals | [pdf-internals](pdf-internals.md) |
| P4-S02 | text heuristics | [text-heuristics](text-heuristics.md) |
| P4-S03 | integration testing | [integration-testing](integration-testing.md) |
| P4-S04 | PDF internals, FFI | [pdf-internals](pdf-internals.md), [ffi](ffi.md) |
| P4-S05 | PDF layout/typography | [pdf-layout-typography](pdf-layout-typography.md) |
| P4-S06 | PDF layout/typography | [pdf-layout-typography](pdf-layout-typography.md) |
| P4-S07 | PDF layout/typography | [pdf-layout-typography](pdf-layout-typography.md) |
| P4-S08 | PDF layout/typography | [pdf-layout-typography](pdf-layout-typography.md) |
| P4-S09 | PDF layout/typography | [pdf-layout-typography](pdf-layout-typography.md) |
| P4-S10 | PDF layout/typography | [pdf-layout-typography](pdf-layout-typography.md) |
| P4-S11 | PDF layout/typography, Unicode | [pdf-layout-typography](pdf-layout-typography.md), [unicode](unicode.md) |
| P4-S12 | integration testing | [integration-testing](integration-testing.md) |
| P5-S01 | image header formats | [image-header-formats](image-header-formats.md) |
| P5-S02 | process spawning, temp-file handling | [process-spawning](process-spawning.md), [temp-file-handling](temp-file-handling.md) |
| P5-S03 | integration testing | [integration-testing](integration-testing.md) |
| P6-S01 | batch processing, filesystem traversal | [batch-processing](batch-processing.md), [filesystem-traversal](filesystem-traversal.md) |
| P6-S02 | concurrency, batch processing | [concurrency](concurrency.md), [batch-processing](batch-processing.md) |
| P6-S03 | integration testing | [integration-testing](integration-testing.md) |
| P6-S04 | security testing | [security-testing](security-testing.md) |
| P6-S05 | docs, packaging, CI | [docs](docs.md), [packaging](packaging.md), [ci](ci.md) |

## Start with these records

- [Research method, sources and limits](RESEARCH.md)
- [Architect questions and source deviations](ARCHITECT-QUESTIONS.md)
- [Machine-readable research evidence](research-sources.json)
- [Machine-readable coverage map](coverage.json)
- [Static review of this reference pack](VALIDATION.md)

The existing source-only restrictions remain in force. No Rust/Cargo/application,
OCR engine, native library, fixture generator, CI job or test was executed to create
this pack. Python/file utilities were used only for documentation research, writing,
coverage checks and archive inspection. Refer to `../RUN_TEST_GUIDE.md` for the
application’s future user-run validation; this skills task does not close its gaps.
