# Skill: OOXML shared packaging and helpers

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S01 actually need — not a general treatise on the topic.

## Scope

P1-S01 Open Packaging Conventions, relationships, shared text/table/style helpers and OMML lowering. Not replacing the approved Rust extractors with the .NET SDK.

Blueprint/architecture aliases: `OOXML`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Use ECMA-376 Part 2 for packaging and Part 1 for markup semantics; do not assume all parts share the same edition date. Resolve each relationship relative to its owning part, not universally from the package root.

2. Load root relationships/content types to identify main parts. Keep relationship IDs local to their source part. Distinguish TargetMode=External from internal targets and never fetch an external relationship.

3. Recognize supported Strict/Transitional namespace forms deliberately. Prefix equality is not namespace equality. Resolve AlternateContent according to supported capabilities, otherwise preserve a limitation rather than concatenating Choice and Fallback.

4. Factor run properties, relationships, table spans and OMML mappings once. Enforce recursion limits in math/style inheritance. Unknown math constructs retain available literal content plus one useful warning, not fabricated LaTeX semantics.

5. Read compressed assets through bounded member lookup and use generated output names. WMF/EMF preservation does not imply those bytes are browser-renderable images.

## Not This

Do not hard-code every media path under word/media, resolve all rId values globally, or treat an SDK example as a requirement to add a .NET dependency.

## Common Mistakes

External target treated as a ZIP member; reused rId in different slide parts resolving to the wrong image; applying direct-formatting toggle rules blindly to inherited styles.

## Verification Checklist

- [ ] Relationships cover per-part ID reuse, external links and normalized targets.
- [ ] All specified OMML mappings and unknown fallback are checked.
- [ ] Strict/Transitional and alternative-content support boundaries are explicit.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://ecma-international.org/publications-and-standards/standards/ecma-376/) — ECMA-376 - Ecma International. Accessed 2026-09-16; retrieved. Evidence key: `ooxml`.
- [2](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) — zip. Accessed 2026-09-16; retrieved. Evidence key: `zip`.
- [3](https://www.w3.org/TR/xml/) — Extensible Markup Language (XML) 1.0 (Fifth Edition). Accessed 2026-09-16; retrieved. Evidence key: `xml`.
- [4](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.verticalmerge?view=openxml-3.0.1) — VerticalMerge Class (DocumentFormat.OpenXml.Wordprocessing) | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `vmerge`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
