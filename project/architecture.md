# docconvert — System Architecture

**Version 2.0 (merged)** · Date: 2026-09-14 · Status: Approved

This document supersedes both prior drafts: the multi-crate, `pdfium-render`-based design
(previously "Version 3.0") and the lean single-binary, ≤8-dependency design (previously
"Version 1.0"). Neither is used going forward — this is the one architecture document for the
project. Every dependency decision below was checked against the live crates.io registry and
current GitHub state on 2026-09-14 (see §11, Architectural Decisions), not carried forward
from either prior draft on assumption. Detailed per-format algorithms, the full CDM type
definitions, and phased build order live in the companion document `blueprint.md`.

Note on the prior "spec §N" citations: one earlier draft was written against an external
28-section requirements specification that was never provided in this merge. Where that
draft's rules were spelled out inline (renderer escaping rules, detection signals, etc.) they
are preserved here as the actual source of truth. Where a rule existed only as a bare "spec
§N" pointer with no inline content, it has been either resolved explicitly below or flagged as
an open question — nothing is silently inherited from a document this project doesn't have.

---

## 0. Project Definition

### 0.1 Mission

`docconvert` is an **offline-first, Rust-based, general-purpose document and structured-data
conversion engine** that accepts a wide range of document, presentation, and spreadsheet
formats and converts them into either **Markdown (`.md`)** or **plain text (`.txt`)**,
preserving meaningful content, structure, notation, and relationships with minimal information
loss.

> **Use TXT when plain text is sufficient. Use Markdown when structure, formatting, or complex
> content needs to be preserved. Never silently discard meaningful information.**

> **For spreadsheets: preserve the workbook's meaningful organization, values, formulas, and
> visual assets where possible, while producing a readable Markdown or plain-text
> representation.**

The project is domain-independent: novels, textbooks, research papers, presentations,
technical manuals, legal documents, business reports, programming documentation, scientific
material, financial workbooks, and data exports all go through the same general conversion
system. It is inspired by tools like Microsoft's MarkItDown but extends the architecture to
support richer structure preservation, multiple output modes, spreadsheet data, and more
reliable handling of difficult documents.

### 0.2 Supported Input Formats

| Category      | Format      | Module            | Status   |
|---------------|-------------|--------------------|----------|
| Document      | TXT / MD    | `extract/text.rs`  | Core (Phase 0) |
| Spreadsheet   | CSV / TSV   | `extract/csv.rs`   | Core (Phase 0) |
| Document      | HTML/XHTML  | `extract/html.rs`  | Core (Phase 0) |
| Document      | DOCX        | `extract/docx.rs`  | Core (Phase 1) |
| Document      | EPUB        | `extract/epub.rs`  | Core (Phase 1) |
| Document      | ODT         | `extract/odt.rs`   | Core (Phase 1) |
| Document      | RTF         | `extract/rtf.rs`   | Core (Phase 1) |
| Spreadsheet   | XLSX / XLS  | `extract/xlsx.rs`  | Core (Phase 2) — both via `calamine`, see AD-3/AD-14 |
| Spreadsheet   | ODS         | `extract/ods.rs`   | Core (Phase 2) |
| Presentation  | PPTX        | `extract/pptx.rs`  | Core (Phase 3) |
| Document      | PDF         | `extract/pdf.rs`   | Core (Phase 4) |
| Other         | Images      | `extract/image.rs` | Extended (Phase 5) |
| Document      | Generic XML | `extract/xml.rs`   | Extended (Phase 6) |
| Legacy        | DOC/PPT (OLE2) | `detect.rs` only | Detect-and-report, not converted (AD-5) |

Format detection is multi-signal — magic bytes, ZIP-package structure inspection, OLE2
signature, then extension fallback (blueprint §Phase 0, `detect.rs`) — never extension alone.

### 0.3 Dual Output System

**Plain text (`.txt`)** — for documents whose meaningful information is primarily ordinary
text: novels, essays, articles, letters, simple notes, text-heavy reports. Output is clean and
readable without unnecessary Markdown symbols. For spreadsheets, TXT produces aligned
fixed-width tables with plain values.

**Markdown (`.md`)** — for documents containing information that benefits from structured
representation: tables, images, diagrams, mathematical formulas, source code, complex
headings, lists, links, captions, footnotes, references. For spreadsheets, Markdown produces
GFM tables with preserved formulas and chart references.

### 0.4 Automatic Output Selection

A scoring heuristic over the extracted `Document` (tables, images, code blocks, math, links,
footnotes, nested lists, multiple heading levels, slide/chapter boundaries, workbook presence)
recommends `.md` or `.txt`, with the contributing signals recorded as a human-readable reason
string. `--format md|txt` always overrides, with reason `"user-specified"`. Full scoring rule
in blueprint P0-S07.

### 0.5 Content Preservation Goals

The engine preserves **content**, not merely raw text: text and Unicode; structural elements
(headings, lists, quotes, appendices); technical content (math, code, chemical/scientific
notation); visual content (images, diagrams, captions, figure/image relationships); spreadsheet
data (organization, values, formulas, data types, merged cells, charts, comments, hyperlinks,
number formats); and cross-references (footnotes, endnotes, hyperlinks). A spreadsheet formula
`=SUM(B2:B10)` must never be silently replaced by its cached value — that destroys information
about how the value was calculated (AD-6).

### 0.6 Improvements Over Existing Tools

Relative to MarkItDown and similar multi-format converters: dual output with automatic
`.md`/`.txt` selection; a plain-text mode that doesn't pollute simple documents with Markdown
syntax; stronger structure preservation (tables, images, code, math, footnotes, reading order);
better reading-order reconstruction for multi-column PDFs; page/slide/chapter/sheet boundaries
preserved as structural units; first-class spreadsheet support with formula preservation;
native Rust performance, no Python runtime; offline-first, no mandatory cloud services; every
conversion warning surfaced, never silently dropped; extensible — a new format is a new
extractor module plus one `Format` enum variant, nothing else moves.

### 0.7 Explicit Non-Goals

This project is **not**: a document editor or creator (no DOCX/PPTX/PDF/XLSX writing); a
pixel-perfect reproduction system; a spreadsheet recalculation engine (formulas preserved as
text, never evaluated); a document summarizer, translator, or Q&A system; a domain-specific
analyzer; a replacement for word processors, spreadsheet applications, or document viewers; an
image classifier or caption generator (AD-9 — honest stub only: dimensions + preserved asset +
optional OCR text); an audio/video transcription tool or a URL/YouTube fetcher — comparable
tools support these via optional cloud or ML dependencies (speech-recognition services, cloud
video analyzers); docconvert deliberately doesn't, to keep the offline-first goal (§0.1)
absolute rather than conditional on which optional package is installed (§0.8, AD-13).

### 0.8 Positioning vs. MarkItDown (researched against the live project, 2026-09-14)

§0.1 names Microsoft's MarkItDown as this project's inspiration, so that comparison was
checked against the actual current upstream project (182k GitHub stars, 13.3k forks) rather
than assumed. What that checking confirmed, and what it deliberately does not carry over:

**Confirmed and adopted in spirit:**
- **Dependency discipline is the house style there too, not just here.** MarkItDown's own
  contributor guidelines state new formats are added "sparingly — especially if they incur new
  dependencies," and direct most new-format requests to third-party plugins instead of the
  core package. §4.2's dependency register follows the same discipline independently arrived
  at — this is external confirmation from the team maintaining the most widely used tool in
  this space, not a new decision made because of it.
- **AD-1's default-build PDF choice is externally validated.** MarkItDown's own core PDF path
  is built on `pdfminer.six` — a flat, non-layout-aware text extractor, the same category of
  tool as this project's default-build `pdf-extract` choice (whose own README lists pdfminer as
  a direct comparable). MarkItDown does not attempt PDFium-grade page-layout reconstruction in
  its default path either. AD-1's two-tier split isn't a compromise invented to resolve this
  project's own internal disagreement — it's the same trade-off the reference project made,
  made explicit and given an opt-in upgrade path instead of being left as a permanent
  limitation.
- **Multi-signal, never-trust-the-extension-alone detection** and a **narrowest-API/hostile-
  input security posture** (MarkItDown's README has its own "Security Considerations" section
  making the same argument as this document's §9) are both already this project's design;
  confirmed as shared industry practice for this problem domain, not something to change.

**Deliberately not carried over, with reasons:**
- **No runtime plugin system.** MarkItDown's plugin mechanism is cheap in Python (dynamic
  `import` via entry points). The equivalent in a single static Rust binary is dynamic library
  loading — a substantial, security-sensitive undertaking with no simple crate-level
  equivalent, and not justified at this project's maintainer scale. A new format here is a new
  extractor module compiled into the binary (§0.6), not a runtime-loaded plugin.
- **No cloud or LLM dependency, anywhere, ever.** Recent MarkItDown capabilities — Azure
  Document Intelligence, Azure Content Understanding, and the `markitdown-ocr` plugin (which
  uses an LLM vision client instead of a local OCR engine) — all require an external API and,
  for most of them, a billable cloud call. §0.1's offline-first goal is written as absolute,
  not conditional on which optional feature is enabled, so none of these patterns are
  candidates for adoption here. AD-10's local-process-only OCR adapter stands as originally
  decided, not weakened by seeing a cloud-based alternative exists elsewhere.
- **Different fidelity bar, on purpose.** MarkItDown's own README states it targets "text
  analysis tools" and "may not be the best option for high-fidelity document conversions for
  human consumption" — it optimizes for LLM-friendly Markdown, where a human-only concern like
  exact multi-column reading order matters less. This project's mission (§0.5) makes no such
  carve-out. That difference in ambition is *why* AD-1's `pdf-layout` feature exists at all —
  it's solving a fidelity bar the reference project deliberately doesn't attempt, not
  duplicating work it already does well.

---

## 1. Target Platform(s)

**Primary: local command-line (CLI) tool** on Linux, macOS, and Windows (x86_64; aarch64 where
practical).

- Headless: no GUI, no background daemon, no server mode.
- Offline-first: the core (default build) functions with zero network access.
- UI/UX / design system: **Not Applicable** — no visual interface exists; there is nothing for
  a Designer to produce. The CLI's textual output (progress lines, `--preview`, reports) is
  specified in the blueprint, not a design system.
- Mobile / embedded: **Not Applicable** — out of scope; the engine is structured as a library
  (`libdocconvert` + `docconvert` binary) so a future port or embedding isn't precluded.

---

## 2. Project Goals

1. Convert documents, presentations, spreadsheets, web documents, and images into clean
   Markdown or plain text, preserving structure and relationships — never just raw text or
   pixels (§0.1, §0.5).
2. One Common Document Model (CDM): every extractor produces it, every renderer consumes it
   (§7).
3. **Lightweight by default, capable when asked**: the default build is a single static
   binary with no native/FFI dependencies, target ≤5MB stripped+LTO. A second, explicitly
   opt-in build (`--features pdf-layout`) trades that size guarantee for full PDF page-layout
   fidelity. See §11 AD-1 for why this is two build profiles instead of one.
4. **Offline-first**: all conversion local; OCR only via a locally installed external engine
   the user enables — never bundled, never required.
5. **Honest failures**: every unconvertible feature becomes a reported warning or a clear
   fatal error — never silent loss.
6. **Honest scope**: features requiring ML/rendering engines (image classification/captioning,
   chart rasterization, legacy-binary DOC/XLS/PPT conversion) are explicit non-goals, reported
   as such rather than attempted badly.
7. **Dependency discipline as a standing practice, not a headcount**: every dependency is
   justified in §4.2's register against the current state of its alternatives — checked, not
   assumed — whenever it's added, not just at project kickoff. See §11 AD-1 for why an earlier
   draft's "≤8 dependencies" rule is replaced by this.

---

## 3. Scope Note: Why Two Prior Designs Existed

One earlier pass built a 24-crate workspace around `pdfium-render` (a wrapper around Chromium's
PDFium) and `calamine`, fully planned through a release phase, but never stated a binary-size
or dependency-count constraint. A second pass restarted at a single binary with an explicit
≤5MB/≤8-dependency contract, rejected `calamine` and `pdfium-render` by name, and only reached
Phase 0. Both instincts were partly right and partly wrong — see §11 for the resolution. This
document is the result of resolving them, not a summary of the disagreement.

---

## 4. Technology Stack

### 4.1 Language: Rust (over C++) — settled, not reopened

Both prior drafts independently chose Rust and neither is contested here. Summary: this
application's dominant risk is parsing adversarial or malformed binary/XML/ZIP input from
arbitrary documents, where memory-safety bugs are the historical failure mode in C++
implementations (Poppler, LibreOffice CVEs). Rust's compiler-enforced memory safety, single
static-binary distribution via `cargo build --release`, and mature pure-Rust ecosystem for
this exact domain (`zip`, `quick-xml`, `flate2`) make it the right call. C++'s only real
advantage — mature heavyweight engines (MuPDF, LibreOffice, Tesseract) — either violates the
lightweight-by-default goal (MuPDF is AGPL besides) or is available to Rust anyway via FFI
where genuinely needed (§11 AD-1).

### 4.2 Dependency Register (verified against crates.io / GitHub, 2026-09-14)

Every dependency below was checked for current maintenance status, transitive dependency cost,
and license on the date of this document — not carried forward from either prior draft.
Version numbers are the latest stable as of the check date; pin via `cargo add <crate>` at
implementation time and record the resolved versions in `Cargo.lock` (committed) and
`agent.md` — do not hand-type version numbers from this table into `Cargo.toml`, they will be
stale by the time Phase N is built.

**Default build — no native/FFI dependencies, all MIT-licensed:**

| # | Crate | Purpose | Checked-in status (2026-09-14) | Introduced |
|---|---|---|---|---|
| 1 | `zip` (≥8, default-features off + `deflate`) | ZIP containers for DOCX/PPTX/XLSX/ODT/ODS/EPUB | 265M downloads, updated 2026-08-11 | Phase 0 |
| 2 | `quick-xml` | Streaming XML for OOXML/ODF/EPUB/XHTML/HTML | 412M downloads, updated 2026-08-22 | Phase 0 |
| 3 | `flate2` | PDF `/FlateDecode` streams | mature, std-adjacent | Phase 0 |
| 4 | `clap` (derive) | CLI surface | industry standard | Phase 0 |
| 5 | `serde` + `serde_json` | Report JSON, config parsing hygiene | industry standard | Phase 0 |
| 6 | `walkdir` | Recursive directory batch scan | 619M downloads; stable/complete, not abandoned | Phase 0 |
| 7 | `base64` | Optional `--embed-assets` data-URI encoding | 1.5B downloads | Phase 0 |
| 8 | `csv` | RFC4180 CSV/TSV parsing (quoting, embedded newlines, escaping) | 244M downloads, BurntSushi; see AD-2 | Phase 0 |
| 9 | `encoding_rs` | Legacy encodings (RTF `\ansicpg`, legacy XLS codepages) | 524M downloads, Firefox's decoder | Phase 1/2 |
| 10 | `logos` | RTF control-word tokenizer (hand-written parser on top) | 76.8M downloads, updated 2026-01-30 | Phase 1 |
| 11 | `calamine` | XLSX/XLS/ODS reading (values, formulas, dates, merges, comments) | 12.7M downloads, updated 2026-07-27; see AD-3 | Phase 2 |
| 12 | `lopdf` | PDF object model / xref / stream decompression (via `pdf-extract`) | 19.2M downloads, updated 2026-09-08 | Phase 4 |
| 13 | `pdf-extract` | Correct PDF text decoding (CMap/CFF/Type1 font-encoding tables) | 5.0M downloads, updated 2026-06-25; see AD-1 | Phase 4 |

Counted as 13 across the *whole* finished product, introduced incrementally as each phase
needs them — not 13 on day one. Phase 0 needs exactly 8. See AD-1 for why this replaces a flat
headcount ceiling.

**Feature-gated, opt-in only (`--features pdf-layout`), excluded from the default/lightweight
build entirely:**

| # | Crate | Purpose | Why it's gated, not default |
|---|---|---|---|
| 14 | `pdfium-render` | Full PDF page-layout extraction: per-glyph bounding boxes, font metrics, page object geometry — needed for multi-column reading order, font-size heading detection, and ruled-table geometry detection | Dynamically loads a separate native PDFium shared library via `libloading` (confirmed in its own dependency graph) — this is not a "big Rust crate," it's an FFI boundary to a ~15–40MB native binary that must be bundled or system-installed. Breaks the single-static-binary goal outright. See AD-1. |

**Explicitly rejected** (checked and declined, with reasons, so this isn't re-litigated later):

| Crate | Why rejected |
|---|---|
| `docx-rs` | Write-oriented ("a .docx file **writer**" per its own description) — no read-path control. |
| `docx-rust` | Read-capable, but pulls `hard-xml` (a second XML parser alongside `quick-xml`) and `async_zip`/`futures-io` (async machinery this synchronous CLI has no use for); pre-1.0 (0.1.11). Violates the one-XML-library, no-async principles for uncertain correctness gain over hand-rolled OOXML parsing, which both prior drafts already independently chose. |
| `rtf-parser` | Actively maintained (updated 2026-06-11) and was worth checking, but its only non-serde dependency is `tsify` + `wasm-bindgen` — it's built WASM-first; pulling `wasm-bindgen` into a native CLI binary is dead weight. Hand-written tokenizer via `logos` stays the plan. |
| `epub` crate | Depends on `regex` and `xml-rs` — a second, heavier XML stack duplicating `quick-xml`. EPUB is ZIP+OPF+XHTML; the Phase 0 HTML extractor and `zip`/`quick-xml` already cover it with zero new dependencies (AD-4). |
| `html5ever` + `scraper` | Browser-grade HTML5 parsing is more than this project needs — document-embedded HTML is typically far cleaner than arbitrary web pages, and a tolerant hand-rolled parser on `quick-xml` (blueprint P0-S06) already has a fully specified malformed-tag recovery strategy. Rejecting this was already correct in one prior draft; confirmed, not re-decided. |
| `leptess` (OCR bindings) | Unmaintained since 2023-02-21. Confirms AD-1's broader pattern: OCR stays an external-process adapter (`--ocr-command`, default `tesseract`), never a bundled/linked library. |
| `infer` (magic-byte sniffing) | Only handles the outer container's magic bytes; ZIP-package disambiguation (DOCX vs PPTX vs XLSX vs EPUB vs ODT vs ODS) still needs custom logic that opens the archive and inspects internal paths regardless. The full magic-byte table is short, stable, and already fully specified in blueprint P0-S02 without it. |

---

## 5. Architectural Overview

Single binary with two build profiles, compiled from one codebase with two Cargo targets
(`docconvert` CLI + `libdocconvert` for tests/future embedding):

```
Input File
  |  (1) Detection            detect.rs      — magic bytes > ZIP-package sniff > OLE2 > extension
  |  (2) Extraction            extract/*.rs   — one module per format family, dispatched by Format
  |  (3) Common Document       model.rs       — Block / Inline / Table / Workbook (§7)
  |      Model (+ AssetManager)
  |  (4) Asset processing      assets.rs      — dedupe by hash, stable names, manifest
  |  (5) Output selection      render/mod.rs  — auto md-vs-txt scoring (§0.4)
  |  (6) Rendering              render/{markdown,txt}.rs
  |  (7) Output + report       convert.rs     — files, assets/, report.{json,txt}
```

Cross-cutting: `config.rs` (defaults -> profile -> config file -> CLI precedence), `ocr.rs`
(optional external-process engine adapter, never linked in), `report.rs`, `error.rs`.

Rules (unchanged from both prior drafts, kept because neither disputed them):
- Extractors never render; renderers never read source files.
- Extractors receive an `ExtractCtx` (options, asset manager, warning sink) and return a
  `Document`. They must not panic on malformed input — typed errors or degrade-with-warning
  only.
- A new format is one new extractor module plus one `Format` enum variant. Nothing else moves.
- Structure reconstruction (reading order, heading levels, table detection) happens **inside**
  the relevant extractor, not in a separate pipeline stage — each format's structure signal is
  too different to generalize (PDF: font geometry; DOCX: paragraph styles; HTML: tag
  semantics).

---

## 6. Component Responsibilities

| Component | File(s) | Responsibility |
|---|---|---|
| Detection | `src/detect.rs` | Multi-signal format ID: magic bytes, ZIP-package sniff, OLE2/encryption flags, extension-mismatch notes |
| CDM | `src/model.rs` | `Meta`, `Block`, `Inline`, `TextStyle`, `Table`, `Workbook`/`Sheet`/`Cell`, `Boundary`, `Placeholder`, `Document` — full types in blueprint P0-S01 |
| Assets | `src/assets.rs` | Content-hash dedupe, stable collision-free names, MIME-by-magic, write-out, manifest |
| Extractors | `src/extract/*.rs` | `text.rs` (TXT/MD), `csv.rs` (CSV/TSV via `csv` crate), `html.rs` (HTML/XHTML), `docx.rs`, `xlsx.rs`, `ods.rs`, `pptx.rs`, `odt.rs`, `epub.rs`, `rtf.rs`, `pdf.rs`, `image.rs`, `xml.rs` |
| Renderers | `src/render/` | `markdown.rs`, `txt.rs`, `mod.rs` (auto-selection) |
| OCR adapter | `src/ocr.rs` | off/auto/force; spawns local engine (default `tesseract`) via `std::process::Command` argv array, never a shell; zero impact when absent |
| Config | `src/config.rs` | Precedence chain, built-in profiles |
| Report | `src/report.rs` | JSON + human-text manifests |
| Orchestration | `src/convert.rs` | Single-file pipeline, batch, thread-pool parallelism (Phase 6), failure isolation |
| CLI | `src/main.rs` | `clap` surface incl. `--preview`, quiet/verbose/strict |
| Errors | `src/error.rs` | `ConvertError`: Io/Zip/Xml/Unsupported/Encrypted/Corrupt/Config/Ocr/Output |

---

## 7. Common Document Model (CDM)

One recursive tagged tree, produced by every extractor and consumed by every renderer, so
nothing downstream of extraction knows or cares what the original format was:

- `Document` — metadata, `Vec<Block>`, footnotes, assets, an optional `Workbook`, warnings.
- `Block` — the structural unit: `Heading`, `Paragraph`, `List`, `Table`, `CodeBlock`,
  `MathBlock`, `Quote`, `Rule`, `Image`, `Placeholder` (chart/drawing/embedded-object/scanned-
  page — honest stub per AD-9), `Boundary` (page/slide/chapter/worksheet), `FormulaList`.
- `Inline` — text-level content within a block: `Text`, `Styled`, `Code`, `Link`, `MathInline`,
  `FootnoteRef`, `Image`, `Break`.
- `Workbook`/`Sheet`/`Cell` — the spreadsheet-specific structured path; cells carry both
  `formula` and `display` value so a formula is never silently replaced by its cached result
  (AD-6).
- `Asset` — dedupe-aware binary content (images, embedded objects) referenced by index from
  `Block`/`Inline` variants, never inlined directly in the tree.

Full field-level Rust type definitions (exact enum variants, struct fields, derives) are
implementation detail and live in blueprint P0-S01, not duplicated here — architecture.md
states the model's shape and invariants; blueprint.md states its exact code.

---

## 8. Data Flow

**Single file.** `path -> detect_file() -> Detection{format,...}` -> dispatch extractor ->
`Document{meta, blocks, footnotes, assets, workbook, warnings}` -> `choose_output()`
(auto/heuristic -> `md|txt`, reason recorded) -> render -> write output + assets + report.

**Batch (Phase 6).** Inputs expanded (files; directories with `-r` recursive) -> per-file size
guard -> N worker threads (default = available parallelism, capped at 8) -> results collected;
one failure never stops the batch unless `--strict`.

**Boundaries.** Page/slide/chapter/worksheet boundaries travel inside the CDM as
`Block::Boundary`; renderers decide representation (MD: comment marker / heading; TXT: labeled
separators).

**Spreadsheets** take the structured path: extractors fill `Document.workbook`; the
sheet-to-blocks lowering (Markdown/fixed-width table for small sheets, CSV dump + summary for
large/irregular ones; formula listings when `--preserve-formulas`) happens once in a shared
`sheet_blocks()` helper (blueprint P0-S05) reused by CSV, XLSX, and ODS extractors, so
renderers stay format-agnostic.

---

## 9. Security Architecture & Threat Model

Trust boundary: **every input file is hostile.**

| Threat | Mitigation |
|---|---|
| Malformed/weaponized ZIP (zip-slip, zip bombs) | Asset output names are always ours, never from the archive; per-file and total decompressed-size guards; incomplete archives -> `Corrupt` error, never partial silent output |
| Malformed XML (billion-laughs, huge docs) | `quick-xml` streaming pull parser only — no DOM, no entity expansion; bounded buffers; size guard first |
| PDF parser bombs (nested objects, huge streams) | Bounded scan via `lopdf`'s object model; stream size caps; `/Encrypt` -> hard `Encrypted` error with a clear message |
| Path traversal from in-document references | Asset output names are generated (`image-NNN.ext`), never derived from document content; loader paths normalized and must stay in-package/in-dir |
| Command injection via OCR | Engine argv built as an array via `std::process::Command` (no shell); temp files in the OS temp dir with unique names; engine path is user-configured, default `tesseract` |
| Unbounded memory | `--max-size` guard (default 200MB); streaming where practical; assets flushed after write |
| Output overwrite / data loss | Overwrite disabled by default; explicit `--overwrite` required |
| Denial of service via batch | Per-file failure isolation; `--strict` opt-in stops the batch on first failure |
| `pdfium-render` FFI surface (opt-in build only) | Confined to the `pdf-layout` feature; the default build has zero FFI surface. When enabled, the loaded native library path is either the system-installed PDFium or an explicitly user-provided path — never fetched at runtime. |

Not applicable (with reason): authentication/authorization (local single-user tool, no
accounts); secrets/tenant isolation (no services, no shared state); replay/network injection
(no network I/O in the default build; the opt-in build adds none either — PDFium is loaded
locally, not fetched). Supply-chain mitigation is the committed, pinned `Cargo.lock`.

---

## 10. Deployment Architecture

- **Default build**: single static release binary per platform (`cargo build --release`, LTO,
  strip, `codegen-units=1`, `panic=abort`), target ≤5MB. No installer, no services, no PATH
  requirements beyond the binary itself.
- **`pdf-layout` build**: a second release artifact per platform, built with `--features
  pdf-layout`, bundling or documenting the required native PDFium library per platform. Not
  size-constrained; explicitly the "full fidelity" variant, clearly labeled as larger.
- Optional runtime discovery (both builds): an OCR engine executable (`--ocr-command`, default
  `tesseract`) — feature-gated at runtime, zero impact when absent.
- Reproducibility: pinned lockfile, documented toolchain version, resolved dependency versions
  and as-built binary size recorded in `agent.md` at each release.
- Published for Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64) once Phase
  6 packaging work lands.

---

## 11. Architectural Decisions (ADRs)

- **AD-1 — Two build profiles instead of one universal binary.** The project's own stated
  goals both require full PDF layout fidelity (multi-column reading order, font-size heading
  detection, ruled-table geometry — §0.6) *and* a lightweight, dependency-minimal default. No
  currently-available pure-Rust crate provides page-layout-aware glyph positioning: `lopdf`
  exposes only the raw PDF object model (no computed text layout), and `pdf-extract` returns
  flat decoded text (no per-character bounding boxes). Only `pdfium-render` provides that, and
  it does so by dynamically loading a large native PDFium library via `libloading` — not a
  "heavy Rust crate" but a different category of dependency (an FFI boundary to a real
  rendering engine). Rather than silently downgrading the project's own stated PDF-quality bar
  to fit a size budget, or silently abandoning the size budget to hit the PDF-quality bar, the
  two are separated into explicit, labeled build profiles. Default build: `pdf-extract` +
  `lopdf` give correct text decoding (the hardest, most silently-buggy part of hand-rolling
  PDF text — font CMap/CFF/Type1 encoding tables) with approximate (stream-order) reading
  order and a clear warning when a document's layout is complex enough that the approximation
  matters. `pdf-layout` build: `pdfium-render` unlocks the full reading-order/table/heading
  algorithm design from the earlier multi-crate draft (blueprint Phase 4, migrated in).
- **AD-2 — Use the `csv` crate, not a hand-rolled RFC4180 parser.** An earlier draft hand-rolled
  CSV parsing to keep the dependency count down. CSV quoting/escaping/embedded-newline handling
  is a well-known "looks simple, isn't" correctness trap, and `csv` (BurntSushi, 244M
  downloads) costs nothing extra in the dependency graph that isn't already justified elsewhere
  — it's a correctness upgrade with no real lightweight cost.
- **AD-3 — Include `calamine`, reverse an earlier rejection.** An earlier draft rejected
  `calamine` as "heavy, pulls extra deps." Checked against its actual current dependency graph:
  `byteorder`, `codepage`, `encoding_rs`, `quick-xml`, `serde`, `zip`, plus two small
  single-purpose numeric-parsing helpers (`atoi_simd`, `fast-float2`). Every non-trivial entry
  in that list is already a required dependency of this project for other reasons. The
  marginal cost of `calamine` over what's already needed is two small crates, in exchange for
  not hand-rolling XLSX/XLS/ODS binary-format parsing (shared strings, number formats, merged
  cells, dates-as-serial-numbers) across three formats. The original rejection wasn't
  supported by the numbers once checked.
- **AD-4 — Hand-roll DOCX/PPTX/ODT/EPUB/generic-XML on `zip` + `quick-xml`.** Both prior
  drafts already agreed here independently, and dependency research (docx-rs is write-only;
  docx-rust pulls a second XML stack and async machinery for a pre-1.0 crate; the `epub` crate
  duplicates XML tooling this project already has) confirms it. OOXML/ODF/EPUB are documented,
  stable, ZIP-based XML schemas — hand-parsing them is standard practice for conversion tools
  in this space, not a shortcut.
- **AD-5 — Legacy binary DOC/PPT (OLE2): detect-and-report, not convert.** Converting them
  requires OLE2 compound-document parsing (Word/PowerPoint binary formats) far beyond either
  build profile's scope. Detected cleanly via `detect.rs`, reported with a clear explanation,
  never attempted badly. **XLS is not in this category** — see AD-14.
- **AD-6 — Spreadsheet formulas are first-class data**, never silently replaced by cached
  values (`Cell.formula` alongside `Cell.display`, plus `Block::FormulaList`).
- **AD-7 — Warnings are data, not print statements.** Extractors push into `ctx.warnings`;
  reports and `--preview` surface them; nothing prints and forgets.
- **AD-8 — Markdown-in, Markdown-out passes through unparsed** for md-to-md conversion (the
  source already is the target format); md-to-txt still routes through the Markdown parser.
- **AD-9 — Image classification/captioning ships as an honest stub.** Dimensions + preserved
  asset + optional OCR text when enabled. ML-based classification and generated descriptions
  are documented non-goals, reported as such via `--describe-images`' warning rather than
  silently ignored.
- **AD-10 — OCR is always an external-process adapter, never a linked library.** Confirmed,
  not just carried forward: the one Rust Tesseract-binding crate available (`leptess`) has been
  unmaintained since February 2023. Spawning the user's own `tesseract` install via
  `std::process::Command` avoids both the staleness risk and a native-library linking
  requirement in every build profile, including `pdf-layout`.
- **AD-11 — AD-1's default PDF path is independently validated by MarkItDown itself.**
  MarkItDown's own core PDF converter is built on `pdfminer.six`, a flat/non-layout-aware text
  extractor — the same category as this project's `pdf-extract` default. The reference project
  this document is explicitly inspired by (§0.1) made the identical default-path trade-off.
  See §0.8.
- **AD-12 — No runtime plugin system.** Considered because MarkItDown has one. Rejected: its
  plugin mechanism relies on Python's cheap dynamic-import model; the Rust equivalent is
  dynamic library loading, a materially bigger and more security-sensitive undertaking not
  justified for a single-maintainer project (§0.8). New formats extend the compiled binary
  directly, consistent with the dependency-discipline posture in §4.2.
- **AD-13 — No cloud or LLM dependency, under any feature flag, ever.** Considered because
  MarkItDown now offers Azure Document Intelligence, Azure Content Understanding, and an
  LLM-vision OCR plugin. Rejected outright, not just deprioritized: §0.1's offline-first goal
  is written as absolute. This closes the door on ever "solving" a hard extraction problem
  (scanned-PDF OCR quality, chart/image description) by reaching for a cloud API instead of
  accepting the honest-stub / external-local-process approach already decided in AD-9 and
  AD-10. See §0.8.
- **AD-14 — XLS is a `calamine` format, not an AD-5 legacy format.** Caught while detailing
  Phase 2, not at first pass: legacy binary XLS (BIFF) is one of `calamine`'s directly
  supported formats (its dependency on `codepage` exists specifically for legacy-XLS string
  encoding) — it doesn't need OLE2 compound-document parsing the way DOC/PPT do. §0.2's format
  table and AD-5 are corrected accordingly. Two fidelity gaps remain and are handled by
  degrading with a warning rather than by adding a dependency to close them: legacy XLS
  formula *text* may be unavailable depending on the resolved `calamine` version's BIFF
  support (cached value is preserved either way, per AD-6's spirit — a missing formula string
  is a fidelity warning, not a dropped value); and legacy-XLS chart-image recovery is
  deliberately out of scope (would need the `cfb` crate for raw OLE2-stream inspection, for a
  narrow benefit) — chart presence is still reported via a `Placeholder::Chart` with no asset,
  never silently dropped.
- **AD-15 — Scanned-PDF OCR is only available in the `pdf-layout` build.** Surfaced while
  detailing Phase 5, not assumed up front: OCR needs a rasterized bitmap of the page, and
  only `pdfium-render` (Stage 4.B, feature-gated) can rasterize a PDF page — `lopdf` and
  `pdf-extract` (the default build's dependencies, AD-1) cannot. The default build's scanned
  pages therefore always stay `Placeholder{kind: ScannedPage}` regardless of `--ocr` mode,
  with an explicit warning naming the reason and the fix (rebuild with `--features
  pdf-layout`). This is a direct, unavoidable structural consequence of AD-1 and AD-10 taken
  together, not a gap either decision overlooked — documented here so it's an intentional
  boundary, not a support surprise. Standalone images have no such gap (the file is already a
  bitmap) and are OCR-eligible in both build profiles.

---

## 12. Required Skills -> Component Map

| Skill | Applies to |
|---|---|
| Rust core (ownership, enums, `Result`) | all modules |
| ZIP container anatomy | docx, xlsx, pptx, odt, ods, epub extractors |
| OOXML (WordprocessingML/PresentationML/SpreadsheetML) | docx, pptx, xlsx |
| ODF (OpenDocument XML) | odt, ods |
| EPUB/OPF packaging | epub |
| HTML/XHTML semantics | html, epub |
| RTF control-word grammar | rtf |
| PDF object/stream model + content-stream operators | pdf (default path: `lopdf`/`pdf-extract`; `pdf-layout` path: `pdfium-render` page/text APIs) |
| Spreadsheet data modeling (dates-as-serial-numbers, merged cells, formulas) | xlsx, ods, csv |
| CLI/config design | main.rs, config.rs |
| Property-based / golden testing | tests, all extractors |

---

## 13. Testing Strategy

- Unit tests per module (detection signals, renderers, CSV/RTF/HTML parsers).
- Integration tests with **programmatically generated fixtures** (ZIP/XML/PDF builders in
  `tests/common/`) — no binary assets committed to the repo.
- Golden-file tests for MD/TXT renderers.
- Negative tests: corrupt ZIP, truncated PDF, `/Encrypt` PDF, extension mismatch, unsupported
  OLE2, oversized file, unknown format — each must yield the exact spec'd error/warning, never
  empty silent output.
- Formula-preservation tests (AD-6, verbatim round-trip).
- Fuzz targets (RTF tokenizer, PDF object scanner, CSV parser) — ≥24 CPU-hours each without a
  new crash before v1.0 (§14).
- `pdf-layout` build gets its own fixture suite once Phase 4 reaches that feature (multi-column,
  ruled table, font-size heading detection) — these tests do not run against the default build,
  since the default build doesn't have that capability by design (AD-1).

---

## 14. Success Criteria (v1.0 exit bar)

1. Every phase in `blueprint.md`'s roadmap is complete with green CI, for both build profiles.
2. Every fixture in `tests/fixtures/` produces byte-identical output matching `tests/golden/`.
3. Fuzz targets have run ≥24 CPU-hours each without producing a new crash.
4. Every format in §0.2 works end-to-end for real-world documents from a curated corpus of
   ≥20 documents per format.
5. Default-build pre-built binaries published for Linux (x86_64, aarch64), macOS (x86_64,
   aarch64), and Windows (x86_64), each ≤5MB stripped; `pdf-layout` binaries published
   alongside, unconstrained on size but documented.
6. Documentation covers: user manual, CLI reference, per-format quirks list, the two-build-
   profile explanation, developer guide for adding a new extractor.
7. End-to-end conversion of a mixed batch of 100 real documents completes in ≤5 minutes on an
   8-core machine.
8. Spreadsheet formula preservation verified: no formula is silently replaced by its cached
   value in any test fixture.
9. Output behavior is versioned: breaking changes to output format require a major version
   bump.
