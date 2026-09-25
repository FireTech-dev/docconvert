# Skill: OOXML PresentationML

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P3-S01, P3-S02 actually need — not a general treatise on the topic.

## Scope

P3-S01/S02 slide order, shape-tree text, tables, pictures, chart placeholders and speaker notes. Not animation playback or exact visual slide rendering.

Blueprint/architecture aliases: `OOXML PresentationML`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Enumerate presentation sldIdLst in order, resolving each r:id through the presentation relationships. Numeric slide IDs and ZIP filenames are identifiers, not ordering algorithms.

2. Within a slide, process supported shape-tree children in source order and recurse into groups with a depth limit. Keep this source order distinct from inferred visual reading order; geometric reordering needs its own approved policy.

3. Resolve pictures/charts and note parts through the slide relationships. Never share a global rId map across slides. Preserve asset blocks in their position relative to text and tables.

4. Parse DrawingML paragraphs/runs and paragraph level/bullet properties. Preserve line breaks and text styles supported by CDM; expose unsupported drawing content through meaningful placeholders.

5. Apply include-notes only to the intended speaker-note text, avoiding master boilerplate/slide numbers where specified. Titles identified by placeholder metadata should not also be duplicated as body paragraphs.

## Not This

Do not sort slide filenames, append every picture at document end, treat chart inventory as chart rendering, or assume group children are independent top-level coordinates.

## Common Mistakes

Slide-local rId collision; duplicate title; notes-master boilerplate emitted as authored notes; drawing/source order marketed as exact visual order.

## Verification Checklist

- [ ] Nonsequential slide filenames still produce presentation order.
- [ ] A mixed text/picture/table/chart slide retains block placement.
- [ ] Notes default-off/default-on tests and repeated relationship IDs are included.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://learn.microsoft.com/en-us/office/open-xml/presentation/structure-of-a-presentationml-document) — Structure of a PresentationML document | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `presentation`.
- [2](https://ecma-international.org/publications-and-standards/standards/ecma-376/) — ECMA-376 - Ecma International. Accessed 2026-09-16; retrieved. Evidence key: `ooxml`.
- [3](https://www.w3.org/TR/xml/) — Extensible Markup Language (XML) 1.0 (Fifth Edition). Accessed 2026-09-16; retrieved. Evidence key: `xml`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
