# docconvert — Implementation Blueprint

Version: 2.0 (merged) · Companion to `architecture.md` v2.0

Detailing policy (architect.md §5): **Phase 0 is fully detailed below. Phases 1–6 are
outlined (Step ID + Objective only).** Ask before expanding a later phase: "Ready for me to
detail Phase N?"

Execution rule: one step at a time, in order. After each step: `cargo build` clean, `cargo
test` green, then update `agent.md`. A step is not "done" until its Acceptance Criteria and
Definition of Done are met.

Note on citations: this blueprint is self-contained. Where an earlier draft referenced an
external "spec §N" for a rule, that rule is either fully spelled out inline here (the common
case) or the citation has been replaced with a pointer into `architecture.md`. No external
specification document is assumed to exist.

Note on dependency versions: exact version numbers below reflect what was current on
2026-09-14. Pin with `cargo add <crate>` at implementation time rather than hand-typing these —
resolve to whatever is then-current stable, and record the resolved versions in `Cargo.lock`
(committed) and `agent.md`.

---

# Roadmap Overview

| Phase | Goal | Scope | Exit criteria | Skills |
|---|---|---|---|---|
| **0** | Foundation + first vertical slice | Scaffold, errors, CDM, detection, assets, TXT/MD/CSV/HTML extractors, MD/TXT renderers, auto-selection, config+profiles, reports, single-file CLI + preview | `docconvert` converts .txt/.md/.csv/.tsv/.html/.xhtml to .md or .txt with assets dir + report; all P0 tests green | Rust core, HTML, testing |
| **1** | Word-processing & books | DOCX, EPUB, ODT, RTF extractors (hand-rolled on `zip`+`quick-xml`, AD-4) | All four formats pass integration tests incl. lists, tables, links, footnotes, embedded images | ZIP, OOXML, ODF, EPUB, RTF |
| **2** | Spreadsheets | XLSX + XLS + ODS via `calamine` (AD-3/AD-14); sheet-to-blocks policy | Formula-preservation test passes verbatim (AD-6); merged cells, dates, hidden sheets correct | OOXML SpreadsheetML, ODF, spreadsheet data modeling |
| **3** | Presentations | PPTX extractor (hand-rolled, AD-4) | Multi-slide fixture reproduced: notes + images + tables kept | OOXML PresentationML |
| **4** | PDF — two build profiles | **4.A (default build):** `lopdf`+`pdf-extract` correct text decoding, approximate reading order. **4.B (`pdf-layout` feature):** `pdfium-render` full page-layout reconstruction — multi-column reading order, font-size heading detection, ruled/unruled tables, header/footer stripping (AD-1) | 4.A: encrypted/scanned-PDF fixtures handled correctly, text decoding golden-tested. 4.B: two-column academic PDF, ruled-table PDF reproduced correctly | PDF internals, PDF layout/typography |
| **5** | Images + OCR | Standalone image path, image-header parsing, external OCR adapter (off/auto/force, AD-10) | OCR-on fixture test with a fake engine script; OCR-absent warning otherwise | Image headers, process spawning |
| **6** | Batch, hardening, packaging | Recursive batch, parallel jobs, failure isolation, strict mode, profiles E2E, negative-test suite, samples, README, packaging for **both** build profiles | All CLI flags exercised; negative suite green; both release binary sizes recorded in `agent.md` | Testing, docs |

---

# Stage 1 — Full Outline (Phase → Stage → Step)

## Phase 0 — Foundation & First Vertical Slice
- Stage 0.A — Skeleton
  - **P0-S01** Project scaffold: crate layout, error types, Common Document Model, `Format` enum.
- Stage 0.B — Input intelligence
  - **P0-S02** Format detection: magic bytes, ZIP-package sniffing, OLE2/encryption flags, extension-mismatch notes.
  - **P0-S03** Asset manager: hash dedupe, stable names, MIME-by-magic, write-out, manifest.
- Stage 0.C — First extractors
  - **P0-S04** TXT + Markdown extractor (MD parser: headings, lists, fences, quotes, links, images, tables, footnotes).
  - **P0-S05** CSV/TSV extractor via the `csv` crate (AD-2) → Workbook model.
  - **P0-S06** HTML/XHTML extractor (semantic blocks, script/nav removal, figures, asset loading via injected loader).
- Stage 0.D — Output
  - **P0-S07** Markdown renderer + TXT renderer + automatic output selection.
- Stage 0.E — Pipeline & CLI
  - **P0-S08** Configuration system (defaults → profile → config file → CLI), built-in profiles, reports, single-file pipeline, CLI with preview mode; end-to-end tests.

## Phase 1 — Word-Processing & Books (outlined)
- Stage 1.A — Shared package tooling
  - **P1-S01** ZIP/XML shared helpers + OOXML relationships parser.
- Stage 1.B — DOCX
  - **P1-S02** DOCX extractor: paragraphs, heading styles, runs/formatting, lists (`numbering.xml`), tables + merges, hyperlinks, images, footnotes, metadata.
- Stage 1.C — EPUB
  - **P1-S03** EPUB extractor: container → OPF → spine reading order, chapter boundaries, metadata, images via the Phase-0 HTML extractor (loader injection reuse, AD-4).
- Stage 1.D — ODT
  - **P1-S04** ODT extractor: `content.xml` text/lists/tables/images/links/notes, automatic styles, `meta.xml`.
- Stage 1.E — RTF
  - **P1-S05** RTF extractor: `logos`-based control-word tokenizer, groups, `\u` escapes, `encoding_rs`-backed cp1252 decode, paragraphs, tables, `\pict` images, `\fld` hyperlinks, destination skipping.
- Stage 1.F — Verification
  - **P1-S06** Phase-1 integration + cross-format golden tests.

## Phase 2 — Spreadsheets (outlined)
- Stage 2.A — XLSX + XLS core via `calamine` (AD-3/AD-14)
  - **P2-S01** Workbook open, sheet enumeration, cell value/type/formula extraction into `Workbook`/`Sheet`/`Cell` — covers XLSX and legacy XLS (BIFF) through the same `calamine` reader.
- Stage 2.B — Fidelity
  - **P2-S02** Number-format-to-`CellKind::Date` mapping, merged-cell ranges, cell comments, hyperlinks, hidden-sheet handling (`--include-hidden-sheets`), chart inventory (name-only placeholders); legacy-XLS formula-text and chart-image gaps degrade to warnings (AD-14), never silent loss.
- Stage 2.C — Lowering policy
  - **P2-S03** Wire `xlsx.rs`/`ods.rs` onto the shared `sheet_blocks()` policy already built in P0-S05; `FormulaList` emission.
- Stage 2.D — ODS
  - **P2-S04** ODS extraction (`calamine` covers ODS natively — no second spreadsheet crate needed).
- Stage 2.E — Verification
  - **P2-S05** Spreadsheet test suite incl. verbatim formula-preservation example (AD-6), merged cells, dates, hidden sheets, legacy-XLS fixture, large-sheet CSV-dump path.

## Phase 3 — Presentations (outlined)
- Stage 3.A — Slide structure
  - **P3-S01** PPTX slide order, titles, text boxes, bullets/indent levels (hand-rolled `zip`+`quick-xml`, AD-4).
- Stage 3.B — Rich content
  - **P3-S02** Tables, images, chart placeholders, speaker notes (`--include-notes`), slide boundaries.
- Stage 3.C — Verification
  - **P3-S03** Presentation tests: multi-slide fixture with table + image + notes, golden output.

## Phase 4 — PDF (outlined)

### Stage 4.A — Default build: correct text extraction (`lopdf` + `pdf-extract`, AD-1)
  - **P4-S01** `pdf-extract`/`lopdf` integration: load, `/Encrypt` detection → `FatalError::EncryptedDocument`, baseline text decode.
  - **P4-S02** Paragraph/heading reconstruction from flat decoded text: blank-line paragraph splitting, best-effort heading heuristic (short isolated line, no trailing punctuation — documented as weaker than §4.B's font-metric approach), scanned-page detection (zero extractable characters → `NeedsOcr`).
  - **P4-S03** Default-path PDF tests: single-column prose fixture, scanned-page fixture, encrypted-PDF fixture; golden text output; explicit test asserting the "approximate reading order" warning fires on a synthetic multi-column fixture (proving the limitation is surfaced, not hidden).

### Stage 4.B — `pdf-layout` feature: full structural fidelity (`pdfium-render`, opt-in)
  - **P4-S04** `pdfium-render` integration behind `#[cfg(feature = "pdf-layout")]`; per-character bounding-box + font metadata extraction via `page.text()`.
  - **P4-S05** Line and column clustering: median line-height/char-width, gap-based multi-column split, left-column-then-right-column reading order.
  - **P4-S06** Heading detection via modal body font-size + line-isolation heuristic, level mapping (largest distinct size → level 1, capped at 6).
  - **P4-S07** Header/footer stripping via cross-page repetition of normalized (y-position, text) pairs (≥60% of pages), toggled by `--no-strip-headers-footers`.
  - **P4-S08** Table detection: ruled (axis-aligned line-segment grid via `PdfPageObjectType::Path`) and unruled (consistent x-band text alignment across ≥3 consecutive lines) variants.
  - **P4-S09** Image extraction from `PdfPage::objects()`, associated with surrounding text by y-position.
  - **P4-S10** List and code-block reconstruction: bullet/numbering-pattern detection at consistent x-offset; monospace-font-run detection (≥3 consecutive lines, consistent indentation) → `Block::CodeBlock` + info-level warning.
  - **P4-S11** Math font/Unicode-range detection (Cambria Math, STIX, Symbol; U+1D400–U+1D7FF, U+2200–U+22FF) → `Inline::Math`/`Block::MathBlock` as literal Unicode + warning "formula rendered as literal Unicode".
  - **P4-S12** `pdf-layout` verification: two-column academic-layout fixture, ruled-table fixture, unruled (whitespace-aligned) table fixture, embedded-raster-image fixture, math-font-text fixture. Acceptance: correct multi-column reading order, correctly-leveled headings, headers/footers removed, ruled table recovered as `Block::Table`.

## Phase 5 — Images & OCR (outlined)
- Stage 5.A — Standalone images
  - **P5-S01** Hand-rolled image header parser (PNG/JPEG/GIF/BMP/WebP/TIFF/JP2 dimensions only — no image-decoding crate needed for header-only reads) + standalone image-to-document conversion path.
- Stage 5.B — OCR adapter (AD-10)
  - **P5-S02** External OCR adapter: off/auto/force modes, `--ocr-command` (default `tesseract`) invoked via `std::process::Command` array argv (no shell), temp-file protocol, `--ocr-lang`, confidence passthrough where the engine reports it.
- Stage 5.C — Wiring & verification
  - **P5-S03** OCR wiring into the standalone-image path and the scanned-PDF path (`NeedsOcr` pages from Phase 4); tests via a fake engine script (no real Tesseract dependency in CI); OCR-absent warning path.

## Phase 6 — Batch, Hardening, Packaging (outlined)
- Stage 6.A — Batch
  - **P6-S01** Multiple files, directories, recursive (`-r`) scan via `walkdir`, filtering, overwrite control.
- Stage 6.B — Parallelism
  - **P6-S02** Worker-thread pool (`std::thread` + channels, default = available parallelism capped at 8), per-file failure isolation, `--strict`, progress/quiet/verbose output contract.
- Stage 6.C — Profiles & config end-to-end
  - **P6-S03** Profile-application tests and config-file precedence integration tests across all built-in profiles.
- Stage 6.D — Negative-test suite
  - **P6-S04** Corrupt ZIP, truncated/encrypted PDF, OLE2 legacy, oversized file, unknown format, extension mismatch — each asserted against its exact error/warning message.
- Stage 6.E — Release
  - **P6-S05** Samples generator, README, usage docs, error reference, packaging notes for **both** build profiles; record default-build and `pdf-layout`-build binary sizes + build times in `agent.md`.

---

# Stage 2 — Phase 0, Fully Detailed

---

## P0-S01 — Project scaffold, error types, Common Document Model

- **Step ID:** P0-S01
- **Objective:** Create the crate and the entire Common Document Model (CDM) plus the typed error system; everything else compiles against this.
- **Depends On:** — (first step)
- **Files to Create:** `Cargo.toml`, `src/lib.rs`, `src/error.rs`, `src/model.rs`, `.gitignore`
- **Files to Modify:** —
- **Inputs:** architecture.md §4.2, §5, §7, §9.
- **Outputs:** Compiling crate exposing `docconvert::model`, `docconvert::error`.
- **Exact Instructions:**
  1. `Cargo.toml`: package `docconvert` v0.1.0, edition 2021, `[lib] name = "docconvert"` + `[[bin]] name = "docconvert"`. Runtime deps (current stable at implementation time, per architecture.md §4.2 — do not hand-type versions from this doc): `zip` (default-features=false, features=["deflate"]), `quick-xml`, `clap` (features=["derive"]), `serde` (derive), `serde_json`, `flate2`, `walkdir`, `base64`, `csv`. `[profile.release]`: `lto=true`, `codegen-units=1`, `strip=true`, `panic="abort"`. Add a `[features] pdf-layout = []` stanza now (empty — Phase 4 adds the actual `pdfium-render` optional dependency behind it; declaring the feature flag early keeps `cfg(feature = "pdf-layout")` valid from day one even though nothing uses it yet). `src/main.rs` starts as `fn main() { println!("docconvert") }` placeholder (replaced in P0-S08).
  2. `error.rs`: `pub enum ConvertError { Io(std::io::Error), Zip(String), Xml(String), Unsupported(String), Encrypted(String), Corrupt(String), Config(String), Output(String), Ocr(String), Other(String) }`; implement `Display` (prefix each with its category, e.g. `"unsupported format: ..."`), `std::error::Error`, `From<std::io::Error>`, and `pub fn is_fatal(&self) -> bool` (all variants fatal at file level; warnings are separate data, never an error variant). Helper `pub type Result<T> = std::result::Result<T, ConvertError>;`.
  3. `model.rs` — implement exactly these public types (derive `Debug, Clone`, and `serde::Serialize` where noted; `Default` where sensible):
     - `enum Format { Pdf, Docx, Doc, Odt, Ods, Odp, Rtf, Epub, Html, Txt, Markdown, Pptx, Xlsx, Xls, Csv, Tsv, Image, Unknown }` with `Display` (lowercase names) and `Default = Unknown`.
     - `enum ImageKind { Png, Jpeg, Gif, Bmp, Tiff, Webp, Jp2 }` with `ext(&self) -> &'static str`, `mime(&self)`.
     - `struct Meta { title, author, subject, description, created, modified, language: Option<String>, source_filename: String, source_format: Format, page_count, slide_count, chapter_count, worksheet_count: Option<usize> }`.
     - `struct TextStyle { bold, italic, underline, strike, sup, sub: bool }` with `is_plain()`.
     - `enum Inline { Text(String), Styled{ style: TextStyle, children: Vec<Inline> }, Code(String), Link{ children: Vec<Inline>, url: String }, MathInline(String), FootnoteRef(String), Image{ asset: usize, alt: Option<String> }, Break }`.
     - `enum Align { Default, Left, Right, Center }`.
     - `struct TableCell { inlines: Vec<Inline>, colspan: usize, rowspan: usize, align: Align }` (default `colspan=1, rowspan=1`).
     - `struct Table { header: Vec<TableCell>, rows: Vec<Vec<TableCell>>, caption: Option<String>, approximate: bool }`.
     - `struct ListItem { level: usize, blocks: Vec<Block> }`.
     - `enum Boundary { Page(usize), Slide{ n: usize, title: Option<String> }, Chapter(Option<String>), Worksheet(String) }`.
     - `enum PlaceholderKind { Chart, Drawing, EmbeddedObject, ScannedPage }` with `label(&self) -> &'static str` ("Chart"/"Drawing"/"Embedded object"/"Scanned page").
     - `enum Block { Heading{ level: usize, inlines: Vec<Inline> }, Paragraph(Vec<Inline>), List{ ordered: bool, start: Option<u64>, items: Vec<ListItem> }, Table(Table), CodeBlock{ language: Option<String>, text: String }, MathBlock(String), Quote(Vec<Block>), Rule, Image{ asset: usize, alt: Option<String>, caption: Option<String> }, Placeholder{ kind: PlaceholderKind, label: Option<String>, asset: Option<usize> }, Boundary(Boundary), FormulaList(Vec<(String,String)>) }`.
     - `enum CellKind { Empty, Number, Text, Bool, Date, Error }`.
     - `struct Cell { reference: String, formula: Option<String>, display: String, kind: CellKind, number_format: Option<String>, hyperlink: Option<String> }` — `formula` and `display` are always both populated when a formula exists (AD-6): never collapse to just the cached value.
     - `struct Sheet { name: String, hidden: bool, cells: BTreeMap<(u32,u32), Cell>, max_row: u32, max_col: u32, merged: Vec<String>, comments: Vec<(String,String)>, charts: Vec<Option<String>> }`.
     - `struct Workbook { title: Option<String>, sheets: Vec<Sheet> }`.
     - `struct Asset { filename: String, rel_path: String, mime: String, bytes: Vec<u8>, alt: Option<String>, origin: String, duplicate_of: Option<String>, hash: u64 }`.
     - `struct Document { meta: Meta, blocks: Vec<Block>, footnotes: Vec<(String, Vec<Inline>)>, assets: Vec<Asset>, workbook: Option<Workbook>, warnings: Vec<String> }` with `Document::new(source_filename, source_format)` and `fn text_len(&self) -> usize` (recursive inline text length — used by auto-selection and preview).
     - Model helpers: `pub fn inline_text(inlines: &[Inline]) -> String` (flattens to plain text; `Break` → `"\n"`) and `pub fn walk_inlines_mut(blocks: &mut [Block], f: &mut dyn FnMut(&mut Inline))` (recursive traversal over every `Block` variant containing inlines, including `Table` cells and `List` items; footnotes handled separately by the caller).
  4. `lib.rs`: `pub mod error; pub mod model;` (further modules added in later steps).
  5. `.gitignore`: `/target`. `Cargo.lock` is **committed** (do not ignore it — supply-chain pinning, architecture.md §9/§10).
- **Relevant Skills:** Rust core.
- **Approved Dependencies:** all 9 default-build crates from architecture.md §4.2 declared now (lockfile pinned early); only `serde` used by this step's code.
- **Rationale:** CDM first forces every later extractor to target one contract; error enum structure matches architecture.md §9's threat/error taxonomy.
- **Security Considerations:** None directly; `ConvertError` must never expose filesystem paths beyond the input's own filename in messages.
- **Testing Requirements:** Unit tests: `Meta::default` fields; `inline_text` over nested `Styled`/`Link`/`Break`; `walk_inlines_mut` visits table cells and list items; `Format` `Display` strings.
- **Acceptance Criteria:** `cargo build` succeeds with zero warnings; `cargo test` green.
- **Definition of Done:** Tests pass; `agent.md` updated with step status + dependency lockfile hash note.

---

## P0-S02 — Format detection

- **Step ID:** P0-S02
- **Objective:** Identify any input by magic bytes + package structure + text heuristics; never trust the extension alone; report mismatches, encryption, legacy OLE2.
- **Depends On:** P0-S01
- **Files to Create:** `src/detect.rs`, `tests/detect.rs`
- **Files to Modify:** `src/lib.rs` (+`pub mod detect;`)
- **Inputs:** architecture.md §5, §9.
- **Outputs:** `pub struct Detection { format: Format, mime: Option<String>, extension: String, extension_match: bool, image_kind: Option<ImageKind>, notes: Vec<String>, encrypted: bool }`; `pub fn detect_file(path: &Path) -> Result<Detection>`.
- **Exact Instructions:**
  1. Read up to 64 KiB header via a bounded `File` read (never slurp the whole file). Unreadable → `ConvertError::Io`; empty → `Corrupt("file is empty")`.
  2. Match in this order:
     - `%PDF` prefix → `Pdf` (mime `application/pdf`). Scan the header window for `/Encrypt` → `encrypted = true`.
     - `PK\x03\x04` / `PK\x05\x06` → ZIP: open with the `zip` crate (error → `Corrupt("incomplete or damaged ZIP archive")`). Collect entry names (case-insensitive matching throughout). If a `mimetype` entry exists, read it: `application/vnd.oasis.opendocument.text` → Odt, `.spreadsheet` → Ods, `.presentation` → Odp, `application/epub+zip` → Epub. Else if `[Content_Types].xml` is present: probe entry names — `word/document.xml` → Docx, `ppt/presentation.xml` → Pptx, `xl/workbook.xml` → Xlsx, else Unknown + note "OOXML package with unrecognized parts". Else → Unknown + note "ZIP archive without recognizable document structure".
     - `D0 CF 11 E0 A1 B1 1A E1` → OLE2 compound: search the header window for the UTF-16LE byte pattern of `EncryptedPackage` → `encrypted=true` + format Unknown + note. Otherwise map by extension: `.doc`→Doc, `.xls`→Xls, `.ppt`→Unknown-with-note "legacy binary Office format — detected, conversion unsupported" (AD-5).
     - `{\rtf` (allow BOM/leading whitespace) → Rtf.
     - Image magics: `\x89PNG` → Png; `\xFF\xD8\xFF` → Jpeg; `GIF87a`/`GIF89a` → Gif; `BM` → Bmp; `II*\x00`/`MM\x00*` → Tiff; `RIFF….WEBP` → Webp; JP2 signature box `\x00\x00\x00\x0CjP` → Jp2. Format `Image`, `image_kind` set, mime per kind.
     - Textual: decode header as UTF-8 lossy; lowercase; `<!doctype html` or `<html` within the first 2 KiB → Html (note if also `<?xml` → "XHTML"). `<?xml` alone → Unknown + note "XML document without recognized schema". Otherwise, if UTF-8-ish (≤1% replacement chars): extension `.md`/`.markdown` OR heuristic (≥2 lines starting `# `, a fenced code block, or `[x](y)` link syntax) → Markdown, else Txt.
     - Otherwise → Unknown + note "no recognizable signature".
  3. Extension handling: lowercase the extension; an `expected_formats(ext)` table (`docx`→Docx, `doc`→Doc, `odt`→Odt, `ods`→Ods, `rtf`→Rtf, `epub`→Epub, `html`/`htm`/`xhtml`/`xht`→Html, `txt`→Txt, `md`/`markdown`→Markdown, `pptx`→Pptx, `xlsx`→Xlsx, `xls`→Xls, `csv`→Csv, `tsv`→Tsv, image extensions→Image). `extension_match` = detected format is in the expected set (Txt/Markdown interchangeable). On mismatch, push note `extension ".x" does not match detected content (<Format>)`.
  4. CSV/TSV have no magic bytes: detection is by extension; sniff the delimiter from the first line for a note only.
- **Relevant Skills:** Rust core, file-format signatures.
- **Approved Dependencies:** `zip`.
- **Rationale:** architecture.md §5 requires multi-signal detection and mandates reporting mismatches/encryption/corruption rather than guessing.
- **Security Considerations:** Bounded header read; `zip` crate errors mapped to `Corrupt`, never panicked; entry-name path components are never used here (only inspected as strings).
- **Testing Requirements:** In-memory/tempfile fixtures: PNG bytes named `.docx` (mismatch note); minimal DOCX zip (empty `word/document.xml`); EPUB zip with `mimetype`; RTF; plain text; `%PDF-1.4 … /Encrypt` flag; OLE2 header; empty file → error; truncated `PK` file → `Corrupt`.
- **Acceptance Criteria:** All fixture cases assert the exact `format` + relevant notes; no test depends on the file extension alone passing.
- **Definition of Done:** Tests green; detection has zero `unwrap()` on user-controlled data.

---

## P0-S03 — Asset manager

- **Step ID:** P0-S03
- **Objective:** Central asset pipeline: dedupe, stable collision-free names, MIME by magic, manifest, disk write-out.
- **Depends On:** P0-S01
- **Files to Create:** `src/assets.rs`, tests inside the module
- **Files to Modify:** `src/lib.rs`
- **Inputs:** architecture.md §6, §9.
- **Outputs:** `pub struct AssetManager`; `add(&mut self, bytes: Vec<u8>, origin: &str, alt: Option<String>) -> usize` (index into `Document.assets`); `write_all(&self, dir: &Path) -> Result<usize>`; `manifest(&self) -> serde_json::Value`.
- **Exact Instructions:**
  1. State: `assets: Vec<Asset>`, `by_hash: HashMap<u64, usize>`, a counter per kind prefix.
  2. Hash = `std::collections::hash_map::DefaultHasher` over the bytes (deterministic within a process; sufficient for dedupe — document this in code comments). Duplicate → return the existing index without creating a new asset entry.
  3. Extension/MIME from magic bytes (reuse the image magic table from `detect.rs`; unknown binary → `.bin`, `application/octet-stream`). Name pattern: `image-{seq:03}.{ext}` (all visual assets use the `image-` prefix, matching the convention used throughout the renderers).
  4. `rel_path = "assets/<filename>"`. `write_all` creates `dir/assets`, writes each non-duplicate asset, returns the count written. Refuses to write outside `dir` (filenames are always generated, never derived from input paths — asserts no `/` in filename, matching the zip-slip mitigation in architecture.md §9).
  5. `manifest()` → `{"assets":[{filename, mime, bytes, origin, duplicate_of}]}`.
- **Relevant Skills:** Rust core.
- **Approved Dependencies:** none beyond std + `serde_json` for the manifest.
- **Rationale:** One asset pipeline shared by every extractor keeps naming, dedupe, and manifest logic consistent regardless of source format.
- **Security Considerations:** Filenames are always generated, never derived from input content — path traversal from a malicious document is structurally impossible at write time.
- **Testing Requirements:** Dedupe test (same bytes added twice → one file, second index equals first); distinct bytes → sequential names; magic-byte detection → correct extension; `write_all` produces the expected files on disk.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P0-S04 — TXT + Markdown extractors

- **Step ID:** P0-S04
- **Objective:** TXT → paragraph blocks; Markdown → full CDM parse (so md→txt works); md→md is a passthrough with validation warnings (AD-8).
- **Depends On:** P0-S01
- **Files to Create:** `src/extract/mod.rs`, `src/extract/text.rs`
- **Files to Modify:** `src/lib.rs`
- **Inputs:** architecture.md §0.2, §7, AD-8.
- **Outputs:** `extract/mod.rs`: `pub struct ExtractCtx<'a> { pub options: &'a crate::config::Options, pub assets: &'a mut AssetManager, pub warnings: &'a mut Vec<String> }` — `Options` is introduced in this step as a minimal `#[derive(Clone)] pub struct Options` in a new `src/config.rs`, containing only the fields needed so far: `format_choice, embed_assets, ocr_mode, ocr_lang, ocr_command, include_hidden_sheets, preserve_formulas, include_comments, overwrite, strict, max_file_bytes, report_kind, describe_images, include_notes, export_charts, jobs`, with sensible defaults. Profile/config-file parsing arrive in P0-S08 as `config.rs` widens. Functions: `pub fn extract_txt(bytes, meta) -> Document`; `pub fn extract_markdown(bytes, meta) -> Document`.
- **Exact Instructions:**
  1. TXT: normalize `\r\n`/`\r` → `\n`; split on blank lines → `Paragraph` blocks of plain `Text`; lines of only `---`/`***`/`===` (≥3 chars) → `Rule`. `Meta.source_format = Txt`.
  2. MD parser, line-oriented state machine: ATX headings `#{1,6}` → `Heading` (clamp level ≤6); fenced code blocks with an optional info string → `CodeBlock`; indented (4-space) code outside lists → `CodeBlock`; `>` blockquote (recursively parses inner lines) → `Quote`; `---`/`***`/`___` → `Rule`; unordered `-`/`*`/`+` and ordered `1.` list items, 2-space-per-level nesting → `List` with `ListItem` levels; a blank line closes a list; table `| a | b |` + separator row `|---|---:|` → `Table` (alignment parsed from the separator, `\|` unescaped); footnote definitions `[^id]: text` → `Document.footnotes`; paragraph inline parsing — `**bold**`, `*italic*`/`_italic_`, `~~strike~~`, `` `code` ``, `[text](url)`, `![alt](src)` (a remote `http(s)://` or missing local file becomes a `Link` with a note warning "external/missing image not embedded"), `[^id]` refs → `FootnoteRef`, `$…$`/`$$…$$` → math. Inline parsing is recursive descent over the line string; unmatched markers degrade to literal text — characters are never dropped.
  3. md→md handling: the pipeline (P0-S08) short-circuits and copies source bytes through when output format is `md` and input is Markdown; the parser above is still used for md→txt.
- **Relevant Skills:** parsing.
- **Approved Dependencies:** none.
- **Rationale:** TXT and Markdown are the simplest inputs and prove the CDM/renderer contract before anything format-specific is added.
- **Security Considerations:** Text only, no injection surface; recursive descent must be iterative-safe for long lines (no unbounded recursion proportional to line length).
- **Testing Requirements:** Golden-style tests: nested list levels; table with `\|` escape and alignment; footnote round-trip; emphasis nesting `***both***`; unclosed fence → warning + paragraph fallback; TXT blank-line paragraphing.
- **Acceptance Criteria:** Tests green; parsing never panics on 10 adversarial strings (all markers left unclosed).
- **Definition of Done:** Tests green.

---

## P0-S05 — CSV/TSV extractor

- **Step ID:** P0-S05
- **Objective:** CSV/TSV → `Workbook` → blocks, via the `csv` crate rather than a hand-rolled parser (AD-2 — quoting/escaping/embedded-newline handling is a well-known correctness trap not worth re-deriving).
- **Depends On:** P0-S01, P0-S03
- **Files to Create:** `src/extract/csv.rs`
- **Files to Modify:** `src/extract/mod.rs`, `Cargo.toml` (already declared in P0-S01)
- **Inputs:** architecture.md §0.2, §8, AD-2.
- **Outputs:** `pub fn extract_csv(bytes, meta, delimiter: u8) -> Document`.
- **Exact Instructions:**
  1. UTF-8 lossy decode; strip a leading BOM if present. Build a `csv::ReaderBuilder` with `.delimiter(delimiter).flexible(true).has_headers(false)` over the decoded bytes — the crate handles RFC4180 quoting, `""` escaping, and embedded newlines/commas inside quoted fields; do not hand-parse any of that. Skip fully-empty trailing rows. CRLF is handled by the crate.
  2. Build one `Sheet` named after the filename stem; cells: `kind = Text` (or `Number` if the field parses as `f64` — set `kind` accordingly but `display` keeps the original text, never a reformatted number), reference computed in `A1` style (column letters derived from index).
  3. Sheet-to-blocks lowering (shared function `sheet_blocks(sheet, opts) -> Vec<Block>` in `extract/mod.rs`, reused by `xlsx.rs`/`ods.rs` in Phase 2 — build it generically now, not CSV-specific): emit `Boundary(Worksheet(name))`; if rows ≤ 200 and cols ≤ 20 → `Heading` level 2 "Worksheet: {name}" + `Table` (row 0 = header); else `Heading` + warning "large/irregular sheet exported as CSV listing" + `CodeBlock(language="csv")` of the used range + a summary paragraph (dimensions, non-empty cell count); formulas (none in CSV) are skipped; comments → bullet `List` if `include_comments`.
  4. `Document.workbook = Some(..)` always (the report generator in P0-S08 reads it).
- **Relevant Skills:** parsing, spreadsheet data modeling.
- **Approved Dependencies:** `csv`.
- **Rationale:** AD-2 — the `csv` crate costs nothing extra in the dependency graph beyond what's already justified, and removes a whole class of quoting/escaping bugs a hand-rolled parser would need to re-derive and test from scratch.
- **Security Considerations:** Bounded field length not needed (file-size guard is upstream in P0-S08); lossy decode never fails.
- **Testing Requirements:** Quoted commas/newlines inside fields; empty trailing rows; a 300-row fixture takes the CSV-dump path (assert warning + code block); number-vs-text `kind` inference; TSV via `delimiter = b'\t'`.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P0-S06 — HTML/XHTML extractor

- **Step ID:** P0-S06
- **Objective:** Semantic HTML/XHTML → CDM, with script/nav stripping and an injected asset loader (reused by EPUB in Phase 1, AD-4).
- **Depends On:** P0-S02, P0-S03
- **Files to Create:** `src/extract/html.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §0.6, §9.
- **Outputs:** `pub fn extract_html(bytes, meta, loader: &mut dyn FnMut(&str) -> Option<Vec<u8>>, ctx: &mut ExtractCtx) -> Document`.
- **Exact Instructions:**
  1. Preprocess bytes → String: strip control characters except `\n\t`; single-pass replacement of named entities beyond the XML5 built-ins from a fixed table (~45 entries: `nbsp, mdash, ndash, lsquo/rsquo/ldquo/rdquo, hellip, trade, reg, copy, deg, plusmn, frac12, bull, middot, euro, pound, yen, sect, para, dagger, Dagger, laquo, raquo, times, divide, infin, minus, oelig, scaron, Yuml, fnof, circ, tilde, ensp, emsp, thinsp, zwnj, zwj, lrm, rlm, sbquo, bdquo, permil, lsaquo, rsaquo`) and numeric references (`&#NN;`/`&#xHH;`), except codepoints for `< > & " '` which stay named; unknown named entities → `\u{FFFD}`. The result must be well-formed enough for `quick-xml`.
  2. Streaming parse with `quick-xml` (`Reader`, `read_event_into`, `trim_text=false`). Tag names compared case-insensitively. Void elements (`br img hr meta link input source wbr area base col embed param track`) treated as self-closing even when written as a Start tag.
  3. Skip-subtree set: `script style noscript template iframe svg canvas form button select textarea input nav header footer aside dialog` — on Start, count depth until the matching End (`nav`/`header`/`footer`/`aside` skipped as navigation/decorative interface elements; push a warning once, "skipped N navigation/interface sections").
  4. Semantic mapping: `h1`–`h6` → `Heading` (level); `p` → `Paragraph`; `ul`/`ol`/`li` → `List` (nesting via a list-context stack; `li` content may contain nested lists → `ListItem.blocks`); `table`/`thead`/`tbody`/`tr`/`td`/`th` (colspan/rowspan attrs) → `Table` (first row or `thead` → header); `blockquote` → `Quote` (recursive block sink); `pre` → `CodeBlock` (preserve text verbatim, trim one leading/trailing newline); `code` inside `pre` sets `language` from `class="language-x"` if present; `a[href]` → `Link` (children parsed inline); `img[src,alt]` → load via `loader`: `Some(bytes)` → register in `AssetManager` → `Block::Image` with a caption picked up from an enclosing `figure`/`figcaption`; `loader` returning `None` → warning "image missing or unreadable: src"; `figure`/`figcaption` → caption association; `sup`/`sub`/`strong`/`b`/`em`/`i`/`u`/`s`/`strike`/`del` → `Inline` styles; `hr` → `Rule`; `title` → `Meta.title` if unset; `<br>` → `Inline::Break`.
  5. Whitespace: outside `pre`, collapse runs to a single space, trim at block boundaries; text is flushed at every block start/end.
  6. Malformed tolerance: an unexpected End tag → ignore if not on the open-tag stack; EOF → close all open contexts gracefully. A hard XML-level parse error → stop at that point and return what was built so far plus a warning "HTML truncated at parse error: …" (never return `Err` for HTML — this project wants best-effort output for web-sourced documents; only I/O errors are `Err`).
- **Relevant Skills:** HTML semantics, streaming XML.
- **Approved Dependencies:** `quick-xml`.
- **Rationale:** Document-embedded HTML is typically far cleaner than arbitrary web pages; a tolerant hand-rolled parser on `quick-xml` avoids the `html5ever`+`scraper` weight for a need this narrow (architecture.md §4.2, rejected-dependencies table). Loader injection lets the Phase-1 EPUB extractor reuse this verbatim.
- **Security Considerations:** Loader results are content only; `src` strings never touch the filesystem here (the loader decides); entity preprocessing eliminates XML-unknown-entity parse aborts.
- **Testing Requirements:** Semantic article fixture → headings/lists/table/quote/code/link/image; script+nav stripped (assert absent + warning); unclosed-tags fixture; uppercase tags; nested lists; colspan preserved; `figcaption` → caption.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P0-S07 — Renderers + automatic output selection

- **Step ID:** P0-S07
- **Objective:** CDM → Markdown and CDM → TXT, plus automatic md-vs-txt selection.
- **Depends On:** P0-S01
- **Files to Create:** `src/render/mod.rs`, `src/render/markdown.rs`, `src/render/txt.rs`
- **Files to Modify:** `src/lib.rs`
- **Inputs:** architecture.md §0.3, §0.4 — the rules below are the complete specification, not a pointer elsewhere.
- **Outputs:** `pub fn render_markdown(doc: &Document, embed_assets: bool) -> String`; `pub fn render_txt(doc: &Document) -> String`; `pub enum OutFormat { Md, Txt }`; `pub fn choose_output(doc: &Document, choice: FormatChoice) -> (OutFormat, String /*reason*/)`.
- **Exact Instructions:**
  1. **Markdown renderer:** Heading `#`×`min(level,6)`; Paragraph text via the inline serializer; inline serializer handles `Text` (escape `` \ * _ ` [ ] < > | # `` at line start, `!` before `[`), `Styled` (nest `**`/`*`/`~~`; underline is unrepresentable in plain Markdown — dropped silently by design, no HTML fallback emitted), `Code` (use `` ``` `` fencing if content contains backticks), `Link` `[t](u)` (escape `)` in the URL as `%29`), `MathInline` `$…$`, `FootnoteRef` `[^id]`, `Break` → two trailing spaces + newline, `Image` `![alt](assets/…)` or a data-URI when `embed_assets` (base64, `data:{mime};base64,`). `List`: `- ` / `{n}. ` with 2-space indent per level (`start` = `List.start`). `Table`: header + `|---|` separator with `:` alignment, `|` escaped in cells, colspan expanded with empty cells, rowspan collapsed to the first row, `caption` as an italic paragraph after the table; `approximate` → HTML comment `<!-- approximate reconstruction -->` before the table. `CodeBlock`: fence with a backtick run one longer than any run in the content. `MathBlock`: `$$…$$`. `Quote`: prefix every rendered line with `> `. `Rule`: `---`. `Boundary`: `Page` → `<!-- page N -->`; `Slide` is handled by the extractor as a heading; `Chapter` → `---`; `Worksheet` is already emitted as a heading by the extractor. `FormulaList` → `### Formulas` + bullets `` `D2 = B2 * C2` ``. `Placeholder` → a quote line `> [Chart: title — preserved as an external asset]`-style (an image-backed placeholder with an asset renders as a normal `Image` line instead). Footnotes appended at file end `[^id]: text`. Guarantees: at most one consecutive blank line; the file ends with a single `\n`; pipes are never left unescaped inside a table.
  2. **TXT renderer:** Heading: uppercase text + `=` underline (level 1) / `-` underline (level ≥2); lists `- `/`1.` with 2-space indent per level; Table → fixed-width: column width = max cell plain-text width (capped at 32), numeric columns right-aligned, 2-space gutter, header underlined with `-`; `CodeBlock` → 4-space-indented block between blank lines; images → `[Image: alt]` (`[Image]` if no alt); Placeholder chart → `[Chart: {label} — preserved as an external asset]`; Quote → 4-space indent; boundaries: Page → `\n---- Page N ----\n`, Chapter/Worksheet → 60-char `=` separator + label; `FormulaList` → `FORMULAS` header + `D2 = B2 * C2` lines; footnotes at end `[1] text`, refs inline as `[1]`; links: `text (url)` when `url != text`, else just `text`.
  3. **Auto-selection** (`choice = Auto` only): `score = +2` per `Table` block, `+2` if a `Workbook` is present, `+2` per `Image`/asset-backed `Placeholder`, `+1` any `CodeBlock`, `+1` any math, `+1` any `Link`, `+1` if footnotes exist, `+1` for nested lists (`level>0`), `+1` if more than one distinct heading level, `+2` for slide/chapter boundaries; `score ≥ 2` → `Md`, else `Txt`; the reason string lists the contributing signals (e.g. "markdown: 3 tables, 2 images, footnotes"). An explicit choice always wins, with reason `"user-specified"`.
- **Relevant Skills:** Markdown/ASCII layout.
- **Approved Dependencies:** `base64` (embed mode only).
- **Rationale:** These rules implement §0.3/§0.4's "structure needs Markdown, plain text is enough otherwise" principle as concrete, testable output rules.
- **Security Considerations:** Escaping prevents Markdown-injection from breaking table structure.
- **Testing Requirements:** Golden tests for the Markdown table example and TXT table example from §0.3; pipe-escaping; nested-list indent; footnote emission; underline dropped silently; auto-selection in both directions with correct reason strings.
- **Acceptance Criteria:** Goldens byte-exact except documented trailing-newline handling.
- **Definition of Done:** Tests green.

---

## P0-S08 — Config, profiles, report, pipeline, CLI

- **Step ID:** P0-S08
- **Objective:** Wire everything into the full pipeline (architecture.md §5, §8) with the CLI features that belong to Phase 0 (single file, preview, reports, profiles, config file, overwrite, quiet/verbose).
- **Depends On:** P0-S02…S07
- **Files to Create:** `src/report.rs`, `src/convert.rs`, `src/main.rs` (replace placeholder), `tests/e2e.rs`
- **Files to Modify:** `src/config.rs` (widen), `src/extract/mod.rs` (`ExtractCtx` gets the full `Options`), `src/lib.rs`
- **Inputs:** architecture.md §5, §8, §10.
- **Outputs:** working `docconvert` binary.
- **Exact Instructions:**
  1. `config.rs` completion: final `Options` fields + `FormatChoice { Auto, Md, Txt }`, `OcrMode { Off, Auto, Force }`, `ReportKind { None, Text, Json }`; `Options::apply_profile(name)` for profiles `academic, technical, plain, spreadsheet, archive, accessibility, tts, ocr, strict` (document each effect in code comments: e.g. `plain` → format Txt, no formula-listing changes; `spreadsheet` → formulas+comments+hidden-sheets on; `academic` → Md+formulas+comments; `tts` → Txt, minimal asset placeholders; `strict` → `strict=true`; `ocr` → OCR mode Auto; `archive` → Md+embed off+report Json; `accessibility` → Md, `include_notes` on); `load_config_file(path)`: `key = value` lines, `#` comments, keys are CLI long names, values parsed with clear `Config` errors on typos (suggest the nearest key); precedence defaults → profile → config file → CLI flags (`clap` `Option` fields distinguish "not passed" from "explicitly set").
  2. `report.rs`: `#[derive(Serialize)] pub struct Report { input, detected_format, extension_match, output_format, format_reason, timestamp (seconds since epoch via SystemTime, no chrono dependency), duration_ms, pages, slides, chapters, worksheets, tables, images, formulas, ocr_status: String, warnings: Vec<String>, error: Option<String>, outputs: Vec<String>, assets: Vec<String>, extractor_version: &'static str = env!("CARGO_PKG_VERSION"), config_summary: Vec<String> }`; `Report::from_document(...)`; `to_json()` (pretty `serde_json`), `to_text()` (aligned human-readable).
  3. `convert.rs`: `pub fn convert_file(path: &Path, opts: &Options) -> Result<Report>` implementing exactly: size guard (default 200MB → fatal `Unsupported("file exceeds --max-size")`), detect, dispatch: Markdown-input + md output → copy bytes through unparsed (AD-8), else extract via `extract::extract_file(path, detection, options) -> Result<Document>` (a dispatcher over `Format`; `Unsupported`/`Encrypted` variants produce clear messages, e.g. Encrypted → "file is encrypted; password-protected files are not supported"), choose output, render, `mkdir -p` output dir (default: the input file's own directory; `-o` overrides; filename = stem + extension; exists && `!overwrite` → skip with report status "skipped (output exists)"), write output, `assets.write_all`, write a report file when `ReportKind != None` (`<stem>.report.json|txt` next to the output), return `Report`. Also `pub fn preview(path, opts) -> Result<String>`: detect + extract + a formatted summary (format, mismatch notes, meta, counts of pages/slides/chapters/worksheets/images/tables/formulas, chosen format + reason, warnings) without writing anything.
  4. `main.rs` (`clap` derive): positional `paths` (Phase 0: first path used; multiple paths and directories are accepted but processed sequentially — parallel batch is P6-S02), flags: `-o/--output`, `-f/--format {auto,md,txt}`, `--profile`, `--config`, `--ocr {off,auto,force}`, `--ocr-lang`, `--ocr-command`, `--embed-assets`, `--include-hidden-sheets`, `--no-formulas`, `--no-comments`, `--describe-images` (accepted; effect = warn "image description is not part of the offline core", AD-9), `--overwrite`, `--strict`, `--max-size <MB>`, `--report {none,text,json}`, `-q/--quiet`, `-v/--verbose`, `--preview <FILE>` (previews instead of converting). Exit codes: 0 success, 1 conversion failed, 2 config/usage error. Verbose → print the text report to stderr; quiet → errors only. Progress line `[1/1] file → out.md (md, 3 warnings)` unless quiet.
  5. `extract/mod.rs`: `extract_file` dispatcher covering the Phase-0 formats; `Pdf/Docx/Xlsx/Pptx/Odt/Ods/Epub/Rtf/Doc/Xls/Odp/Image` → `Unsupported("<fmt> support lands in Phase N (blueprint.md)")` until their phases land; `Unknown` → `Unsupported("unknown file format: <notes>")`.
- **Relevant Skills:** CLI design, orchestration.
- **Approved Dependencies:** `clap`, `serde_json`, `walkdir` (declared now, used starting P6), `base64`.
- **Rationale:** This wires the whole Phase-0 pipeline per architecture.md §5/§8; the precedence chain is explicit by design (architect.md's Explicit Decisions principle).
- **Security Considerations:** `--max-size` enforced before parsing; output directory created with default permissions; no shell-outs in Phase 0 (the OCR adapter's `Command` array-argv usage arrives in Phase 5).
- **Testing Requirements:** `tests/e2e.rs` using a `tempfile`-free pattern (`std::env::temp_dir()` + a unique suffix — do not add a `tempfile` dependency for this): txt→md, md→txt, csv→md, html→md with an image asset written to `assets/`, preview contains correct counts, overwrite-skip path, unknown-format exit message, config-file override test, profile-application test.
- **Acceptance Criteria:** `cargo test` all green; manual smoke test: run the binary on each sample in `tests/`.
- **Definition of Done:** Tests green; `agent.md` updated; the Phase-0 exit-criteria checklist recorded in `agent.md`.

---

# Phase 1 — Word-Processing & Books, Fully Detailed

---

## P1-S01 — Shared OOXML helpers + relationships parser

- **Step ID:** P1-S01
- **Objective:** Factor the logic DOCX and PPTX both need (Phase 3) — package relationships,
  run-property parsing, table-cell parsing — so it's written once, not duplicated.
- **Depends On:** P0-S01, P0-S03
- **Files to Create:** `src/extract/ooxml.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §4.2 AD-4.
- **Outputs:** internal helpers used by `docx.rs` (this phase) and `pptx.rs` (Phase 3):
  `pub fn parse_rels(bytes: &[u8]) -> HashMap<String, String>` (rId → target path, from any
  `_rels/*.rels` part); `pub fn parse_run_properties(reader: &mut Reader, ns: char) ->
  TextStyle` parameterized on the run-properties namespace prefix (`'w'` for WordprocessingML,
  `'a'` for DrawingML — both use the same element shapes: bold/italic/strike/underline/vert-
  align toggles, just different tag prefixes); `pub fn parse_table_cell(reader: &mut Reader,
  ns: char) -> TableCell` (handles `gridSpan`/`colspan` and `vMerge`/vertical-merge-to-rowspan
  the same way in both namespaces); `pub fn omml_to_latex(reader: &mut Reader) -> String`
  (OOXML Math → LaTeX, used by DOCX now, available to PPTX later if needed).
- **Exact Instructions:**
  1. `parse_rels`: stream-parse a `.rels` XML part with `quick-xml`; each `<Relationship Id="rIdN" Type="…" Target="…"/>` → insert `(Id, Target)`. Targets are stored as given (relative paths); callers resolve against the *part's own directory* (e.g. slide rels resolve relative to `ppt/slides/`, not the package root — this is the exact bug class that causes wrong images in naive implementations).
  2. `parse_run_properties`: on encountering `<{ns}:rPr>` (or `<{ns}Pr>` for DrawingML's `<a:rPr>`), scan child start-tags until the matching end: `b`/bold-attr-`0` variants toggle `TextStyle.bold`; `i`→italic; `strike`→strike; `u` (any `val` other than `"none"`)→underline; `vertAlign val="superscript"`→sup, `val="subscript"`→sub. Both namespaces use the same tag *names* under their own prefix, so one function body handles both — only the prefix byte differs in the tag-match.
  3. `parse_table_cell`: read `gridSpan`/`w:gridSpan val="N"` → `colspan=N` (DrawingML tables use `gridSpan` too). Vertical merge: `vMerge val="restart"` (or bare `<w:vMerge/>`) starts a merge group *and* is the cell that ends up with `rowspan > 1`; `vMerge val="continue"` (or bare, non-restart) cells are *not* emitted as their own `TableCell` — the caller must count consecutive continuation cells in the same column and add to the restart cell's `rowspan` after the whole table is parsed (a single forward pass can't know the final count). Concrete algorithm: collect raw `(row, col, is_restart, is_continue)` cell records first, then a second pass computes rowspans and produces the final `Vec<Vec<TableCell>>` with continuation cells omitted.
  4. `omml_to_latex`: recursive-descent over `<m:oMath>`/`<m:oMathPara>`. Element mapping (exhaustive for what's implemented; anything else falls through to rule 5): `m:f` (fraction) → `\frac{num}{den}`; `m:sSup` → `{base}^{sup}`; `m:sSub` → `{base}_{sub}`; `m:sSubSup` → `{base}_{sub}^{sup}`; `m:rad` with `m:deg` present and non-empty → `\sqrt[deg]{e}`, else `\sqrt{e}`; `m:nary` → `\sum`/`\int`/`\prod` chosen by the `m:chr` operator attribute, `_{low}^{up}{e}`; `m:d` (delimiter) → `\left{open}{e}\right{close}` using the actual bracket characters from `m:dPr` (default `(` `)`); `m:m` (matrix) → `\begin{matrix} … \end{matrix}`, rows joined `\\`, cells joined `&`; `m:acc` → accent char mapped (`\hat`, `\vec`, `\dot`, `\tilde`, fallback: leave the raw accent character adjacent to `{e}`); `m:r`/`m:t` → the run text, verbatim (LaTeX special characters `\ { } _ ^ # $ % &` escaped with a backslash).
  5. Unknown/unimplemented OMML elements: emit their text content verbatim (unescaped structure, escaped LaTeX specials) and push `ctx.warnings` "OOXML math element not fully converted: <tag>; rendered as plain text" — never drop the content, never panic on nesting depth (bound recursion at 64 levels, beyond which flatten to text with a warning).
- **Relevant Skills:** OOXML.
- **Approved Dependencies:** `quick-xml` (already in the tree).
- **Rationale:** DOCX and PPTX share WordprocessingML/DrawingML's run-property and table shapes closely enough that one parameterized implementation is both less code and less bug surface than two near-duplicates — exactly the kind of factoring a 24-crate-per-format design would have made harder, and a single-module design makes natural.
- **Security Considerations:** Bounded recursion depth in `omml_to_latex` (billion-laughs-style nested-group protection, consistent with architecture.md §9).
- **Testing Requirements:** `parse_rels` on a sample `.rels` file; run-properties on nested bold+italic; `vMerge` two-pass rowspan computation on a 3-row vertical merge; each OMML mapping rule (fraction, superscript, subscript, both together, radical with/without degree, sum with bounds, parenthesized expression, 2×2 matrix, accent); unknown OMML element → verbatim text + warning, not a panic.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P1-S02 — DOCX extractor

- **Step ID:** P1-S02
- **Objective:** Full-fidelity DOCX extraction: headings by style ID (not locale-dependent
  name), lists via `numbering.xml`, tables with real merges, images, footnotes, native math.
- **Depends On:** P1-S01
- **Files to Create:** `src/extract/docx.rs`
- **Files to Modify:** `src/extract/mod.rs` (dispatch `Format::Docx`)
- **Inputs:** architecture.md §0.2, AD-4.
- **Outputs:** `pub fn extract_docx(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>`.
- **Exact Instructions:**
  1. Open as ZIP (`zip` crate). A read error that looks like `/Encrypt`-style OOXML encryption (an `EncryptedPackage`/`EncryptionInfo` stream, or the ZIP central directory being unreadable while an OLE2 wrapper is present) → `ConvertError::Encrypted`; a structurally broken ZIP → `ConvertError::Corrupt`.
  2. `docProps/core.xml` → `Meta` (title, author/creator, subject, created, modified). Missing part is not an error — `Meta` fields stay `None`.
  3. `word/styles.xml`: build `HashMap<String, u8>` mapping **`w:styleId`** (never the locale-dependent `w:name`) → outline level, from `<w:outlineLvl w:val="N">` when present, falling back to matching `w:name val="heading N"` (covers older files that never set an explicit outline level). This is the single most common DOCX-heading bug in naive implementations — get the key right.
  4. `word/numbering.xml`: `HashMap<u32, (bool /*ordered*/, Option<u32> /*start*/)>` keyed by `numId`. Numbering format `decimal`/`lowerRoman`/`upperRoman`/`lowerLetter`/`upperLetter` → ordered; `bullet`/`none` → unordered. (`numId` in the document body indexes into an abstract-numbering indirection layer — resolve `w:num/w:abstractNumId` → the actual level definitions before building this map, don't key directly off the document-body `numId` against level definitions meant for a different abstract list.)
  5. `word/_rels/document.xml.rels` via `ooxml::parse_rels`.
  6. `word/footnotes.xml` if present: each `<w:footnote w:id="N">` → a footnote entry, **skipping** `w:type="separator"`/`"continuationSeparator"` (these are formatting artifacts, not content).
  7. Stream-parse `word/document.xml`:
     - `<w:p>`: check `w:pPr/w:pStyle/@w:val` against the heading map → `Block::Heading{level: outline+1, ..}` (clamped ≤6). Else check `w:pPr/w:numPr` (`w:numId`, `w:ilvl`) → list item; **group consecutive same-`numId` paragraphs** into one `Block::List`, `ilvl` driving `ListItem.level` (a change in `numId` — even at the same `ilvl` — starts a new list, matching how Word itself treats separate lists). Else → `Block::Paragraph`.
     - Runs (`<w:r>`): `parse_run_properties` (P1-S01, `ns='w'`) for style; `<w:t>` → text (preserve `xml:space="preserve"` significant whitespace); apply style wrapper via `Inline::Styled`.
     - `<w:hyperlink r:id="rIdN">` → resolve via rels → `Inline::Link`; `<w:hyperlink w:anchor="…">` (internal bookmark link, no rId) → `Inline::Link` with a `#anchor` url.
     - `<w:footnoteReference w:id="N">` → `Inline::FootnoteRef("N")`.
     - `<w:tbl>` → `Block::Table` via `ooxml::parse_table_cell` (`ns='w'`) with the two-pass rowspan algorithm from P1-S01; header detection: first row is header if every cell's first run has `w:b` set.
     - `<w:drawing>` → find `a:blip/@r:embed`, resolve via rels → `word/media/…` → read bytes → register via `AssetManager` → `Block::Image`; if the immediately following paragraph has style `Caption`, use its text as the image's caption instead of emitting it as a separate paragraph.
     - `<m:oMath>`/`<m:oMathPara>` → `ooxml::omml_to_latex`; if it's the paragraph's only content → `Block::MathBlock`, else → `Inline::MathInline`.
  8. Any part read that's missing or malformed beyond the tolerances above → a warning, not a fatal error (e.g. an unreadable `numbering.xml` degrades all lists to plain paragraphs with a warning, rather than failing the whole document).
- **Relevant Skills:** OOXML WordprocessingML.
- **Approved Dependencies:** `zip`, `quick-xml` (both already in the tree).
- **Rationale:** This is the highest-value format in the word-processing group and the one where naive implementations most visibly break (locale-dependent heading styles, vertical merges, list grouping) — architecture.md §0.5's content-preservation goal is tested hardest here.
- **Security Considerations:** ZIP-slip and entry-count/size guards from architecture.md §9 apply; `word/media/` paths are read by name from the rels-resolved target, never taken verbatim from arbitrary document content.
- **Testing Requirements:** All six fixtures and acceptance checks from the earlier draft's DOCX phase, kept because they're still exactly the right tests: 3-level headings using a non-English style-name locale (proves `styleId` keying, not `name` keying); nested ordered+unordered list; table with both `gridSpan` and `vMerge`; inline image; native equation (`$$…$$` in Markdown output); footnote. Add: two consecutive same-`ilvl` lists with different `numId` render as two separate lists, not one merged list.
- **Acceptance Criteria:** All fixtures pass; heading levels come from `styleId` in every test, never from a hardcoded `name` string match alone.
- **Definition of Done:** Tests green.

---

## P1-S03 — EPUB extractor

- **Step ID:** P1-S03
- **Objective:** EPUB → CDM, reading order from the OPF spine (never filename order), reusing
  the Phase-0 HTML extractor via loader injection (AD-4).
- **Depends On:** P0-S06, P0-S03
- **Files to Create:** `src/extract/epub.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §0.2, §4.2 (epub crate rejected — AD-4), AD-4.
- **Outputs:** `pub fn extract_epub(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>`.
- **Exact Instructions:**
  1. Open as ZIP. Read `META-INF/container.xml` → `rootfile/@full-path` → the OPF file's path (this is EPUB's actual entry point; **do not assume** `content.opf` at a fixed location — some packages nest it under a different directory).
  2. Parse the OPF (`quick-xml`): `<metadata>` → `Meta` (`dc:title`, `dc:creator`, `dc:language`); `<manifest><item id="…" href="…" media-type="…">` → `HashMap<id, (href, media_type)>`; `<spine><itemref idref="…">` → the **reading order** — resolve each `idref` through the manifest to get the actual XHTML file path. This spine order is the sole source of chapter order; a package's internal file naming (`ch1.xhtml`, `ch2.xhtml`, …) is not reliable proof of order and must never be used instead.
  3. For each spine item, in spine order: read its XHTML bytes from the ZIP; call `extract::html::extract_html` (P0-S06) with a `loader` closure that resolves an `<img src="…">` **relative to the spine item's own path within the archive** (e.g. spine item `OEBPS/text/ch01.xhtml` with `src="../images/fig1.png"` resolves to `OEBPS/images/fig1.png`, not to the archive root) — get this path-resolution step right, it's the most common EPUB-image bug. Append the returned blocks; if this isn't the first spine item, prepend a `Block::Boundary(Boundary::Chapter(title))` (title = the chapter's first `Heading` block's text if one exists, else `None`).
  4. Footnotes: EPUB3 marks these with `epub:type="footnote"` on an `<aside>`, referenced by an `<a href="#id">`. After the HTML extractor returns a chapter's blocks, scan for anchor links whose target resolves to an in-package `id` on an `aside[epub:type="footnote"]` element — the HTML extractor doesn't know about EPUB-specific attributes, so this pass happens here, in `epub.rs`, as a post-process over the returned `Vec<Block>`/`Vec<Inline>`: replace the link with `Inline::FootnoteRef(id)`, move the aside's content into `Document.footnotes`, and remove it from the chapter body.
  5. Cover image: if the manifest has an item with `properties="cover-image"` (EPUB3) or the OPF `<meta name="cover" content="id"/>` (EPUB2 fallback), register it as an asset but do **not** insert it as a body `Block::Image` — it's metadata, not chapter content; note its asset index in a `Meta` extension field if useful for renderers, otherwise it's simply available in `Document.assets`.
- **Relevant Skills:** EPUB/OPF packaging, HTML semantics (reused).
- **Approved Dependencies:** `zip`, `quick-xml` (both already in the tree) — no `epub` crate (AD-4).
- **Rationale:** Reusing the HTML extractor rather than writing a second XHTML walker is the entire point of AD-4's dependency-avoidance argument paying off in actual code reuse, not just avoided `Cargo.toml` lines.
- **Security Considerations:** Same ZIP guards as elsewhere; spine/manifest `idref` resolution never trusts a path outside the archive (normalized and checked to stay within the ZIP's own entries).
- **Testing Requirements:** 2-chapter EPUB fixture with one image at a nested path and one footnote (programmatically built via the `zip` crate in a test helper, per architecture.md §13 — no committed binary fixtures): assert chapter order follows the spine even when spine order and filename order are deliberately made to differ; nested-path image resolves; `Boundary::Chapter` appears between chapters, not before the first one; footnote round-trips into `Document.footnotes` and the in-text reference becomes a proper `FootnoteRef`; a novel-style EPUB (no images/tables) scores low enough in P0-S07's auto-selection to choose `.txt`; an illustrated EPUB scores high enough for `.md`.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P1-S04 — ODT extractor

- **Step ID:** P1-S04
- **Objective:** OpenDocument Text → CDM: style-inheritance-aware bold/italic, correct list
  typing, and — the one ODT-specific correctness trap — never treating a covered table cell
  as real data.
- **Depends On:** P0-S01, P0-S03
- **Files to Create:** `src/extract/odt.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §0.2, AD-4.
- **Outputs:** `pub fn extract_odt(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>`.
- **Exact Instructions:**
  1. Open as ZIP; read `meta.xml` → `Meta`; read `styles.xml` and `content.xml`.
  2. **Style resolution:** build `HashMap<String, ResolvedStyle{bold, italic}>` by walking each `<style:style style:name="X" style:parent-style-name="Y">`'s `style:parent-style-name` chain up to the root, merging `fo:font-weight="bold"`/`fo:font-style="italic"` found at any level (a child style's explicit property wins over an inherited one; an unset property inherits). Cache resolved results — a document can have deep chains and this must not be re-walked per paragraph.
  3. **List typing:** for each `<text:list-style style:name="X">`, inspect its `<text:list-level-style-*>` children: `text:list-level-style-number` at the relevant level → ordered; `text:list-level-style-bullet` → unordered. A list references its style via `<text:list text:style-name="X">`.
  4. Stream-parse `content.xml`'s `<office:text>`:
     - `<text:h text:outline-level="N">` → `Block::Heading{level: N}`.
     - `<text:p>` → `Block::Paragraph`, inline runs via the resolved-style map (`<text:span text:style-name="X">` looks up `X` in the resolver).
     - `<text:list>` → `Block::List` (ordered per step 3); nested `<text:list>` inside a `<text:list-item>` → nested `ListItem.blocks`.
     - `<table:table>` → `Block::Table`. `<table:table-row>` → row; `<table:table-cell table:number-columns-spanned="N" table:number-rows-spanned="M">` → cell with `colspan=N, rowspan=M`. **`<table:covered-table-cell>` elements are the placeholder markers ODF emits for cells covered by a merge — they must be recognized by tag name and skipped entirely, never emitted as an empty `TableCell`.** (This is the ODT-specific version of the same merge-handling correctness issue P1-S01 solves for OOXML, but ODF's mechanism is a distinct covering-placeholder tag rather than a `vMerge` continuation attribute — handle it as its own case, don't try to force it through the OOXML two-pass algorithm.)
     - `<draw:frame><draw:image xlink:href="Pictures/…">` → read bytes from the ZIP path directly (ODF images are referenced by direct archive path, not through a separate relationships file the way OOXML is) → register asset → `Block::Image`.
     - `<text:note text:note-class="footnote">` → footnote content is **inline** in ODF (unlike DOCX's separate `footnotes.xml`) — `<text:note-citation>` gives the marker, `<text:note-body>` gives the content; extract both at the point of occurrence into `Document.footnotes` plus an `Inline::FootnoteRef` at that location.
- **Relevant Skills:** ODF.
- **Approved Dependencies:** `zip`, `quick-xml` (both already in the tree).
- **Rationale:** ODT's style-inheritance and covered-cell mechanics are different enough from OOXML's that sharing P1-S01's helpers wouldn't actually save real code — hand-written here, consistent with AD-4.
- **Security Considerations:** Same ZIP guards as elsewhere.
- **Testing Requirements:** Heading level from `text:outline-level`; bold inherited through a 2-level parent-style chain resolves correctly; a table with a horizontal+vertical merge asserts the covered cells never appear as empty `TableCell` entries in the output; inline footnote round-trips without a separate-file lookup.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P1-S05 — RTF extractor

- **Step ID:** P1-S05
- **Objective:** Hand-written RTF tokenizer (`logos`) + recursive-descent group walker;
  correctness bar: never panic on malformed/truncated input, degrade with a warning instead.
- **Depends On:** P0-S01
- **Files to Create:** `src/extract/rtf.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §4.2 (`logos`, `encoding_rs`; `rtf-parser` rejected — see
  rejected-dependencies table).
- **Outputs:** `pub fn extract_rtf(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>`.
- **Exact Instructions:**
  1. Token enum via `#[derive(logos::Logos)]`: `GroupStart` (`{`), `GroupEnd` (`}`), `ControlWord` (regex `\\[a-zA-Z]+(-?\d+)?`, captured as `(name: String, param: Option<i32>)`), `HexEscape` (regex `\\'[0-9a-fA-F]{2}`, captured as a raw byte — these bytes are in the *current codepage*, not UTF-8, and must be buffered and decoded together with adjacent hex escapes via `encoding_rs` using the state's current codepage before becoming `Inline::Text`, not decoded one byte at a time), `ControlSymbol` (regex `\\[^a-zA-Z\n]`, e.g. `\~` non-breaking space, `\-` optional hyphen, `\_` non-breaking hyphen), `PlainText` (regex `[^\\{}\n]+`).
  2. Parser state: a `Vec<GroupState>` stack, each holding `{bold, italic, strike, underline, super_sub: Option<SuperSub>, font_size_half_pt: Option<u32>, codepage: u16 /* inherited from the document's \ansicpg or \pc/\pca default */, skip: bool}`. `GroupStart` pushes a **clone** of the current top state (so unclosed toggles inside a group don't leak out — this is the entire reason RTF uses a state stack); `GroupEnd` pops it.
  3. Destination groups to skip entirely (set `skip=true` for the pushed state and don't emit any text while it or a descendant is on top): `\fonttbl`, `\colortbl`, `\stylesheet`, `\info`, `\generator`, `\*\…` (any group starting with the ignorable-destination control symbol `\*` whose control word isn't independently recognized).
  4. Recognized control words while not skipping: `\b`/`\b0` → toggle `bold`; `\i`/`\i0` → `italic`; `\strike`/`\strike0` → `strike`; `\ul`/`\ulnone` → `underline`; `\super`/`\sub`/`\nosupersub` → `super_sub`; `\fsN` → `font_size_half_pt = N` (half-points: `\fs24` = 12pt); `\ansicpg N` (document-level, before the first group typically) → sets the base codepage used for `\'XX` hex-escape decoding via `encoding_rs` (default Windows-1252 if absent); `\par`/`\pard` → finalize the current paragraph's accumulated inlines into a `Block::Paragraph`, start a new one; `\pict` → enters a picture sub-group: look for `\pngblip` or `\jpegblip` to pick the format, then the group's following hex-digit-pair text (whitespace-tolerant) is the image bytes — decode, register asset, emit `Block::Image`. `\wmetafile`/`\emfblip` (WMF/EMF) have no decoder here — emit `Placeholder{kind: Drawing}` + a warning "WMF/EMF image not decoded" rather than attempting conversion. `\field` containing a `\fldinst` whose text starts with `HYPERLINK` → the quoted URL that follows becomes `Inline::Link`'s target, and the sibling `\fldrslt` group's text is the link's visible text.
  5. `PlainText` tokens, outside a skipped destination: become `Inline::Text` wrapped per the current state's active toggles (`Inline::Styled` nesting bold/italic/strike/underline; `Inline::Text` inside an `Inline::Styled{sup/sub}` wrapper when `super_sub` is set).
  6. **Heading inference** (post-process, after all paragraphs are collected — RTF has no native heading concept, so this mirrors PDF's Stage-4.B approach at a much simpler scale): compute the modal (most common) `font_size_half_pt` across all paragraphs as `body_size`. Any paragraph whose first run's size is `≥ 1.2 × body_size` **and** which begins a new "size regime" (i.e. is followed by a paragraph at or below `body_size`) → re-tag as `Block::Heading` with a level chosen by relative size (largest distinct oversized value seen → level 1, next → level 2, capped at 4 — RTF documents rarely have more than a couple of real heading tiers).
  7. **Never panic**: unterminated groups at end-of-input close implicitly (treat EOF as an implicit close of every open group); an unrecognized control word is ignored (its parameter, if any, is simply not applied) rather than causing an error; the whole extractor never returns `Err` for malformed *content* — only `Err` for a hard I/O failure. A `\'` hex escape with fewer than 2 following hex digits is skipped with a warning, not a panic on the unwrap.
- **Relevant Skills:** RTF control-word grammar.
- **Approved Dependencies:** `logos`, `encoding_rs` (both already in the tree).
- **Rationale:** No maintained, native-only Rust RTF crate exists that fits this project's dependency posture (`rtf-parser`'s only non-serde dependency is WASM-oriented `tsify`/`wasm-bindgen` — checked and rejected, architecture.md §4.2). RTF's actual grammar (nested groups, destination-skipping, codepage-dependent hex escapes) is bounded enough to hand-write safely with `logos` doing the lexing.
- **Security Considerations:** Group-stack depth capped (architecture.md §9-style bounded-recursion guard against a maliciously deep `{{{{...` nesting bomb) — beyond the cap, treat as `Corrupt` rather than growing the stack unbounded.
- **Testing Requirements:** Bold/italic/super/subscript toggle on and correctly un-toggle at group close (proving the stack-clone-per-group mechanism, not just linear toggling); `HYPERLINK` field → `Inline::Link`; embedded `\pngblip`/`\jpegblip` decode to assets; `\wmetafile` → placeholder + warning, not a crash; heading inference on a synthetic 3-tier document produces sane levels; a battery of 15 deliberately truncated/malformed fixtures (unterminated group, dangling `\'` with 0/1 hex digits, `\'` at EOF, 200-deep nested groups) — none panics, all return a `Document` (possibly mostly empty) plus warnings.
- **Acceptance Criteria:** Tests green; zero panics across the malformed-input battery.
- **Definition of Done:** Tests green.

---

## P1-S06 — Phase-1 integration + cross-format tests

- **Step ID:** P1-S06
- **Objective:** Wire all four Phase-1 extractors into `extract::extract_file`'s dispatcher
  and prove they interoperate correctly with Phase-0's renderers and the CLI pipeline.
- **Depends On:** P1-S02, P1-S03, P1-S04, P1-S05
- **Files to Create:** `tests/phase1_integration.rs`
- **Files to Modify:** `src/extract/mod.rs` (replace the Phase-0 placeholder `Unsupported` arms for `Docx/Epub/Odt/Rtf` with real dispatch)
- **Inputs:** blueprint.md P0-S08 §5 (`extract_file` dispatcher).
- **Outputs:** working end-to-end conversion for DOCX/EPUB/ODT/RTF through the CLI.
- **Exact Instructions:**
  1. Update the `extract_file` match arms for `Format::Docx | Epub | Odt | Rtf` to call the corresponding Phase-1 function instead of returning `Unsupported`.
  2. End-to-end CLI tests (`tests/phase1_integration.rs`, same pattern as P0-S08's `tests/e2e.rs`): each of the four formats converts to `.md` with assets written where applicable; `--preview` reports correct counts (footnotes, images, tables) for each; a DOCX→txt conversion strips structure sensibly (auto-selection or explicit `-f txt`); `--report json` produces a valid report for each format.
  3. Cross-format regression: run the full Phase-0 test suite again unmodified — adding four dispatcher arms must not change any Phase-0 test's output (a common regression class: accidentally widening a shared helper's behavior while wiring in a new caller).
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new.
- **Rationale:** architect.md's Step Review — a phase isn't done until it's proven end-to-end through the CLI, not just unit-tested per extractor in isolation.
- **Security Considerations:** none beyond what each extractor already covers.
- **Testing Requirements:** as in Exact Instructions.
- **Acceptance Criteria:** All Phase-0 tests still green; all Phase-1 end-to-end tests green.
- **Definition of Done:** `cargo test` fully green; `agent.md` updated with the Phase-1 exit-criteria checklist.

---

# Phase 2 — Spreadsheets, Fully Detailed

---

## P2-S01 — XLSX + XLS core via `calamine`

- **Step ID:** P2-S01
- **Objective:** Open XLSX and legacy XLS workbooks through one reader (`calamine` handles
  both — AD-14) and populate the CDM's `Workbook`/`Sheet`/`Cell` types.
- **Depends On:** P0-S01
- **Files to Create:** `src/extract/xlsx.rs`
- **Files to Modify:** `Cargo.toml` (add `calamine`), `src/extract/mod.rs`
- **Inputs:** architecture.md §4.2 AD-3/AD-14, §7 (CDM).
- **Outputs:** `pub fn extract_xlsx(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>` (handles both `Format::Xlsx` and `Format::Xls`).
- **Exact Instructions:**
  1. `calamine`'s exact API surface (`Reader` trait methods, the `Data` enum's variant set, and how formulas/merges are exposed) has moved across versions — **verify against the resolved version's docs.rs page before writing this step's code**, don't hand-type method names from memory. The algorithm below is written at a level stable across those versions.
  2. Open via `calamine`'s auto-detecting workbook opener over an in-memory cursor (bytes are already loaded, not a filesystem path — this project reads via its own file-size-guarded I/O in P0-S08, not `calamine`'s path-based opener). A file that fails to open as any calamine-supported format → `ConvertError::Corrupt`.
  3. `Meta.worksheet_count = Some(sheet_names().len())`. For each sheet, in the workbook's own sheet order (never re-sorted): create a `Sheet{name, hidden: <from calamine's sheet visibility API if the resolved version exposes it, else default false with a one-time warning "hidden-sheet detection unavailable in this calamine version">, ..}`.
  4. For each cell in the sheet's used range (skip fully-empty leading/trailing rows and columns — don't materialize a `Sheet` sized to a workbook's often-oversized declared dimensions): compute the `A1`-style `reference`. Map calamine's value variant → `(display: String, kind: CellKind)`: empty → `(String::new(), Empty)`; string → `(s, Text)`; boolean → `("TRUE"/"FALSE", Bool)`; error variant → `(format!("{:?}", err), Error)`; integer/float → the tricky case (step 5); datetime variant (where the resolved calamine version has a dedicated datetime type) → format as ISO 8601 (`YYYY-MM-DD` or `YYYY-MM-DDThh:mm:ss` depending on whether a time component is present) → `(iso_string, Date)`.
  5. Numeric cells: a spreadsheet library alone can't tell you "this is a date" from the value — that comes from the cell's **number format string**, which `calamine` exposes per-cell in recent versions (again: verify the exact accessor on the resolved version). If a number-format string is available: match it against common date/time patterns (`"m/d/yyyy"`, `"yyyy-mm-dd"`, `"h:mm"`, etc. — a prefix/keyword match on `y`/`m`/`d`/`h` format codes is sufficient, don't try to fully parse Excel's format-code mini-language) → convert the underlying serial number to a real date (Excel's epoch, accounting for the well-known 1900 leap-year bug) → ISO 8601, `kind = Date`; a format containing `%` → append `%` to the display value (the underlying number is already the fraction, e.g. `0.5` → `"50%"`), `kind = Number`... **no** — use `Number` generically for anything not a date, since `CellKind` (P0-S01) only distinguishes `Empty/Number/Text/Bool/Date/Error`, not percentage/currency sub-kinds; percentage/currency formatting is a *display* detail folded into the `display` string, not a new `CellKind` variant (keep the enum from P0-S01 as designed — don't expand it here without a reason strong enough to justify a CDM change this late). If no number-format string is accessible at all in the resolved version, fall back to: integral value → plain integer string; non-integral → the float's default string form; never guess "this looks date-like" from the raw number alone (a serial-number guess produces false positives on ordinary large numbers) — no format string means `kind = Number`, not a coerced guess.
  6. Formulas: `opts.preserve_formulas` → look up the cell's formula text via calamine's formula-range API (a separate range, parallel to the value range, indexed by the same row/column — confirm the exact accessor name on the resolved version) → `Cell.formula = Some(text)` (store the formula exactly as calamine returns it, `=`-prefix convention matching the resolved version's own output — don't reformat it). `Cell.display` is **always** populated from the cached value regardless of `preserve_formulas` (AD-6 — a formula's presence never blanks out the display value).
  7. Legacy XLS (`Format::Xls`) goes through the exact same code path as XLSX above — `calamine`'s `Reader` trait abstracts over both. The two fidelity gaps noted in AD-14 (formula text may be `None` even when a formula exists, for old BIFF files where calamine can't recover the formula string; chart images are never extracted) degrade silently to `Cell.formula = None` (display value still correct) and a `Placeholder{kind: Chart, asset: None}` respectively — both are warnings, not failures, and both are documented limitations rather than bugs (AD-14).
- **Relevant Skills:** spreadsheet data modeling.
- **Approved Dependencies:** `calamine`.
- **Rationale:** AD-3 (verified lightweight) and AD-14 (XLS is in scope through the same crate) — this single step replaces what an earlier draft split into two separate hand-rolled XLSX/XLS extractors.
- **Security Considerations:** Cell-count guard (a sheet with an absurd declared dimension but sparse actual data must not cause an allocation proportional to the declared size — iterate the used range calamine actually reports, not a naive `0..max_row × 0..max_col` loop over declared bounds).
- **Testing Requirements:** 3-sheet XLSX with SUM/IF/AVERAGE formulas, dates, a boolean cell, and an error cell (`#DIV/0!`); a minimal legacy XLS fixture with a formula (assert display value correct even if formula text comes back `None`); a sheet with a declared 1,000,000-row dimension but only 5 actual rows of data (assert the extractor doesn't attempt to materialize a million empty cells).
- **Acceptance Criteria:** Tests green; no test asserts a `calamine` method name that wasn't independently verified against the resolved version's docs.
- **Definition of Done:** Tests green; the resolved `calamine` version and the specific API names used (formula accessor, hidden-sheet accessor, number-format accessor) recorded in `agent.md` for future steps to rely on without re-deriving.

---

## P2-S02 — Fidelity: merges, comments, hyperlinks, hidden sheets, charts

- **Step ID:** P2-S02
- **Objective:** Close the fidelity gaps P2-S01 left open: merged-cell ranges, comments,
  hyperlinks, hidden-sheet inclusion policy, chart inventory.
- **Depends On:** P2-S01
- **Files to Modify:** `src/extract/xlsx.rs`
- **Inputs:** architecture.md §0.5 (content preservation), AD-14.
- **Outputs:** `Sheet.merged`, `.comments`, `.charts` populated; `Cell.hyperlink` populated.
- **Exact Instructions:**
  1. **Merged cells:** if the resolved `calamine` version exposes merge ranges directly, use that. If not (verify against docs.rs — this capability has varied across versions), fall back: re-open the XLSX's own ZIP (bytes already in memory — this is not a second file read) and stream-parse `xl/worksheets/sheetN.xml` (N = the sheet's position) for `<mergeCells><mergeCell ref="B2:D2"/>` directly with `quick-xml` — this fallback only applies to XLSX (an OOXML zip); legacy XLS has no equivalent to fall back to, so on XLS a missing merge-API means `Sheet.merged` simply stays empty with a one-time warning. Either way, populate `Sheet.merged: Vec<String>` with the raw range references (e.g. `"B2:D2"`) — resolving them into rendered colspan/rowspan happens in the shared `sheet_blocks()` lowering (P2-S03), not here, so this step's output is data, not presentation.
  2. **Comments:** `opts.include_comments` → same pattern as merges: use calamine's comment API if the resolved version has one, else fall back to parsing `xl/comments*.xml` (one file per sheet that has any) for XLSX only. `Sheet.comments: Vec<(String /*cell ref*/, String /*text*/)>`. XLS: same "unavailable, warn once" fallback as merges.
  3. **Hyperlinks:** `Cell.hyperlink` — calamine's hyperlink accessor if available, else (XLSX only) `xl/worksheets/_rels/sheetN.xml.rels` cross-referenced with `<hyperlink ref="A1" r:id="rIdN"/>` entries in the sheet XML.
  4. **Hidden sheets:** `opts.include_hidden_sheets` (default `false`) — when `false`, a `Sheet` whose `hidden = true` (from P2-S01 step 3) is extracted (so it's still counted/available for a future report) but **excluded from the rendered blocks** in P2-S03's lowering, with a summary line noting how many sheets were skipped and why. When `true`, hidden sheets render exactly like visible ones.
  5. **Charts:** no chart *images* are extracted (that would need `xl/charts/chart*.xml` cross-referenced with rendered images in `xl/media/`, which OOXML doesn't guarantee exist as pre-rendered bitmaps at all — Excel renders charts client-side from data, it doesn't ship a picture of every chart). Instead: detect chart *presence* per sheet by checking for `xl/drawings/drawing*.xml` parts referencing `<c:chart>` graphic frames (XLSX only; legacy XLS: unavailable, same warn-once fallback), and record a `Placeholder{kind: Chart, label: <chart title from the chart XML's `c:title` if resolvable, else None>, asset: None}` at the sheet's position in `Sheet.charts`. This is a smaller commitment than the earlier draft's chart-image extraction — chart presence is reported honestly (never silently dropped, matching architecture.md's core principle) without adding image-rendering scope this project has no rendering engine to fulfill anyway.
- **Relevant Skills:** OOXML SpreadsheetML (fallback paths only), spreadsheet data modeling.
- **Approved Dependencies:** `quick-xml` (fallback paths; already in the tree) — no new dependency.
- **Rationale:** Fidelity features layered onto P2-S01's core rather than folded into it, so a `calamine`-version-specific accessor problem in one fidelity feature (say, comments) doesn't block the whole extractor — each falls back or degrades independently.
- **Security Considerations:** Fallback ZIP-reopen reuses the same guards as every other ZIP read (architecture.md §9).
- **Testing Requirements:** Merged-cell fixture (both via a calamine-API path and, if the resolved version lacks it, via the XML-fallback path — test whichever path is actually active); comment fixture; hyperlink fixture; hidden-sheet fixture tested both with and without `--include-hidden-sheets`; chart-presence fixture asserts a `Placeholder` with no asset, and a warning explaining why no image exists.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green; whichever fallback paths ended up active (vs. native calamine API) recorded in `agent.md`.

---

## P2-S03 — Lowering policy wiring

- **Step ID:** P2-S03
- **Objective:** Route XLSX/XLS `Sheet`s through the `sheet_blocks()` policy P0-S05 already
  built for CSV — no new lowering logic, just correct reuse.
- **Depends On:** P2-S01, P2-S02, P0-S05
- **Files to Modify:** `src/extract/xlsx.rs`
- **Inputs:** blueprint.md P0-S05 (`sheet_blocks()`).
- **Outputs:** `extract_xlsx` calls `extract::sheet_blocks(&sheet, opts)` per sheet, exactly as `extract_csv` already does.
- **Exact Instructions:**
  1. After P2-S01/P2-S02 populate each `Sheet`, call the shared `sheet_blocks()` function from `extract/mod.rs` (built generically in P0-S05, not CSV-specific) for each sheet in workbook order, concatenating the results into `Document.blocks` with a `Block::Boundary(Boundary::Worksheet(name))` between sheets (already part of `sheet_blocks()`'s own output — no extra boundary logic needed here).
  2. Hidden-sheet skip (P2-S02 step 4) happens here: a hidden sheet with `include_hidden_sheets=false` contributes only the "N hidden sheets skipped" summary note, not its own `sheet_blocks()` output.
  3. `Document.workbook = Some(Workbook{title, sheets})` — the full structured data, always populated regardless of the lowering policy chosen for `Document.blocks` (so a future consumer reading `Document.workbook` directly, bypassing rendering entirely, still gets everything).
- **Relevant Skills:** none new.
- **Approved Dependencies:** none new.
- **Rationale:** This step is almost entirely wiring precisely because P0-S05 was built generically on purpose — the payoff of that upfront generality is that Phase 2 doesn't re-derive a second lowering policy.
- **Security Considerations:** none new.
- **Testing Requirements:** A 3-sheet XLSX produces the same table-vs-CSV-dump lowering behavior per sheet as the equivalent data would via the CSV extractor (small sheet → table, large/irregular → CSV dump) — same thresholds, same output shape, proving the shared function is genuinely shared and not silently diverged.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P2-S04 — ODS

- **Step ID:** P2-S04
- **Objective:** OpenDocument Spreadsheet through the same `calamine` reader (native ODS
  support), with ODF's own formula-syntax and value-type quirks handled explicitly.
- **Depends On:** P2-S01, P2-S03
- **Files to Create:** `src/extract/ods.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §4.2 AD-3.
- **Outputs:** `pub fn extract_ods(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>`.
- **Exact Instructions:**
  1. Same `calamine` open/iterate/lower pipeline as P2-S01/P2-S03 — `calamine` reads ODS through the same `Reader` trait, so this function is structurally the same shape as `extract_xlsx`, not a rewrite. Factor the shared open-and-lower loop into a private helper both call if the duplication becomes more than a few lines — don't force a shared function if the two formats' fidelity-gap handling (step 2 below) makes a forced merge awkward.
  2. **ODF formula syntax**: ODF formulas use a different textual convention than Excel's (`of:=SUM([.B2:.B10])` vs. Excel's `=SUM(B2:B10)`) — if calamine's formula accessor returns the raw ODF-syntax string for this format, store it **verbatim** in `Cell.formula` and push a one-time-per-document warning "ODF formula syntax preserved as-is; not translated to Excel syntax" (translating formula syntax between dialects is out of scope — verbatim preservation still satisfies AD-6's "never silently drop the formula" bar, it just doesn't also promise cross-dialect translation).
  3. Everything else (dates, merges via the ZIP-fallback path if needed, comments, hidden sheets, chart-presence-as-placeholder) follows P2-S01/P2-S02's logic; ODF's on-disk representation differs (`content.xml`'s `<table:table>` instead of `sheetN.xml`) only in the fallback paths that read the archive directly — if calamine's own API already exposes the needed data as it does for XLSX, no ODF-specific XML parsing is needed at all here.
- **Relevant Skills:** ODF, spreadsheet data modeling.
- **Approved Dependencies:** `calamine` (already added in P2-S01).
- **Rationale:** One crate covering XLSX/XLS/ODS uniformly (AD-3) means Phase 2's real work is fidelity edge cases, not three separate parsers.
- **Security Considerations:** none beyond P2-S01/S02's.
- **Testing Requirements:** ODS fixture with an ODF-syntax formula (assert verbatim preservation + the one-time warning); ODS fixture exercising the same merge/comment/hidden-sheet cases as P2-S02's XLSX tests, proving parity.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P2-S05 — Verification

- **Step ID:** P2-S05
- **Objective:** Phase-2 exit gate: formula preservation is the one property this project
  treats as a hard invariant (AD-6) — prove it end-to-end, not just per-unit.
- **Depends On:** P2-S01…S04
- **Files to Create:** `tests/phase2_integration.rs`
- **Files to Modify:** `src/extract/mod.rs` (replace `Xlsx/Xls/Ods` placeholder arms with real dispatch)
- **Inputs:** architecture.md AD-6, §13 (testing strategy), AD-14.
- **Outputs:** working end-to-end XLSX/XLS/ODS conversion through the CLI.
- **Exact Instructions:**
  1. Wire the three dispatcher arms.
  2. **The AD-6 test, run at the CLI level, not just against the CDM directly**: a fixture workbook with a cell showing a cached value (e.g. `220000`) computed from `=SUM(B2:B3)` — run it through the full pipeline (`convert_file`) and assert the *rendered output file* contains both the value `220000` and the formula text `=SUM(B2:B3)` somewhere in the Markdown (per P0-S07's `FormulaList` rendering rule). This is the test that would have caught a regression where formula preservation works in the extractor but gets dropped somewhere in rendering — test the seam, not just the unit.
  3. Full-suite regression: Phase 0 and Phase 1 tests still green after wiring in Phase 2's dispatcher arms.
  4. Legacy-XLS fixture through the full pipeline, asserting the AD-14 degraded-gracefully behavior (missing formula text doesn't break the run, display value is still correct, no panic).
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new.
- **Rationale:** AD-6 is called out as a project-level non-negotiable (§0.5) — it earns its own end-to-end test rather than trusting per-step unit coverage to have caught every seam.
- **Security Considerations:** none new.
- **Testing Requirements:** as above.
- **Acceptance Criteria:** All Phase 0/1/2 tests green; the AD-6 end-to-end assertion passes.
- **Definition of Done:** `cargo test` fully green; `agent.md` updated with the Phase-2 exit-criteria checklist, including which `calamine`-version-dependent code paths (native API vs. XML fallback) ended up active.

---

# Phase 3 — Presentations, Fully Detailed

---

## P3-S01 — Slide structure

- **Step ID:** P3-S01
- **Objective:** PPTX slide order from `<p:sldIdLst>` (never filename order — the single most
  important correctness rule for this format), titles, text, bullet lists.
- **Depends On:** P1-S01 (shared OOXML helpers), P0-S03
- **Files to Create:** `src/extract/pptx.rs`
- **Files to Modify:** `Cargo.toml` (no new dependency — reuses `zip`/`quick-xml`), `src/extract/mod.rs`
- **Inputs:** architecture.md §0.2, AD-4; P1-S01's `ooxml` helpers.
- **Outputs:** `pub fn extract_pptx(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>` (partial — rich content added in P3-S02).
- **Exact Instructions:**
  1. Open as ZIP; `docProps/core.xml` → `Meta`.
  2. `ppt/presentation.xml`: parse `<p:sldIdLst><p:sldId id="…" r:id="rIdN"/></p:sldIdLst>` to get the ordered list of relationship IDs. Resolve each via `ppt/_rels/presentation.xml.rels` (`ooxml::parse_rels`) to the actual slide part path (e.g. `ppt/slides/slide7.xml`). **This resolved order is the slide order — the numeral in a slide's filename is not reliable and must never be used for ordering** (a deck edited and reordered in PowerPoint keeps old filenames but updates `sldIdLst`).
  3. For each resolved slide path, in that order: if not the first slide, emit `Block::Boundary(Boundary::Slide{n, title: None})` (title filled in once found — see step 4) before the slide's content, not after.
  4. Parse the slide XML: find the title placeholder — a `<p:sp>` containing `<p:nvSpPr><p:nvPr><p:ph type="title"/>` or `type="ctrTitle"` — its text becomes both `Block::Heading{level:1}` at the top of the slide's blocks and the `title` field on the `Boundary::Slide` just emitted (mutate it in place once found, or buffer the slide's blocks and emit the boundary once the title is known — either implementation is fine, the requirement is that the rendered boundary carries the real title, not `None`, whenever one exists).
  5. Other `<p:sp>` shapes with text: for each `<a:p>` (paragraph) inside the shape's `<p:txBody>`: check `<a:pPr>` for `<a:buChar>` (bullet) or `<a:buAutoNum>` (numbered) — present → list item, grouped into `Block::List` across consecutive list paragraphs the same way P1-S02 groups DOCX list paragraphs, `<a:pPr>`'s `lvl` attribute (0-indexed) driving nesting; absent → `Block::Paragraph`. Runs parsed via `ooxml::parse_run_properties(ns='a')` (P1-S01) — this is the exact reuse P1-S01 was built for.
- **Relevant Skills:** OOXML PresentationML.
- **Approved Dependencies:** `zip`, `quick-xml` (already in the tree).
- **Rationale:** Slide-order-from-`sldIdLst` is the correctness bar an earlier draft's own test fixtures specifically targeted ("PPTX with deliberately out-of-sequence slide filenames") — carried forward here as the defining acceptance test for this step.
- **Security Considerations:** Same ZIP guards as DOCX/ODT/EPUB.
- **Testing Requirements:** A fixture deck where slide filenames are deliberately out of sequence relative to `sldIdLst` order — assert extracted order matches the list, not the filenames; title-and-plain-paragraph-only deck (no bullets/tables/images) extracts cleanly; 2-level nested bullet list preserves nesting.
- **Acceptance Criteria:** Tests green; slide order test specifically proves `sldIdLst`-based ordering, not filename-sorted ordering.
- **Definition of Done:** Tests green.

---

## P3-S02 — Rich content: tables, images, charts, notes, boundaries

- **Step ID:** P3-S02
- **Objective:** Complete the PPTX extractor: tables, per-slide images (per-slide rels, not
  the package-level rels), chart placeholders, optional speaker notes.
- **Depends On:** P3-S01
- **Files to Modify:** `src/extract/pptx.rs`
- **Inputs:** architecture.md §0.5, AD-9 (chart/image honest-stub principle).
- **Outputs:** full `extract_pptx`.
- **Exact Instructions:**
  1. **Tables**: `<a:tbl>` inside a `<p:graphicFrame>` → `Block::Table` via `ooxml::parse_table_cell(ns='a')` (P1-S01) — the same `gridSpan`/`vMerge` two-pass algorithm as DOCX, since DrawingML tables use the same merge attribute shapes.
  2. **Images**: `<p:pic>` → `<p:blipFill><a:blip r:embed="rIdN"/>` — resolve `rIdN` through the **per-slide** relationships file `ppt/slides/_rels/slideN.xml.rels` (**not** the package-level `presentation.xml.rels` used for slide ordering — a common bug is reusing the wrong rels file here) → image path under `ppt/media/` → read bytes → register asset → `Block::Image`.
  3. **Charts**: `<p:graphicFrame>` containing a `<c:chart r:id="rIdN">` reference (resolved, again, via the slide's own rels) → this points at a separate chart part (`ppt/charts/chartN.xml`), which — like XLSX embedded charts (P2-S02) — is Excel/PowerPoint-rendered client-side, not shipped as a pre-rendered image. Emit `Block::Placeholder{kind: Chart, label: <c:title text if resolvable>, asset: None}`, consistent with AD-9's honest-stub principle and P2-S02's identical treatment of spreadsheet charts — don't attempt image extraction PPTX charts don't reliably have either.
  4. **Speaker notes**: `opts.include_notes` (default `false`) → for slide N, check whether `ppt/notesSlides/notesSlideN.xml` exists (resolved via the slide's own rels, `notesSlide` relationship type — not assumed from the slide's numeral, same ordering caution as everywhere else in this format) → parse its text body → append `Block::Quote(vec![Block::Paragraph(vec![Inline::Styled{style: italic, children: [Text("Speaker notes:")]}])])` followed by the notes content, at the end of that slide's blocks. When `include_notes=false` (default), notes parts are never read at all — not read-then-discarded, simply skipped, so a deck with heavy notes doesn't cost parse time when the flag is off.
- **Relevant Skills:** OOXML PresentationML.
- **Approved Dependencies:** none new.
- **Rationale:** The per-slide-vs-package-level rels distinction (images, charts, notes all resolve through `slideN.xml.rels`, only slide *ordering* uses the package-level rels) is the concrete detail an implementation most easily gets wrong by pattern-matching to DOCX's simpler single-rels-file model — spelled out explicitly here so it isn't.
- **Security Considerations:** Same ZIP guards as elsewhere.
- **Testing Requirements:** Deck with a merged-cell table; deck with an image (assert it resolves via the *slide's* rels, via a fixture where the package-level rels intentionally doesn't contain the image's rId — proving the per-slide path is actually being used, not accidentally falling back); deck with a chart (assert `Placeholder`, no asset, no crash); deck with speaker notes, tested both with and without `--include-notes`.
- **Acceptance Criteria:** Tests green.
- **Definition of Done:** Tests green.

---

## P3-S03 — Verification

- **Step ID:** P3-S03
- **Objective:** Phase-3 exit gate: full multi-slide deck with every content type together,
  through the CLI.
- **Depends On:** P3-S01, P3-S02
- **Files to Create:** `tests/phase3_integration.rs`
- **Files to Modify:** `src/extract/mod.rs` (replace `Pptx` placeholder arm with real dispatch)
- **Inputs:** architecture.md §13.
- **Outputs:** working end-to-end PPTX conversion through the CLI.
- **Exact Instructions:**
  1. Wire the dispatcher arm.
  2. Multi-slide fixture combining a title slide, a bulleted-content slide, a table slide, an image slide, a chart slide, and a notes slide — convert end-to-end and assert: slide count matches, `Boundary::Slide` titles are correct and in `sldIdLst` order, the table's merge survived, the image asset was written, the chart is a placeholder with no crash, notes appear only with `--include-notes`.
  3. Full-suite regression: Phases 0–2 still green.
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new.
- **Rationale:** Same reasoning as P1-S06/P2-S05 — a phase's exit gate is proven at the CLI seam, not just per-extractor.
- **Security Considerations:** none new.
- **Testing Requirements:** as above.
- **Acceptance Criteria:** All Phase 0–3 tests green.
- **Definition of Done:** `cargo test` fully green; `agent.md` updated with the Phase-3 exit-criteria checklist.

---

# Phase 4 — PDF, Fully Detailed

---

## Stage 4.A — Default build: correct text extraction

### P4-S01 — `pdf-extract`/`lopdf` integration

- **Step ID:** P4-S01
- **Objective:** Default-build baseline: open the PDF, detect encryption, decode text
  correctly via `pdf-extract`, and independently detect which pages are scanned (no
  extractable text) via `lopdf`'s object model.
- **Depends On:** P0-S01
- **Files to Create:** `src/extract/pdf.rs`
- **Files to Modify:** `Cargo.toml` (add `lopdf`, `pdf-extract`), `src/extract/mod.rs`
- **Inputs:** architecture.md §4.2 AD-1, AD-11.
- **Outputs:** `pub fn extract_pdf(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>` — the default-build baseline; Stage 4.B extends behavior under the `pdf-layout` feature.
- **Exact Instructions:**
  1. Open via `lopdf::Document::load_mem(bytes)` (verify the exact loader name against the resolved version). A load error consistent with a missing/corrupt xref or trailer on an otherwise `%PDF`-signatured file → `ConvertError::Corrupt`. Check for encryption via the trailer's `/Encrypt` entry (or the resolved version's own encryption-detection accessor) → `ConvertError::Encrypted` immediately, before any further parsing — never attempt to guess a password or parse an encrypted stream.
  2. Page enumeration: `doc.get_pages()` → an ordered page-number-to-object-ID map (already correctly ordered by `lopdf` via the page tree — don't re-derive order from raw object IDs, which have no defined relationship to page order in the PDF spec).
  3. Whole-document text decode via `pdf_extract::extract_text_from_mem(bytes)` — this is the correctness-critical path (CMap/CFF/Type1 font-encoding handled internally by `pdf-extract`, not reimplemented here). Wrap this specific call in `std::panic::catch_unwind`: `pdf-extract`'s handling of pathological/malformed embedded fonts has historically been inconsistent across versions, and architecture.md §9's "no input may crash the process" rule applies regardless of which dependency the crash would originate in. A caught panic or a returned `Err` both degrade to `ConvertError::Corrupt("PDF text could not be decoded")`.
  4. Per-page scanned-page detection, independent of step 3: for each page from step 2, get its decompressed content stream via `lopdf` (verify the exact accessor — `Document::get_page_content` or equivalent on the resolved version) and scan the raw bytes for text-showing operators (`Tj`, `TJ`, `'`, `"`) via a simple byte-level scan (presence detection only, not full tokenization). A page with zero such operators, whose content stream is non-trivial in size (i.e., not simply a blank page), is flagged `NeedsOcr`.
  5. `Meta.page_count = Some(doc.get_pages().len())`.
- **Relevant Skills:** PDF internals.
- **Approved Dependencies:** `lopdf`, `pdf-extract`.
- **Rationale:** AD-1 — decoupling correct text decoding (`pdf-extract`, the correctness-critical path) from scanned-page detection (a much simpler `lopdf`-level presence check) means a limitation in one doesn't block the other.
- **Security Considerations:** `catch_unwind` boundary around the `pdf-extract` call; `--max-size` (P0-S08) catches pathological file sizes before any PDF parsing starts; `lopdf`'s own internal stream-size handling applies beneath that.
- **Testing Requirements:** encrypted-PDF fixture → `Encrypted`, no partial output; ordinary text PDF; fully scanned (image-only) PDF → every page flagged `NeedsOcr`; mixed PDF → correct per-page split; truncated-xref fixture → `Corrupt`, no panic; at least one fixture specifically constructed to exercise the `catch_unwind` path (a font-table edge case), not just present in code untested.
- **Acceptance Criteria:** tests green; the `catch_unwind` path is demonstrably exercised, not just theoretically reachable.
- **Definition of Done:** tests green; resolved `lopdf`/`pdf-extract` versions and the exact accessor names used recorded in `agent.md`.

### P4-S02 — Paragraph/heading reconstruction (default path)

- **Step ID:** P4-S02
- **Objective:** Turn `pdf-extract`'s flat decoded text into CDM blocks — paragraphs from
  blank-line gaps, a best-effort heading heuristic — with an honest, unconditional warning
  about the resulting reading-order approximation.
- **Depends On:** P4-S01
- **Files to Modify:** `src/extract/pdf.rs`
- **Inputs:** architecture.md AD-1.
- **Outputs:** `extract_pdf` populates `Document.blocks` for the default build.
- **Exact Instructions:**
  1. Split `pdf-extract`'s output into per-page text on the form-feed character (`\x0c`), which `pdf-extract` conventionally inserts between pages; if none appear, treat the whole output as one page.
  2. Within a page: split on blank-line gaps → candidate paragraphs; within a paragraph, collapse single internal newlines to spaces (PDF extraction line-wraps at visual page width, not sentence structure) **unless** a line looks like a list item (leading `•`/`*`/`-`/`N.`/`N)`) or looks like preserved tabular alignment (runs of ≥2 spaces inside the line) — in either case, keep line breaks as-is.
  3. **Heading heuristic** (explicitly weaker than Stage 4.B's font-metric approach — say so in a code comment): a line qualifies as a heading candidate if it's short (≤80 chars), stands alone (blank lines both before and after), has no trailing sentence punctuation, and isn't a list item by step 2's pattern. All heading candidates become `Block::Heading{level: 2}` (no font-size data available to rank tiers) except a clear first-line title (if it matches `Meta.title`), which becomes level 1.
  4. `NeedsOcr` pages (P4-S01) contribute `Block::Placeholder{kind: ScannedPage, label: Some("page N")}` instead of text; more than 5 such pages collapse into one summarized warning rather than one line per page.
  5. Always push, unconditionally, on every successful default-build PDF conversion: "reading order approximated from PDF text stream order; multi-column or complex layouts may read out of order. Build with --features pdf-layout for full layout reconstruction." This is a standing caveat, not a conditional one — its accuracy can't be self-diagnosed from flat text alone.
- **Relevant Skills:** text heuristics.
- **Approved Dependencies:** none new.
- **Rationale:** AD-1's default path trades structure for correct decoding; the unconditional warning is what keeps that trade explicit rather than a silent quality regression (architecture.md §0.1 extends "never silently discard information" to never silently discarding confidence about output quality).
- **Security Considerations:** none new.
- **Testing Requirements:** single-column prose → correct paragraphing; a bullet-list page → line breaks preserved, not collapsed; scanned-page fixture → placeholders + summarized warning; the reading-order-approximated warning present on every successful conversion, unconditionally, asserted directly (not just "present in some cases").
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S03 — Default-path PDF tests

- **Step ID:** P4-S03
- **Objective:** Stage 4.A exit gate, independent of Stage 4.B.
- **Depends On:** P4-S01, P4-S02
- **Files to Create:** `tests/phase4a_integration.rs`
- **Files to Modify:** `src/extract/mod.rs` (wire `Format::Pdf` to `extract_pdf`)
- **Inputs:** architecture.md §13.
- **Outputs:** working default-build PDF conversion through the CLI.
- **Exact Instructions:**
  1. Wire the dispatcher.
  2. End-to-end: single-column prose → md/txt; scanned PDF → placeholders + warning, exit code **0** (an honestly-reported mostly-placeholder conversion is a success, not a failure); encrypted PDF → clean `Encrypted` error, non-zero exit code, no partial output file written.
  3. A synthetic two-column fixture specifically to exercise the approximate-reading-order warning: assert it fires, and capture the actual (expected-to-be-imperfect) interleaved output in a test comment as a documented before/after baseline for P4-S12 to compare against once Stage 4.B exists.
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new.
- **Rationale:** keeps the default build's own quality bar independently verified before Stage 4.B's added complexity enters the picture.
- **Security Considerations:** none new.
- **Testing Requirements:** as above.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green; `agent.md` updated with Stage 4.A exit status and the two-column baseline fixture flagged for reuse.

---

## Stage 4.B — `pdf-layout` feature: full structural fidelity

### P4-S04 — `pdfium-render` integration (feature-gated)

- **Step ID:** P4-S04
- **Objective:** Stand up the opt-in build target: dependency, native-library loading,
  per-character bounding-box + font extraction — the foundation every later Stage-4.B step
  builds on.
- **Depends On:** P4-S01 (shares the `Format::Pdf` entry point; feature-selected alternate path)
- **Files to Create:** `src/extract/pdf_layout.rs` (cfg-gated module)
- **Files to Modify:** `Cargo.toml` (`pdfium-render` as `optional = true`; replace P0-S01's placeholder `pdf-layout = []` with `pdf-layout = ["dep:pdfium-render"]`), `src/extract/pdf.rs` (dispatch when the feature is compiled in), `src/config.rs` (`pdfium_lib_path: Option<PathBuf>`, feature-gated field)
- **Inputs:** architecture.md §4.2 AD-1, §10 (native-library sourcing).
- **Outputs:** `pub fn extract_pdf_layout(bytes: &[u8], ctx: &mut ExtractCtx) -> Result<Document>` (`cfg(feature = "pdf-layout")` only); `pub struct Glyph { ch: char, bbox: (f32,f32,f32,f32), font_size: f32, font_name: String }`.
- **Exact Instructions:**
  1. Update `Cargo.toml`'s feature stanza in place (don't leave both the old empty one and a new one).
  2. Native-library sourcing — three modes, documented in the README (P6-S05 writes the user-facing text; this step accepts the config surface): (a) system-installed PDFium via `PDFIUM_DYNAMIC_LIB_PATH`; (b) a prebuilt binary vendored alongside the `pdf-layout` release artifact (P6-S05's job); (c) a path passed explicitly via `--pdfium-lib-path` / `pdfium_lib_path`. Default: the platform's conventional search path.
  3. Bind via the resolved `pdfium-render` version's binding call (verify exact API against docs.rs — this has changed shape across versions). Binding failure (library not found or wrong ABI) is an **environment** problem, not a document problem: a new error path, `ConvertError::Config("pdfium library not found: <detail>. Set PDFIUM_DYNAMIC_LIB_PATH or --pdfium-lib-path, or install libpdfium.")` — this must be a clean, actionable message; the most common failure mode of a dynamic-native-library feature is a confusing low-level linker/ABI error if this isn't handled explicitly at the top.
  4. Load the document; PDFium itself refuses password-protected PDFs without a password — surface that as `ConvertError::Encrypted`, the same variant Stage 4.A uses, so callers never need to know which path produced the error.
  5. Per-page, per-character extraction via the resolved version's text-object/character iterator, collecting `Glyph{ch, bbox, font_size, font_name}` with `bbox` already in page-coordinate space (PDFium computes this directly — no manual text-matrix math needed, exactly the capability AD-1 identified as unavailable elsewhere). Buffer as `Vec<Vec<Glyph>>` (pages → glyphs in the document's internal object order — **not yet** reading order; that's P4-S05).
- **Relevant Skills:** PDF internals, FFI.
- **Approved Dependencies:** `pdfium-render` (feature-gated only, never a default-build dependency — AD-1).
- **Rationale:** isolating the FFI boundary into its own cfg-gated module means a default build never touches this code, compiles it out entirely, and never links `libloading`.
- **Security Considerations:** the loaded library path is user-configured or release-vendored — never fetched over the network at runtime (architecture.md §9); binding failure produces a config error, never a panic or unhandled FFI fault.
- **Testing Requirements:** (feature-gated — CI needs a `--features pdf-layout` job) successful bind + glyph extraction on a simple fixture, spot-checked glyph count and bbox; missing-library path → `ConvertError::Config`, actionable message, no panic; encrypted PDF → `ConvertError::Encrypted`, same variant as the default path.
- **Acceptance Criteria:** tests green under the feature; the default build has zero `pdfium-render` in its resolved dependency tree (verify via `cargo tree` without the feature).
- **Definition of Done:** tests green; resolved `pdfium-render` version, its binding API shape, and the three sourcing modes recorded in `agent.md` for P6-S05.

### P4-S05 — Line and column clustering

- **Step ID:** P4-S05
- **Objective:** Turn the unordered glyph bag into reading-order text: line clustering,
  multi-column detection, correct column-major reading order.
- **Depends On:** P4-S04
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** algorithm carried forward from the earlier draft's PDF-extractor phase, verified sound.
- **Outputs:** `fn cluster_lines(glyphs: &[Glyph]) -> Vec<Line>` (`Line{glyphs, y, x_range}`); `fn order_reading(lines: Vec<Line>, page_width: f32) -> Vec<Line>`.
- **Exact Instructions:**
  1. Sort glyphs by `y` descending (PDF's origin is bottom-left, so larger `y` = higher on the page), group into lines where consecutive glyph `y`-centers fall within `0.3 × median glyph height` **computed per-page** (not a fixed constant — page/font scale varies widely between a slide deck and a printed book). Within a line, sort by `x` ascending.
  2. Column detection: build a horizontal-gap histogram across the page's lines. A gap that's **persistent** (similar x-position across ≥40% of lines) and wider than `2 × median inter-word gap` marks a column boundary. No persistent gap → single column, order by `y` alone. One or more found → split lines at the boundary x-positions, group by column band, order **all of column 1 top-to-bottom, then all of column 2**, never interleaving by y-position across columns.
  3. Column detection runs **per page** — a title page followed by two-column body pages is handled correctly without a document-wide assumption.
- **Relevant Skills:** PDF layout/typography.
- **Approved Dependencies:** none new.
- **Rationale:** this is the specific capability the default build structurally cannot provide (AD-1) — the reason `pdf-layout` exists, so it gets the most careful specification in this phase.
- **Security Considerations:** bounded by the same page/glyph limits as P4-S04.
- **Testing Requirements:** single-column fixture → simple top-to-bottom order; the two-column academic-paper fixture flagged back in P4-S03 → column 1 fully precedes column 2, no interleaving; a full-width heading above two columns → treated as its own single-column line, not incorrectly split at the column boundary.
- **Acceptance Criteria:** tests green; the two-column fixture's output is demonstrably better than Stage 4.A's output on the same file.
- **Definition of Done:** tests green.

### P4-S06 — Heading detection via font size

- **Step ID:** P4-S06
- **Objective:** Rank headings by actual font metrics — the capability Stage 4.A's text-only
  heuristic structurally lacks.
- **Depends On:** P4-S05
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** architecture.md AD-1.
- **Outputs:** line-level `is_heading: bool` + `level: usize` annotation feeding block assembly.
- **Exact Instructions:**
  1. Compute the modal font size **by total glyph count** (not line count — a few long body-text lines should outweigh many short captions) as `body_size`, per page.
  2. Heading candidate: font size `≥ 1.15 × body_size`, visually isolated (blank gap `≥ 1.5 × line height` above and below, from P4-S05's clustering), and short (≤120 characters — excludes a large-font pull-quote spanning several lines).
  3. Level assignment is **document-global**, not per-page: collect distinct heading-candidate font sizes across the whole document, sort descending, map largest → level 1, next → level 2, etc., capped at level 6 (matching the CDM's range from P0-S01).
- **Relevant Skills:** PDF layout/typography.
- **Approved Dependencies:** none new.
- **Rationale:** this is the exact quality gap AD-1 named as the reason `pdf-layout` exists.
- **Security Considerations:** none new.
- **Testing Requirements:** 3-tier heading fixture → correct level assignment; large-font pull-quote → correctly excluded; uniform-body-font document → zero false positives.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S07 — Header/footer stripping

- **Step ID:** P4-S07
- **Objective:** Remove running headers/footers/page numbers via cross-page repetition at a
  normalized position, page-number-tolerant.
- **Depends On:** P4-S05
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** algorithm carried forward from the earlier draft, verified sound.
- **Outputs:** lines flagged `is_header_footer: bool`, excluded from `Document.blocks` unless `--no-strip-headers-footers`.
- **Exact Instructions:**
  1. Per line, compute a normalized key: `(round(y / page_height, 2), digits_stripped(text))` — stripping digits from the text is what makes "Page 4" and "Page 5" hash identically, catching page-number variation rather than only exact repeats.
  2. A key appearing at the same normalized position on `≥ 60%` of pages → every matching line, on every page, is flagged and excluded from default block assembly.
  3. Gated by `--no-strip-headers-footers` (default: strip). A single-page document (60%-of-pages is undefined) → no-op, guarded against a divide-by-zero, not a crash.
- **Relevant Skills:** PDF layout/typography.
- **Approved Dependencies:** none new.
- **Rationale:** cross-page repetition at a normalized position is a reliable signal for this specific, well-understood problem — no more elaborate mechanism is needed.
- **Security Considerations:** none new.
- **Testing Requirements:** repeated footer with a changing page number → stripped on every page despite differing text; `--no-strip-headers-footers` → nothing stripped; single-page document → no-op, no crash.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S08 — Table detection: ruled and unruled

- **Step ID:** P4-S08
- **Objective:** Recover tables from page geometry — high-confidence ruled (drawn grid) and
  lower-confidence unruled (whitespace-aligned) variants, using the CDM's existing
  `Table.approximate` flag to distinguish them.
- **Depends On:** P4-S05
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** P0-S01 (`Table.approximate`, already defined for exactly this purpose).
- **Outputs:** `Block::Table` entries; absorbed lines removed from normal paragraph flow.
- **Exact Instructions:**
  1. **Ruled:** iterate the page's vector/path objects (verify exact `pdfium-render` enum name against the resolved version) for axis-aligned line segments; build distinct horizontal/vertical line positions; ≥2 of each forming a closed rectangular grid → candidate table. Assign each text line (P4-S05) to the grid cell its bbox center falls within; cells → `TableCell`s; topmost grid row → header if it's visually distinct (bold/isolated). `Table.approximate = false`.
  2. **Unruled:** in a region with no detected grid, ≥3 consecutive lines each starting text at the same 2–4 x-positions (tolerance-matched, spaced wider than a normal word gap) → whitespace-aligned table; split each line at those positions into cells. `Table.approximate = true` (the CDM field built in P0-S01 specifically for this confidence distinction).
  3. Lines absorbed into either kind of detected table are removed from the page's line stream before later steps (P4-S09 onward) process what's left.
- **Relevant Skills:** PDF layout/typography.
- **Approved Dependencies:** none new.
- **Rationale:** the ruled/unruled confidence split matches exactly what `Table.approximate` was defined for — this step is correctly using ground P0-S01 already prepared, not inventing new CDM surface.
- **Security Considerations:** bounded by the same page/glyph limits as the rest of this phase.
- **Testing Requirements:** ruled-table fixture → correct rows/cols, `approximate=false`; unruled fixture → correct cell split, `approximate=true`; a page with neither → zero false-positive tables.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S09 — Image extraction + caption association

- **Step ID:** P4-S09
- **Objective:** Pull embedded raster images via PDFium's page-object API and associate
  nearby text as captions, positioned correctly in reading order.
- **Depends On:** P4-S04, P4-S05
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** —
- **Outputs:** `Block::Image` entries at correct reading-order positions.
- **Exact Instructions:**
  1. Iterate `page.objects()` for image-type objects; prefer the resolved `pdfium-render` version's direct PNG-export path if it has one (check docs.rs first) over writing a bespoke pixel-buffer-to-PNG encoder — only fall back to hand-encoding if the resolved version truly only returns raw pixels. Register extracted bytes via `AssetManager`; get the bbox from the object's own position data.
  2. Caption association: the nearest non-heading text line within `2 × line height` below the image's bbox becomes its caption (or above, if nothing qualifies below — some layouts caption above); the consumed line is removed from the normal paragraph stream, same as table cells in P4-S08.
  3. Insert `Block::Image` at its correct reading-order position (using P4-S05's column-aware ordering), not appended at page end.
- **Relevant Skills:** PDF layout/typography.
- **Approved Dependencies:** none new.
- **Rationale:** y-position-based caption association is a well-scoped heuristic sufficient for the common case — raw PDF has no structured caption-frame concept the way DOCX/PPTX do, so nothing more elaborate is warranted.
- **Security Considerations:** per-image size guard, consistent with the project-wide `--max-size` philosophy.
- **Testing Requirements:** image with a caption directly below → correctly associated; image with no nearby text → `caption: None`, no crash; image inside a two-column layout → correct column-aware placement, not appended out of order.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S10 — List and code-block reconstruction

- **Step ID:** P4-S10
- **Objective:** Recover bulleted/numbered lists and monospace code blocks from geometry —
  PDF has no native concept of either.
- **Depends On:** P4-S05, P4-S08 (run after table/image line-removal)
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** —
- **Outputs:** `Block::List` / `Block::CodeBlock` entries.
- **Exact Instructions:**
  1. **Lists:** a line starting with a bullet glyph (`•‣▪-*`) or `N.`/`N)`/`(N)` at a consistent x-offset shared with ≥1 nearby line → grouped into `Block::List`; nesting level from relative x-offset steps (each additional consistent indent = one deeper `ListItem.level`), wrapped continuation lines at a deeper indent stay attached to the preceding item rather than starting a new one.
  2. **Code blocks:** ≥3 consecutive lines sharing the same monospace-family font name (`Glyph.font_name` containing `Mono`/`Courier`/`Consolas`/`Menlo`, case-insensitive — a naming-convention heuristic, not a metrics measurement) at consistent indent → `Block::CodeBlock{language: None}` (language isn't guessable from font alone — leave unset) + an info-level warning "code block detected via monospace font heuristic; language unknown."
  3. Both run **after** P4-S08/S09's line-removal, so a monospace table cell isn't double-counted as a code block.
- **Relevant Skills:** PDF layout/typography.
- **Approved Dependencies:** none new.
- **Rationale:** both signals are the same category of geometry-driven heuristic as the rest of Stage 4.B — consistent detection philosophy across block types rather than a different approach per type.
- **Security Considerations:** none new.
- **Testing Requirements:** 2-level nested bullet list → correct grouping/nesting; numbered list → `ordered=true`; 5-line monospace passage → `CodeBlock` + language-unknown warning; a monospace table cell (post-detection) → not double-counted.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S11 — Math font/Unicode-range detection

- **Step ID:** P4-S11
- **Objective:** Recover math content as literal Unicode with an honest warning — never a
  full LaTeX reconstruction, since raw PDF exposes no equation structure to reconstruct from
  (unlike DOCX's OMML).
- **Depends On:** P4-S05
- **Files to Modify:** `src/extract/pdf_layout.rs`
- **Inputs:** AD-9 (honest-stub principle, applied here to a new case).
- **Outputs:** `Inline::MathInline` / `Block::MathBlock` (literal Unicode) where math is detected.
- **Exact Instructions:**
  1. A glyph is math-flagged if its `font_name` matches a known math-font family (`Cambria Math`, `STIX`, `Symbol`, `Euclid Math`, case-insensitive), **or** its codepoint falls in `U+2200`–`U+22FF` (Mathematical Operators) or `U+1D400`–`U+1D7FF` (Mathematical Alphanumeric Symbols). Arrow glyphs (`U+2190`–`U+21FF`) only count when adjacent to other math-flagged glyphs — arrows alone are too common in ordinary prose/diagrams to flag independently.
  2. A run of ≥1 consecutive math-flagged glyphs within a line → `Inline::MathInline(text)` using the literal decoded characters, no LaTeX conversion attempted. An entirely math-flagged line/paragraph → `Block::MathBlock` instead.
  3. Push an info-level warning **once per document**, on first occurrence: "mathematical content rendered as literal Unicode characters, not LaTeX; PDF's raw glyph stream does not expose equation structure the way DOCX's native math markup does."
- **Relevant Skills:** PDF layout/typography, Unicode.
- **Approved Dependencies:** none new.
- **Rationale:** sets expectations correctly rather than silently dropping math content or overclaiming LaTeX-quality reconstruction this input format cannot structurally support.
- **Security Considerations:** none new.
- **Testing Requirements:** Cambria-Math-tagged glyph fixture → detected, wrapped, warning fires exactly once across multiple equations; ordinary arrow glyphs in prose → not false-positively flagged.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

### P4-S12 — `pdf-layout` verification

- **Step ID:** P4-S12
- **Objective:** Stage 4.B exit gate — the whole pipeline (S04→S11) together on realistic
  fixtures, plus the direct before/after comparison against Stage 4.A promised in P4-S03.
- **Depends On:** P4-S04 … P4-S11
- **Files to Create:** `tests/phase4b_integration.rs` (feature-gated: compiled/run only under `--features pdf-layout`)
- **Files to Modify:** `src/extract/pdf.rs` (dispatch to `pdf_layout` when the feature is present; Stage 4.A remains the path when it isn't)
- **Inputs:** architecture.md §13.
- **Outputs:** working `--features pdf-layout` PDF conversion, full fidelity, through the CLI.
- **Exact Instructions:**
  1. Wire dispatch; both paths share the same `Result<Document>` contract, so nothing downstream needs to know which ran — record which one did in the Report (P0-S08) as `pdf_path: "default" | "layout"` for transparency.
  2. Five fixtures exercising Stage 4.B's capabilities **in combination**, not isolation: two-column academic layout; ruled-table PDF; unruled-table PDF; embedded-image-with-caption PDF; math-font-heavy PDF.
  3. **The direct comparison**: run the two-column fixture through both builds in the same test; assert `pdf-layout`'s column ordering is correct, and document (as a comment, not an assertion-of-failure) that the default build's output on the identical fixture is expected to interleave columns — the concrete proof AD-1's two-tier split delivers a real quality difference, not just two code paths producing similar output.
  4. Full-suite regression: Phases 0–3 and Stage 4.A both still pass, feature on or off.
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new.
- **Rationale:** this is the test that actually validates AD-1's core justification — without it, the two-tier design is an assertion in a document, not a demonstrated property of the software.
- **Security Considerations:** none new.
- **Testing Requirements:** as above.
- **Acceptance Criteria:** all five fixtures pass under the feature; the before/after comparison shows a real, documented difference; full regression green both with and without the feature.
- **Definition of Done:** `cargo test --features pdf-layout` and plain `cargo test` both fully green; `agent.md` updated with the Phase-4 exit-criteria checklist for both build profiles.

---

# Phase 5 — Images & OCR, Fully Detailed

---

## P5-S01 — Image header parser + standalone image path

- **Step ID:** P5-S01
- **Objective:** Read dimensions (only dimensions — no decoding/re-encoding) for PNG/JPEG/
  GIF/BMP/WebP/TIFF/JP2, and wire a standalone image file into the CDM as a single-image
  document.
- **Depends On:** P0-S01, P0-S02, P0-S03
- **Files to Create:** `src/extract/image.rs`
- **Files to Modify:** `src/extract/mod.rs`
- **Inputs:** architecture.md §0.2, §4.2 (header-only reads need no decoding crate).
- **Outputs:** `pub fn extract_image(bytes: &[u8], meta: Meta, kind: ImageKind, ctx: &mut ExtractCtx) -> Result<Document>`; `pub fn read_dimensions(bytes: &[u8], kind: ImageKind) -> Option<(u32,u32)>`.
- **Exact Instructions:**
  1. Each format's dimensions live at a small, well-known fixed-offset byte layout — read only that, never decode pixel data:
     - **PNG:** IHDR chunk immediately follows the 8-byte signature + 4-byte length + 4-byte "IHDR" tag; width then height, 4-byte big-endian each.
     - **JPEG:** scan marker segments from byte 2 for an SOF0–SOF3 marker (`0xFFC0`–`0xFFC3`), skipping other segments by their declared length; height then width, 2-byte BE, at a fixed offset within the SOF segment.
     - **GIF:** bytes 6–10 (after the 6-byte signature), width then height, 2-byte **little-endian**.
     - **BMP:** bytes 18–26 of the DIB header (after the 14-byte file header), width then height, 4-byte LE signed integers (a negative height means top-down storage — take the absolute value for the dimension; note the sign in a comment, it would matter if pixel data were ever read, which it isn't here).
     - **WebP:** implement the VP8X chunk case first (explicit 24-bit LE width-1/height-1 fields — the common modern case); VP8/VP8L cases best-effort. Any parse failure across all three → `None`, not an error.
     - **TIFF:** byte-order marker (`II`=LE, `MM`=BE) at offset 0; follow the IFD offset (bytes 4–8) to the first directory, scan tag entries for tag `256` (width) and `257` (height).
     - **JP2:** dimensions live in the `ihdr` box within the `jp2h` box — walk the box structure (4-byte length + 4-byte type per box, recursive) to find it, then read height/width (4-byte BE each, in that order).
  2. Any parse failure at any point → `None`, never an error — dimensions are a report nicety, not load-bearing for the conversion itself; the image is still registered and a valid `Document` still produced.
  3. `extract_image`: registers the whole file as one `Asset`, builds a `Document` with one `Block::Image{asset, alt: None, caption: None}`; recovered dimensions go into the Report (P0-S08), not into a new CDM `Meta` field — `Meta` (P0-S01) doesn't have one and doesn't need one added just for this.
- **Relevant Skills:** image header formats.
- **Approved Dependencies:** none — reading a handful of known fixed-offset fields doesn't need an image-decoding crate; pulling one in only for dimensions is exactly the unjustified-dependency case architecture.md §4.2 exists to prevent.
- **Rationale:** as above.
- **Security Considerations:** every header read is bounded (never past a fixed small prefix); any length/offset field taken from the file itself is range-checked against the actual buffer before being used to index into it — a malformed TIFF IFD offset pointing past EOF degrades to `None`, never an out-of-bounds panic.
- **Testing Requirements:** one real fixture per format with known dimensions, exact width/height asserted; a truncated file per format → `None`, no panic; an adversarially malformed offset field pointing past the buffer end (TIFF and JP2 specifically, since those chase offsets the most) → `None`, no panic, no out-of-bounds read.
- **Acceptance Criteria:** tests green; zero panics across the adversarial battery.
- **Definition of Done:** tests green.

---

## P5-S02 — External OCR adapter

- **Step ID:** P5-S02
- **Objective:** off/auto/force OCR via a local subprocess engine (AD-10) — never a linked
  library, never a network call.
- **Depends On:** P0-S01
- **Files to Create:** `src/ocr.rs`
- **Files to Modify:** `src/lib.rs`
- **Inputs:** architecture.md AD-10, §9.
- **Outputs:** `pub fn run_ocr(image_bytes: &[u8], opts: &Options) -> Result<OcrResult>`; `pub struct OcrResult { text: String, confidence: Option<f32> }`.
- **Exact Instructions:**
  1. `Options.ocr_command` (default `"tesseract"`, already declared in P0-S08) is resolved via the OS's normal `PATH` lookup by `std::process::Command` — never shell-interpreted.
  2. Protocol: write `image_bytes` to a uniquely-named temp file (`std::env::temp_dir()` + `std::process::id()` + a counter — no `tempfile` crate needed, consistent with P0-S08's own approach) with the correct extension for its format; invoke `Command::new(&opts.ocr_command).arg(&input_path).arg(&output_stem).arg("-l").arg(&opts.ocr_lang).output()` — **argv array, never a shell string**; this literal API choice *is* architecture.md §9's command-injection mitigation, not an escaping scheme layered on top of a shell call. Tesseract's own CLI convention (`tesseract input.png output_stem -l eng`) is the default `ocr_command` contract — a user pointing `--ocr-command` elsewhere is responsible for that engine accepting the same argv shape, documented plainly rather than special-cased per third-party engine.
  3. Read `{output_stem}.txt` (Tesseract's own output convention) after a successful exit; non-zero exit → `ConvertError::Ocr` with captured stderr, truncated for the error message.
  4. Confidence: **out of scope for this default invocation** — Tesseract's per-word confidence needs its `tsv` output mode, a different invocation shape than the plain-text default used here. `OcrResult.confidence` stays `None` rather than half-implementing parsing against an output format this step doesn't request.
  5. Temp files (input and output) are cleaned up unconditionally, including on the error path — pick one consistent mechanism (a drop-guard or an explicit cleanup block on every return path) and apply it everywhere, don't leak on an early error return.
  6. `OcrMode::Off` → `run_ocr` never called. `Auto` → called only for signaled content (scanned-PDF pages, P4-S01/S02's `NeedsOcr`; a standalone image only under an explicit request, since a photo isn't presumed to contain text — document this nuance in the CLI help text, since for a standalone image `Auto` and `Force` are functionally equivalent absent a PDF-page-style signal to gate on). `Force` → always attempted.
  7. Engine absent (`Command::new` spawn fails with `NotFound`) → `Auto` degrades to a warning ("OCR requested but no engine found at '<cmd>'; scanned content preserved as Placeholder only"), conversion still succeeds; `Force` → fatal `ConvertError::Ocr` (the user explicitly asked and it wasn't honored — silently downgrading would hide that).
- **Relevant Skills:** process spawning, temp-file handling.
- **Approved Dependencies:** none — deliberately no `tempfile` crate, no OCR-binding crate (AD-10).
- **Rationale:** AD-10, confirmed by the earlier finding that `leptess` has been unmaintained since 2023 — this step exists specifically to avoid ever linking an OCR library.
- **Security Considerations:** argv-array invocation eliminates shell injection by construction; temp files use unpredictable-enough names in the OS temp directory with default permissions, cleaned up unconditionally; the engine path comes from user configuration or `PATH` resolution, never from document content.
- **Testing Requirements:** a fake "engine" script in `tests/fixtures/fake-ocr/` (echoes known text to the expected output file) stands in for real Tesseract in CI, avoiding a hard Tesseract dependency in the test suite; successful round-trip via the fake engine; engine-not-found in `Auto` → warning, conversion still succeeds; engine-not-found in `Force` → fatal error; non-zero exit → `Ocr` error with captured stderr; no temp file survives either the success or failure path (assert via a before/after temp-dir listing).
- **Acceptance Criteria:** tests green; zero leaked temp files across all test runs.
- **Definition of Done:** tests green.

---

## P5-S03 — OCR wiring + verification

- **Step ID:** P5-S03
- **Objective:** Connect the adapter to both consumers (standalone images, scanned-PDF
  pages) and prove the whole OCR path end-to-end through the CLI — including a real scope
  boundary that surfaces here for the first time.
- **Depends On:** P5-S01, P5-S02, P4-S02
- **Files to Create:** `tests/phase5_integration.rs`
- **Files to Modify:** `src/extract/mod.rs` (`Format::Image` dispatch), `src/extract/pdf.rs` (route `NeedsOcr` pages through `run_ocr` where applicable)
- **Inputs:** P4-S02 (`NeedsOcr`), P5-S01/S02.
- **Outputs:** working end-to-end image conversion and scanned-PDF-with-OCR conversion.
- **Exact Instructions:**
  1. Wire `Format::Image` → `extract_image`.
  2. **Scanned-PDF OCR is only available in the `pdf-layout` build.** Rasterizing a PDF page to a bitmap for Tesseract to read requires an actual rendering engine — neither `pdf-extract` nor `lopdf` (Stage 4.A's dependencies) can do this; only `pdfium-render` (feature-gated, Stage 4.B) can. So: the default build's scanned pages always stay `Placeholder{kind: ScannedPage}` regardless of `--ocr` mode, with an explicit warning explaining why ("OCR of scanned PDF pages requires the pdf-layout build; this is the default build. Rebuild with --features pdf-layout, or convert standalone images directly.") — this is a real architectural boundary discovered here, not a bug to silently paper over; record it in architecture.md (AD-15) alongside AD-1/AD-10, since it's a direct structural consequence of both.
  3. Standalone images have no such gap — the file already is a bitmap, always eligible for OCR regardless of build profile.
  4. `pdf-layout` build: for each `NeedsOcr` page, rasterize via `pdfium-render`'s page-to-bitmap export (a capability already available once that dependency is present — no new crate) and OCR the resulting bitmap via `run_ocr`, replacing the placeholder with the recognized text.
  5. End-to-end tests via the fake-engine script from P5-S02: standalone image + `--ocr auto` → OCR text in output; scanned PDF, default build, `--ocr auto` → placeholders + the explicit "requires pdf-layout" warning, never silently skipped; scanned PDF, `--features pdf-layout`, `--ocr auto` → text recovered in place of placeholders.
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new (rasterization uses `pdfium-render`, already present in the `pdf-layout` build).
- **Rationale:** worth stating this build-profile boundary plainly here, at the point it's discovered, rather than as a surprise bug report after release.
- **Security Considerations:** none new beyond P5-S02's.
- **Testing Requirements:** all four scenarios in step 5; the default-build "requires pdf-layout" warning specifically asserted, not just "doesn't crash."
- **Acceptance Criteria:** all four scenarios pass.
- **Definition of Done:** `cargo test` (default and `--features pdf-layout`) green; `agent.md` updated with the Phase-5 exit-criteria checklist and the scanned-PDF-OCR build-profile caveat flagged for the README (P6-S05).

---

# Phase 6 — Batch, Hardening, Packaging, Fully Detailed

---

## P6-S01 — Batch: multiple files, directories, recursive scan

- **Step ID:** P6-S01
- **Objective:** Expand the CLI from one path (Phase 0) to N paths and directories, with
  extension filtering and structure-preserving output — sequential for now (P6-S02 adds
  parallelism).
- **Depends On:** P0-S08
- **Files to Modify:** `src/main.rs`, `src/convert.rs`
- **Inputs:** architecture.md §8.
- **Outputs:** `pub fn expand_inputs(paths: &[PathBuf], recursive: bool) -> Vec<Result<PathBuf, (PathBuf, ConvertError)>>`.
- **Exact Instructions:**
  1. A file path → included as-is. A directory without `-r`/`--recursive` → its immediate children only, filtered to extensions in P0-S02's `expected_formats` table (reused — avoids silently attempting to "convert" every stray file, including things like `.git` contents). A directory with `-r` → `walkdir::WalkDir::new(path)`, same filter, files only (directories skipped), `follow_links(false)` (a conservative default for a tool pointed at arbitrary user directories, avoiding symlink loops by construction).
  2. A nonexistent path → collected as a per-input error in the returned list, not a hard `main()`-level abort — P6-S02 formalizes the failure-isolation philosophy this feeds into; this step just needs to not special-case a bad path yet.
  3. Batch output: `-o` is treated as an output **directory**, not a single-file path. Each input's output filename follows Phase 0's stem+extension rule, written under `-o`, **mirroring the input's relative structure** when the input came from a recursive directory scan (`docs/a/b.docx` → `-o out/` → `out/a/b.md`, not a flattened dump that risks silent filename collisions across subdirectories that were never a collision in the source tree).
  4. `--overwrite` (P0-S08) applies per-file, identical to the single-file case.
- **Relevant Skills:** batch processing, filesystem traversal.
- **Approved Dependencies:** `walkdir` (already in the tree, unused until now).
- **Rationale:** architecture.md §8's batch data flow describes exactly this expansion; structure-preserving output avoids a flattening-induced collision risk that a naive implementation would introduce.
- **Security Considerations:** symlink-loop protection via `walkdir`'s own traversal safety; the local-directory-tree the user pointed the tool at is not itself part of the hostile-input threat model in §9 (that's about document *content*) — this project already trusts the filesystem paths the user chose to scan.
- **Testing Requirements:** mixed file+directory input list, some recursive some not; extension filtering excludes non-document files; nonexistent path → per-input error, batch continues; recursive scan preserves relative directory structure in output.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

---

## P6-S02 — Parallelism

- **Step ID:** P6-S02
- **Objective:** Convert the batch across a worker-thread pool with per-file failure
  isolation and an opt-in `--strict` stop-on-first-failure mode.
- **Depends On:** P6-S01
- **Files to Modify:** `src/convert.rs`, `src/main.rs`
- **Inputs:** architecture.md §5, §8, §9 (batch DoS mitigation).
- **Outputs:** `pub fn convert_batch(inputs: Vec<PathBuf>, opts: &Options) -> BatchResult` where `BatchResult { reports: Vec<(PathBuf, Result<Report>)> }`.
- **Exact Instructions:**
  1. Worker count from `opts.jobs` (declared in P0-S08, default `0` = auto): `std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).min(8)` — capped at 8 regardless of core count, since conversion mixes CPU (parsing) and I/O (file reads/writes) where an uncapped default risks I/O contention rather than real speedup on typical hardware.
  2. Implementation: `std::thread::scope` + a shared `Mutex<VecDeque<PathBuf>>` work queue (no channel/actor framework needed for this shape) with N workers each popping the next input and calling the existing single-file `convert_file` (P0-S08). **The parallelism boundary is exactly "one file, one call to the existing single-file pipeline"** — nothing about extraction/rendering itself needs to become thread-aware, since each `convert_file` call owns its own `AssetManager` and warnings vec with zero shared mutable state between concurrently-running files.
  3. Failure isolation: a worker's `Err` is recorded for that path in `BatchResult.reports`, and the worker moves on — a 100-file batch with one corrupt file still processes the other 99. `--strict`: the first detected failure (order not deterministic across threads — document this explicitly: `--strict` stops "no later than shortly after the first failure," not "at exactly file N") flips a shared `AtomicBool` checked before each worker pops new work; in-flight conversions already started finish rather than being forcibly aborted mid-file.
  4. Progress output: workers send lines through an `mpsc::Sender` to the main thread, which does all actual printing — avoids interleaved/garbled terminal output from concurrent writers.
  5. Batch exit code: 0 only if every file succeeded; 1 if any failed, `--strict` or not; final summary line reports counts ("87 converted, 3 skipped, 10 failed").
- **Relevant Skills:** concurrency, batch processing.
- **Approved Dependencies:** none new (`std::thread`/`std::sync` only).
- **Rationale:** implements architecture.md §9's batch-DoS mitigation exactly (per-file isolation, opt-in strict mode); keeping the parallelism boundary at the whole-file level is what keeps every extractor from Phases 0–5 completely untouched by this step — none need internal thread-safety, since none are ever called concurrently on the same document.
- **Security Considerations:** the shared queue/flag have no untrusted-input-driven sizing concern (bounded by P6-S01's already-reasoned input list); no new file-content trust boundary.
- **Testing Requirements:** N-file batch with one deliberately corrupt → all others still succeed, error captured in the report, exit code 1; same batch with `--strict` → stops promptly, fewer than N attempted, exit code 1; all-valid batch → exit code 0, clean non-interleaved progress output (captured stdout checked line-by-line for well-formedness); `--jobs 1` → fully sequential, deterministic, useful as a debugging mode.
- **Acceptance Criteria:** tests green.
- **Definition of Done:** tests green.

---

## P6-S03 — Profiles & config end-to-end

- **Step ID:** P6-S03
- **Objective:** Prove every built-in profile and the full config-file precedence chain
  (defaults → profile → config file → CLI) end-to-end, closing a gap Phase 0 left (profiles
  were unit-tested individually, never exercised in combination).
- **Depends On:** P0-S08
- **Files to Create:** `tests/phase6_profiles.rs`
- **Inputs:** blueprint.md P0-S08 (profile definitions and effects).
- **Outputs:** none new in `src/` — test-only, an audit closing a real gap.
- **Exact Instructions:**
  1. For each of the nine profiles (`academic, technical, plain, spreadsheet, archive, accessibility, tts, ocr, strict`): run a representative small batch (a DOCX, an XLSX, a scanned-page-stand-in PDF) through the CLI with `--profile <name>` and assert the profile's documented effects actually manifest (e.g. `plain` → every output `.txt` regardless of complexity; `spreadsheet` → formulas/comments/hidden-sheets present; `strict` → the batch actually stops on an induced failure).
  2. Precedence-chain test in one scenario, not pairs: a config file sets `format = md`; `--profile plain` (sets `format = txt`) overrides it; an explicit `-f md` on the command line overrides that — assert the final winning value is `md`, proving CLI > profile > config file > built-in default across all four layers at once.
- **Relevant Skills:** integration testing.
- **Approved Dependencies:** none new.
- **Rationale:** precedence-chain bugs look correct in isolated per-layer unit tests and break in combination — this step tests the combination explicitly. Worth a note in `agent.md` as a process learning (Phase 0's own step review should have caught this gap and didn't), not just a silently-added test.
- **Security Considerations:** none new.
- **Testing Requirements:** as above.
- **Acceptance Criteria:** tests green; the precedence test exercises all four layers together, not pairwise.
- **Definition of Done:** tests green.

---

## P6-S04 — Negative-test suite

- **Step ID:** P6-S04
- **Objective:** A consolidated, exact-message catalogue of hostile/malformed-input
  behavior mapped directly to architecture.md §9's threat table — the explicit audit that
  scattered per-phase tests actually add up to full coverage.
- **Depends On:** all prior phases
- **Files to Create:** `tests/negative_suite.rs`
- **Inputs:** architecture.md §9, §13.
- **Outputs:** none new in `src/` — every behavior under test should already exist from its originating phase; this is the audit.
- **Exact Instructions:**
  1. One test per §9 threat-table row, each asserting the *exact* `ConvertError` variant and message (not just "didn't crash"): corrupt/truncated ZIP → `Corrupt`; ZIP entry claiming an absurd decompressed size → guarded, not OOM; deeply-nested malformed XML → bounded, no blowup; truncated PDF → `Corrupt`; encrypted PDF (both build profiles) → `Encrypted`; OLE2 legacy DOC/PPT → `Unsupported` with the AD-5 explanation text; XLS specifically → **not** this category (AD-14 — cross-referenced here to make sure the negative suite doesn't wrongly test XLS as a rejection case); oversized file → `Unsupported` with the size-guard message; unknown format → `Unsupported` with detection notes; extension mismatch → re-asserted at the report level (not just `Detection`-struct level, already covered in P0-S02's own unit tests); `--strict` batch-stop (cross-referenced to P6-S02, re-run here as part of the consolidated suite).
  2. Every case runs through the actual CLI/`convert_file` entry point, not an isolated internal function — the goal is confidence in the end-to-end contract.
- **Relevant Skills:** security testing.
- **Approved Dependencies:** none new.
- **Rationale:** architect.md's own Boundaries ("never weaken security to simplify implementation") gets a concrete, auditable check here rather than trusting scattered per-phase coverage adds up correctly.
- **Security Considerations:** this step *is* the security-verification step.
- **Testing Requirements:** as above; a code comment mapping each test to its §9 row, so a future §9 addition has an obvious "add the matching test here" prompt — no row left silently uncovered.
- **Acceptance Criteria:** every §9 row has a passing, exact-message test; zero panics anywhere in the suite.
- **Definition of Done:** tests green; `agent.md` records the §9-row-to-test mapping.

---

## P6-S05 — Release: samples, docs, packaging for both build profiles

- **Step ID:** P6-S05
- **Objective:** The project's actual v1.0 exit bar (architecture.md §14) — samples, README,
  format-quirks reference, developer guide, and release artifacts for both build profiles
  with recorded binary sizes.
- **Depends On:** all prior phases — last step in the roadmap
- **Files to Create:** `README.md`, `docs/cli-reference.md`, `docs/format-quirks.md`, `docs/adding-a-format.md`, `tests/samples_gen.rs` (or an equivalent `scripts/` fixture generator)
- **Files to Modify:** CI config (both build profiles in the test/release matrix)
- **Inputs:** architecture.md §10, §14.
- **Outputs:** release binaries for both profiles, per target platform, with recorded sizes.
- **Exact Instructions:**
  1. `README.md`: what it does, install/usage, the two-build-profile explanation in plain language (point at AD-1's reasoning without reproducing the ADR itself), link to `docs/format-quirks.md`.
  2. `docs/cli-reference.md`: every flag from P0-S08/P6-S01/S02, either generated from `clap`'s own help machinery at build time or checked in CI against `clap`'s live `--help` output for drift — docs silently diverging from actual `--help` is a real, common bit-rot bug.
  3. `docs/format-quirks.md`: one section per format, every documented limitation from every phase's Rationale/instructions gathered in one place, stated plainly with the reason — RTF's heading-inference heuristic; the default build's approximate PDF reading order; legacy-XLS formula/chart gaps (AD-14); ODF formula-syntax non-translation; WMF/EMF non-decoding; math rendered as literal Unicode, not LaTeX (P4-S11); scanned-PDF OCR requiring the `pdf-layout` build (P5-S03/AD-15). This is the most direct fulfillment of architecture.md §0.1's "never silently discard information" applied to documentation itself — an honestly documented limitation is not a silent one.
  4. `docs/adding-a-format.md`: developer guide pointing at architecture.md §5's "one extractor module + one `Format` variant" rule, with P0-S06/P1-S01's shared-helper factoring as worked examples.
  5. Packaging: for each target platform (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64), build both profiles; record actual resulting binary sizes in `agent.md`, cross-checked against the default build's ≤5MB target (§14) — a default-build artifact that exceeds it is a finding to report, not silently ship over-budget.
  6. `pdf-layout` artifacts: bundle or clearly document PDFium sourcing per platform (P4-S04's three modes) — this is where that documentation promise actually gets written.
  7. Fixture generation: a script/module producing every fixture referenced across this entire blueprint programmatically (architecture.md §13's no-committed-binary-assets rule) — an audit that every phase's tests are genuinely reproducible from source.
- **Relevant Skills:** docs, packaging, CI.
- **Approved Dependencies:** none new.
- **Rationale:** this is the project's actual v1.0 gate — listed last because it genuinely is last: everything else has to be true before a release with recorded sizes and honest docs means anything.
- **Security Considerations:** reproducible builds (pinned `Cargo.lock`, documented toolchain version) matter specifically here, since this is the step that ships the artifact users actually trust.
- **Testing Requirements:** CI matrix builds/tests both profiles on all five platform targets; the fixture-generation script itself is tested (a broken generator silently invalidating half the test suite is exactly the kind of bug this project's own philosophy exists to prevent); a CI check diffing `docs/cli-reference.md` against `clap`'s live `--help`.
- **Acceptance Criteria:** every item in architecture.md §14's numbered list checked off with evidence in `agent.md`, not just asserted.
- **Definition of Done:** v1.0 tagged; `agent.md` records final state against every §14 criterion, including measured binary sizes and fuzz-hour counts.

---

# All Phases Detailed

Phases 0 through 6 are now fully specified. Nothing in the roadmap remains outline-only.
Implementation proceeds step by step, in dependency order, per the execution rule at the top
of this document — `cargo build` clean, `cargo test` green, `agent.md` updated, after each
step.
