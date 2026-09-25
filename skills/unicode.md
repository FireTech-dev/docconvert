# Skill: Unicode

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P4-S11 actually need — not a general treatise on the topic.

## Scope

P4-S11 literal mathematical text and shared text-length/wrapping considerations. Not permission to add a Unicode library or normalize formulas arbitrarily.

Blueprint/architecture aliases: `Unicode`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Distinguish bytes, Unicode scalar values, extended grapheme clusters and rendered glyphs. Rust char is a scalar, not a user-perceived character or terminal column.

2. For grapheme-aware operations consult a published, versioned UAX #29; the researched fixed reference is revision 47. Use scalar counts only where the project contract explicitly calls for them, such as its current text length. Do not advertise scalar-safe wrapping as grapheme-safe wrapping.

3. Preserve extracted Unicode/formula sequences. Compatibility normalization can change mathematical distinctions; do not apply NFKC/NFKD globally. If normalization is needed for comparison keys, keep the original output text separately.

4. Math detection requires contextual evidence. Font names and U+2200–U+22FF/U+1D400–U+1D7FF are signals from the blueprint, not complete classifiers. Preserve unrecognized characters and warn instead of inventing LaTeX.

5. East_Asian_Width does not alone specify modern terminal width. Combining marks, emoji variation/joiner sequences and ambiguous-width characters need a documented width policy; adding a dependency or changing output layout requires approval.

6. Do not confuse grapheme segmentation with bidirectional layout or linguistic word segmentation; those are different algorithms.

## Not This

Do not split UTF-8 at arbitrary byte offsets, use len() as character count, discard combining marks, or describe every arrow/symbol as math.

## Common Mistakes

Surrogate code units emitted directly as Unicode scalars; a grapheme split over TXT lines; normalized math/text no longer matching the original formula.

## Verification Checklist

- [ ] Combining text, emoji sequences, CJK and supplementary math characters are represented in tests.
- [ ] The length/wrapping unit is named explicitly.
- [ ] Ordinary arrows and mixed prose/math avoid blanket equation classification.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://www.unicode.org/reports/tr29/tr29-47.html) — UAX #29: Unicode Text Segmentation. Accessed 2026-09-16; retrieved. Evidence key: `unicode17`.
- [2](https://www.unicode.org/reports/tr11/) — UAX #11: East Asian Width. Accessed 2026-09-16; retrieved. Evidence key: `width`.
- [3](https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageTextChar.html) — PdfPageTextChar in pdfium_render::prelude - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfglyph`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
