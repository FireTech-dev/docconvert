# Skill: ODF (OpenDocument XML)

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S04, P2-S04 actually need — not a general treatise on the topic.

## Scope

P1-S04/P2-S04 ODT/ODS packaging, styles, text, tables, typed values and repetition. Uses ODF 1.3 as an explicit baseline; no automatic adoption of newer ODF revisions.

Blueprint/architecture aliases: `ODF`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Read mimetype, manifest, content.xml, styles.xml and available metadata through bounded package helpers. Interpret namespace URIs, not presentation prefixes.

2. Keep common and automatic styles distinct; resolve referenced names and parent chains with cycle/depth guards. Preserve explicit whitespace elements, links, notes and paragraph/list boundaries.

3. For cells, retain typed office value attributes, displayed text and table:formula independently. Formula namespace prefixes identify syntax; preserve OpenFormula text instead of relabeling it as Excel formula syntax.

4. Track table:number-columns-repeated and table:number-rows-repeated as coordinate expansion, with checked limits before materialization. Copy coordinate-specific metadata carefully; covered-table-cell occupies a position but is not an independent merged anchor.

5. Respect constraints around repeated and merged cells. Do not repair malformed combinations silently or rebase formulas with naive string substitution. Preserve unsupported cases with a warning or controlled failure per the architecture.

6. Inspect package encryption metadata: compressed ZIP readability does not prove ODF content is unencrypted.

## Not This

Do not equate source element count with sheet coordinate count, ignore automatic styles, convert every office:value to display text without type metadata, or silently translate of:= formulas.

## Common Mistakes

Repeated annotations/hyperlinks lost after the first cell; row spans advancing the cursor twice; default empty repetition allocating millions of CDM cells.

## Verification Checklist

- [ ] Repeated rows/columns include formulas, comments, hyperlinks and hidden state in fixtures.
- [ ] Styles, whitespace, notes and table spans retain meaningful content.
- [ ] Malformed repetition/merge and excessive expansion fail predictably.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.oasis-open.org/office/OpenDocument/v1.3/os/part3-schema/OpenDocument-v1.3-os-part3-schema.html) — Open Document Format for Office Applications (OpenDocument) Version 1.3. Part 3: OpenDocument Schema. Accessed 2026-09-16; retrieved. Evidence key: `odf`.
- [2](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) — zip. Accessed 2026-09-16; retrieved. Evidence key: `zip`.
- [3](https://docs.rs/calamine/latest/calamine/trait.Reader.html) — Reader in calamine - Rust. Accessed 2026-09-16; retrieved. Evidence key: `calamine`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
