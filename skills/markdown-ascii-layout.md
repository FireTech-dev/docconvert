# Skill: Markdown/ASCII layout

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S07 actually need — not a general treatise on the topic.

## Scope

P0-S07 CDM rendering and automatic format selection. Not a full Markdown-to-HTML renderer or font engine.

Blueprint/architecture aliases: `Markdown/ASCII layout`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Escape by syntactic context: ordinary text, link text/destination, table cell, inline code and fenced code need different handling. Choose code-span/fence delimiter lengths that cannot be closed by the content.

2. For GFM tables, protect literal pipes even inside code spans using the table extension rules; convert internal line breaks according to the project renderer contract. Expand rowspan/colspan through one shared occupancy model and preserve the anchor exactly once.

3. Indent nested list blocks relative to their marker width and nesting, including continuation paragraphs and fenced code. Keep ordered-list starts when the model carries them.

4. TXT tables must wrap without dropping remaining characters. Separate numeric alignment from string padding, and document the current width approximation: bytes, Unicode scalars, grapheme clusters and terminal columns are different units.

5. Derive auto output choice from the approved structural rules, not from source extension alone. Return an explainable reason and retain explicit user overrides. Preserve asset paths after final output namespace assignment.

## Not This

Do not globally backslash every punctuation mark, truncate a long cell to fit, equate chars().count() with screen columns, or apply normalization that changes formulas.

## Common Mistakes

Embedded backticks close a code span; colspan placeholders shift later cells; width measured before escaping; inline and block images use inconsistent paths.

## Verification Checklist

- [ ] Goldens include escaped pipes/backticks, nested lists and span occupancy.
- [ ] Concatenated wrapped cell text retains every source character.
- [ ] CJK/combining/emoji limitations are documented and selection reasons are tested.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://spec.commonmark.org/0.31.2/) — CommonMark Spec. Accessed 2026-09-16; retrieved. Evidence key: `commonmark`.
- [2](https://github.github.com/gfm/) — GitHub Flavored Markdown Spec. Accessed 2026-09-16; retrieved. Evidence key: `gfm`.
- [3](https://www.unicode.org/reports/tr29/tr29-47.html) — UAX #29: Unicode Text Segmentation. Accessed 2026-09-16; retrieved. Evidence key: `unicode17`.
- [4](https://www.unicode.org/reports/tr11/) — UAX #11: East Asian Width. Accessed 2026-09-16; retrieved. Evidence key: `width`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
