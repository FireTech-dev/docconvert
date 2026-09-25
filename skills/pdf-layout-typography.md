# Skill: PDF layout/typography

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P4-S05, P4-S06, P4-S07, P4-S08, P4-S09, P4-S10, P4-S11 actually need — not a general treatise on the topic.

## Scope

P4-S05–S11 geometry-based lines, columns, headings, repetition, tables, images, lists/code and literal math. These are project heuristics, not guaranteed PDF semantics.

Blueprint/architecture aliases: `PDF layout/typography`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Normalize all bounding boxes into one page coordinate convention, accounting for rotation/crop/transform behavior of the verified API. With PDF bottom-left coordinates, descending y is top-to-bottom only after that normalization.

2. Cluster glyphs using the specified per-page median height and 0.3-height center tolerance; sort within lines by x and infer spaces from gaps. Validate finite metrics and bound glyph count. Superscripts/rotated text must not silently become reordered prose.

3. Find persistent inter-word gaps using the blueprint ≥40% line support and >2× median inter-word-gap threshold. Separate column gaps from word gaps when estimating statistics. Emit complete columns in order while keeping full-width headings/spanning blocks in their vertical sections.

4. Find modal body font size and apply the project ≥1.15 ratio, ≤120-character and ≥1.5-line-height isolation rules. Rank heading sizes into levels 1–6. These numerical choices come from the blueprint, not a standard claiming universal optimality.

5. Normalize repeated margin text/positions across pages; strip matching keys at ≥60% only per the approved policy. Changing digits must normalize consistently; single-page input is a no-op. The current outer-10% restriction is a source deviation needing review, not a standard requirement.

6. For ruled tables, transform path segments into page space and require connected/closed grid evidence, not just unrelated horizontal and vertical coordinates. For unruled tables require ≥3 aligned rows; explicitly distinguish prose columns and preserve ambiguity as a warning. Track absorbed glyphs so cells, captions, lists and code do not duplicate them.

7. Place images in their column/vertical reading position; consume a nearby non-heading caption once. Group nested list levels by consistent offsets; preserve code indentation and infer code only after removing table/caption lines.

8. Use font/Unicode evidence for math with context. Ordinary arrows in prose are not equations solely because of a code point. Emit literal Unicode, not inferred LaTeX, and aggregate the requested warning.

## Not This

Do not treat drawing order as reading order, use a global median across unlike pages, infer a table from any two columns, or claim heuristic thresholds are objectively best.

## Common Mistakes

Right-column image emitted among left-column text; headings split at column cuts; caption consumed as a table cell then emitted again; PDF transform applied twice; baseline superscript distortion.

## Verification Checklist

- [ ] Single/two-column, full-width heading and rotated/cropped fixtures exercise ordering.
- [ ] Ruled/unruled/no-table, nested lists and monospace cells test non-duplication.
- [ ] Header toggle, captions, math/prose-arrow distinction and warning aggregation are verified.
- [ ] Numeric thresholds and current source deviations remain traceable to blueprint decisions.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageTextChar.html) — PdfPageTextChar in pdfium_render::prelude - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfglyph`.
- [2](https://docs.rs/pdfium-render/0.9.4/pdfium_render/) — pdfium_render - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfium`.
- [2](https://pdfa.org/what-you-may-be-missing-when-you-search-pdf-documents/) — PDF Association: What you may be missing when you search PDF documents. Accessed 2026-09-16; search excerpt only. Evidence key: `pdforder`.
- [4](https://www.unicode.org/reports/tr29/tr29-47.html) — UAX #29: Unicode Text Segmentation. Accessed 2026-09-16; retrieved. Evidence key: `unicode17`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
