# Format behavior and limitations

All behavior here describes unverified source. Retained Phase 0–3 gaps in
[implementation-notes.md](implementation-notes.md) still apply.

## PDF — default 4.A
Correctness-critical decoding is delegated to pdf-extract, with lopdf for structure
and per-page content-operator presence. Reading order is explicitly approximate.
Short isolated lines can be mistaken for headings. Text is kept in decoder order if
page separators are unreliable; page numbers are not invented. Images, tables,
font styles and columns are not reconstructed. Scanned pages become placeholders.
PDF OCR requires the layout build, including under `--ocr force`.
Encryption is rejected conservatively: a literal `/Encrypt` token anywhere can
reject an otherwise unencrypted PDF that happens to quote that token.

## PDF — 4.B layout
PDFium glyph coordinates feed line/column, font-size heading, margin repetition,
closed ruled-grid and approximate x-band table, caption, list, monospace-code and
math heuristics. Math stays Unicode, not LaTeX. Image bitmaps are re-encoded PNG,
not original compressed bytes. Rotated/vertical text, nested Form XObjects,
complex multiple independent ruled tables, merged table cells, multi-level lists,
inline mixed styles, code indentation and paragraph joining remain incomplete.
Wide gaps can confuse prose columns and unruled tables. Two-band unruled tables require a bold header or numeric content to avoid treating ordinary prose columns as tables. Image/table ordering and
captions remain heuristic. Header/footer stripping is limited to outer 10% margins
and 60% repetition across at least two pages. Explicitly retain with the toggle.
Force OCR replaces native text only when recognition is nonempty; images remain.
Full P4-S12 fidelity fixtures and a malicious-font panic fixture are still missing;
the written panic test injects a panic into the decode wrapper, not a font parser.

## DOCX
Shared OOXML text/style, lists, tables, hyperlinks, notes, images and OMML mappings
are supported in source. Arbitrary embedded objects and every OMML construct are
not reconstructed. WMF/EMF bytes may be preserved but are not decoded.

## EPUB
Spine order, local images and cross-chapter notes are supported. CSS layout,
JavaScript and external resources are not executed/fetched. Complex navigation and
malformed package fallback can lose structure; warnings disclose approximations.

## HTML / XHTML
Semantic extraction, not browser rendering. No network, CSS layout or scripts.
Malformed markup, complex lists/tables, and remote images may remain approximate.

## Markdown
Parser is a deliberately limited grammar, not full CommonMark. Explicit Markdown
passthrough copies source bytes and preserves relative links without reorganizing
assets; keep referenced files beside the output. Conversion can lose complex syntax.

## Plain text
UTF-8 text normalization and paragraph grouping. Other text encodings and meaningful
indentation need care; preview is not a byte-equivalence check.

## CSV / TSV
Delimiter-specific records become workbook/table structures. Locale-dependent
numbers, date inference and spreadsheet-like formula semantics are not guaranteed.

## XLSX
Calamine values combined with OOXML formulas, merges, metadata and relationships.
Full Excel number-format grammar, charts and drawing objects are not implemented.

## XLS
Native BIFF values through Calamine, not the unsupported legacy-DOC route. Legacy
formula text/chart/image fidelity depends on Calamine and remains incomplete.

## ODT
ODF styles, text, tables and notes are best effort. Full ODF inheritance, complex
list continuation and object fidelity remain gaps. ODF formulas are not translated
to Excel syntax.

## ODS
Repeated cells/rows, visibility, formulas and merges are handled with bounds.
All repeated-row/merge interactions and styles require further coverage. ODF formula
syntax remains literal rather than silently translated.

## RTF
Text, formatting, fields and hex pictures are parsed with limits. Heading inference
is heuristic; binary picture forms, complete field grammar, complex tables and all
code-page/font switching remain incomplete. WMF/EMF are not decoded.

## PPTX
Slides and shapes follow source order; optional speaker notes. Visual geometric
reading order, all drawing/chart exports and full slide layout are not guaranteed.

## Standalone PNG, JPEG, GIF, BMP, TIFF, WebP, JP2
Original bytes preserved; dimensions use bounded header readers, not full decoders.
BigTIFF, raw JPEG2000 codestreams, animated/multi-frame contents and complete header
validation are not supported. Unknown dimensions warn rather than discard bytes.
OCR auto and force explicitly opt standalone images into OCR; off never invokes it.
The external engine must accept Tesseract's input/output-stem/-l/language protocol.
Missing auto engine warns; missing force engine fails. Confidence is unavailable.
OCR currently uses blocking `Command::output`: no timeout, subprocess sandbox or
bounded stdout/stderr capture. Only final error detail and text-file size are bounded.
Use trusted engines; unattended hostile-input OCR needs further hardening.

## Legacy DOC, PPT, ODP and unknown formats
Detect-and-report/unsupported, not a converter for these formats. XLS is explicitly
separate. Error consistency and the full negative catalogue remain release work.
