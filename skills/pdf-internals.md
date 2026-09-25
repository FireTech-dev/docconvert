# Skill: PDF internals

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P4-S01, P4-S04 actually need — not a general treatise on the topic.

## Scope

P4-S01/S04 PDF object/page/stream handling for flat and native paths. Not a handwritten font/CMap decoder or a security guarantee from token scanning.

Blueprint/architecture aliases: `PDF internals`, `PDF object/stream model + content-stream operators`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Delegate document loading/page enumeration to the approved library. Page-tree order, not object-number order, defines page sequence. Resolve supported indirect references and compressed streams through library APIs.

2. For default decoding use pdf_extract::extract_text_from_mem and the narrow panic boundary; inspect the pinned API before coding. lopdf 0.45.0 documents get_page_content_with_limit for bounded page decompression, but this does not bound earlier document loading or another crate’s allocations.

3. Reject encryption using the parser’s trailer/encryption state before intentional content decoding. Record that loaders may attempt empty-password decryption and the current lexical guard can false-positive; a correct fix requires reviewed parser behavior, not a claim that /Encrypt bytes are definitive.

4. Text-showing operators Tj, TJ, single quote and double quote are operators only in valid content syntax. Strings/comments/inline image data can contain those bytes, and Form XObjects can contain real text. Treat the blueprint presence scan as an approximate signal, not a complete scanned-page classifier.

5. Keep blank pages, raster-only pages, undecodable text and malformed content distinct. Default extraction cannot rasterize pages; PDF OCR must remain behind pdf-layout. Preserve page attribution only when the decoder actually supplies it.

## Not This

Do not parse font encodings manually, split pages by indirect-object order, scan compressed bytes as though they were operators, or label every zero-text page a scan.

## Common Mistakes

False encrypted result from quoted /Encrypt; recursive XObject text missed; enforcing a stream limit only after unbounded decompression; treating missing PDFium as a corrupt document.

## Verification Checklist

- [ ] Ordinary/mixed/blank/scanned/encrypted and damaged-xref cases are separate.
- [ ] Operator-like bytes inside strings/images do not establish semantic text.
- [ ] Library versions, error mappings and limits before/after decoding are documented.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/lopdf/0.45.0/lopdf/struct.Document.html) — Document in lopdf - Rust. Accessed 2026-09-16; retrieved. Evidence key: `lopdf`.
- [2](https://docs.rs/pdf-extract/0.12.0/pdf_extract/fn.extract_text_from_mem.html) — extract_text_from_mem in pdf_extract - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfextract`.
- [3](https://docs.rs/pdfium-render/0.9.4/pdfium_render/) — pdfium_render - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfium`.
- [4](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) — catch_unwind in std::panic - Rust. Accessed 2026-09-16; retrieved. Evidence key: `panic`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
