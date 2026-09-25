# Skill: Spreadsheet data modeling

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S05, P2-S01, P2-S02, P2-S04 actually need — not a general treatise on the topic.

## Scope

Workbook/Sheet/Cell modeling, values, formula fidelity, dates, merged cells and bounded lowering in P0-S05/P2. No formula execution engine.

Blueprint/architecture aliases: `spreadsheet data modeling`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Store formula text independently of the cached/display value and cell kind. A numeric cached result is never a replacement for a present formula. Keep ODF formula namespace/syntax literal; do not translate to Excel syntax implicitly.

2. Use the approved Calamine reader for XLSX/XLS values and formula APIs; inspect its resolved-version return types before coding. Preserve the difference between unavailable formulas, empty formulas and parsing failure.

3. Model sparse coordinates explicitly. Retain hidden-sheet state, comments, hyperlink and merged-range metadata before lowering; display policy must not destroy source metadata.

4. Date inference needs both workbook epoch and number-format/type information. Excel 1900 and 1904 systems differ by 1,462 days; do not apply a universal Unix-epoch offset. Record policy for the legacy 1900 leap-day anomaly and elapsed-time formats rather than inventing calendar dates.

5. Keep merged anchor content once, track covered coordinates, and bound spans/repetitions before expansion. Apply the shared large-sheet policy instead of allocating a full rectangle from one distant cell.

## Not This

Do not evaluate formulas, treat all floats as dates, reconstruct BIFF formulas from cached values, or silently drop hidden/formula metadata while building a table.

## Common Mistakes

Off-by-one spreadsheet addresses; confusing elapsed hours with a date; multiplying repeated rows and columns before checking overflow; rendering merge continuations twice.

## Verification Checklist

- [ ] Formula bytes and cached values are asserted separately, including missing-cache cases.
- [ ] 1900/1904, numeric-not-date, hidden sheets and sparse large coordinates are covered.
- [ ] Merged and repeated cells preserve anchors with bounded expansion.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/calamine/latest/calamine/trait.Reader.html) — Reader in calamine - Rust. Accessed 2026-09-16; retrieved. Evidence key: `calamine`.
- [2](https://support.microsoft.com/en-us/office/date-systems-in-excel-e7fe7167-48a9-4b96-bb53-5612a800b487) — Date systems in Excel | Microsoft Support. Accessed 2026-09-16; retrieved. Evidence key: `dates`.
- [3](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.spreadsheet.cellformula?view=openxml-3.0.1) — CellFormula Class (DocumentFormat.OpenXml.Spreadsheet) | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `sharedformula`.
- [4](https://docs.oasis-open.org/office/OpenDocument/v1.3/os/part3-schema/OpenDocument-v1.3-os-part3-schema.html) — Open Document Format for Office Applications (OpenDocument) Version 1.3. Part 3: OpenDocument Schema. Accessed 2026-09-16; retrieved. Evidence key: `odf`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
