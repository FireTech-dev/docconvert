# Skill: OOXML SpreadsheetML

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P2-S02 actually need — not a general treatise on the topic.

## Scope

P2-S02 OOXML fallback/metadata paths around the approved Calamine reader. Not a second primary spreadsheet parser or formula evaluator.

Blueprint/architecture aliases: `OOXML SpreadsheetML`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Use workbook sheet records and relationship IDs to locate worksheets; do not derive sheet order from sheetN.xml names. Keep chartsheets and unsupported sheet types distinguishable from ordinary cell grids.

2. For fallback cells, interpret the type attribute before reading v. Shared-string indices refer into sst; inline strings and rich-text runs need their own path. Preserve sparse references, booleans, errors and styled numerics.

3. Retain f separately from v. Shared formulas reference a master via si; follower formula text may be omitted. Expanding them requires reference-aware handling of relative/absolute coordinates, not a regex that shifts every A1-looking string. If unavailable, warn instead of substituting the cached value.

4. Resolve style indices through the relevant style records and number-format definitions; consult workbook epoch for dates. Comments/hyperlinks/merges and drawing references belong to their owning sheet/part.

5. Keep legacy XLS on the Calamine BIFF route. Do not imply XML fallback APIs exist for an OLE2 binary workbook.

## Not This

Do not cast every v to a float, treat shared-string indices as visible numbers, assume formula text is always physically present, or use shared-formula expansion to evaluate expressions.

## Common Mistakes

Formula references inside string literals changed by rebasing; style index mistaken for number-format ID; hyperlink relationships taken from the workbook instead of the worksheet.

## Verification Checklist

- [ ] Shared/inline/rich strings, bool/error and sparse cells have fixtures.
- [ ] Formula master/follower plus absolute/mixed references preserve semantics or warn.
- [ ] Dates, merged cells and relationships are tested independently of cached values.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://learn.microsoft.com/en-us/office/open-xml/spreadsheet/structure-of-a-spreadsheetml-document) — Structure of a SpreadsheetML document | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `sheet`.
- [2](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.spreadsheet.cellformula?view=openxml-3.0.1) — CellFormula Class (DocumentFormat.OpenXml.Spreadsheet) | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `sharedformula`.
- [3](https://docs.rs/calamine/latest/calamine/trait.Reader.html) — Reader in calamine - Rust. Accessed 2026-09-16; retrieved. Evidence key: `calamine`.
- [4](https://support.microsoft.com/en-us/office/date-systems-in-excel-e7fe7167-48a9-4b96-bb53-5612a800b487) — Date systems in Excel | Microsoft Support. Accessed 2026-09-16; retrieved. Evidence key: `dates`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
