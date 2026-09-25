# Skill: OOXML WordprocessingML

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S02 actually need — not a general treatise on the topic.

## Scope

P1-S02 DOCX paragraphs/runs, styles, numbering, tables and related stories. Not Word page layout or tracked-change policy design.

Blueprint/architecture aliases: `OOXML WordprocessingML`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Traverse body blocks in document order; keep paragraph and run properties distinct. Preserve tabs/breaks and xml:space-sensitive text rather than concatenating only t strings.

2. Resolve pStyle through styleId and bounded basedOn inheritance; localized display names are not stable heading identifiers. Prefer the specified outline-level mapping and document fallback heuristics.

3. Resolve numId → numbering instance → abstract definition/level. Separate lists when numId changes; retain ilvl and explicit starts/overrides. Equal indentation alone is not a shared list identity.

4. Handle gridSpan horizontally and vMerge as restart/continuation with matching grid coverage. An omitted vMerge val represents continuation; absence of the vMerge element ends the merge. Assign rowspan to the anchor, not every continuation.

5. Resolve inline images/hyperlinks using the document part relationships. Extract footnotes/endnotes as separate stories and connect IDs without emitting separator-only notes as body text.

6. For fields, revisions and embedded objects unsupported by the current contract, preserve visible content and disclose the limitation; ask the Architect before choosing accept/reject tracked-change semantics.

## Not This

Do not identify headings by English style display names, merge adjacent lists solely by level, or duplicate content in merged cells.

## Common Mistakes

Ignoring inherited outline levels; treating empty vMerge as restart; extracting footnotes twice; losing field display text when instruction text is unsupported.

## Verification Checklist

- [ ] Non-English style names and inherited heading levels are covered.
- [ ] Different numId at equal ilvl remains separate.
- [ ] Horizontal/vertical merges, image, hyperlink and notes map to exact CDM expectations.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://learn.microsoft.com/en-us/office/open-xml/word/structure-of-a-wordprocessingml-document) — Structure of a WordprocessingML document | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `word`.
- [2](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.numberingproperties?view=openxml-3.0.1) — NumberingProperties Class (DocumentFormat.OpenXml.Wordprocessing) | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `numbering`.
- [3](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.verticalmerge?view=openxml-3.0.1) — VerticalMerge Class (DocumentFormat.OpenXml.Wordprocessing) | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `vmerge`.
- [4](https://ecma-international.org/publications-and-standards/standards/ecma-376/) — ECMA-376 - Ecma International. Accessed 2026-09-16; retrieved. Evidence key: `ooxml`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
