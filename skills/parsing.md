# Skill: Parsing

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S04, P0-S05 actually need — not a general treatise on the topic.

## Scope

P0-S04/S05 text, limited Markdown and approved csv-crate extraction. Not a new general-purpose parser dependency or permission to claim full CommonMark.

Blueprint/architecture aliases: `parsing`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Separate block parsing from inline parsing. Track fenced-code state before interpreting emphasis, links or tables; support delimiter runs mechanically rather than replacing punctuation globally.

2. Use CommonMark 0.31.2 as the baseline grammar reference and identify project extensions explicitly. GFM tables are an extension; footnotes are not specified by the GFM document used here and need a project-defined contract.

3. For CSV/TSV, configure ReaderBuilder delimiter, headers and ragged-record policy intentionally. Feed the original record stream to the parser so quoted commas, doubled quotes and embedded newlines survive. Do not pre-split into lines.

4. Distinguish byte records from UTF-8 string records when designing an encoding path. If lossy decoding is retained, disclose replacement and keep the source bytes available where required; converting twice can corrupt valid content.

5. Bound nesting, token/field expansion and output work even when total input bytes are bounded. On unsupported Markdown forms preserve literal content and emit a specific limitation warning.

## Not This

Do not use split(",") for CSV, regex-only replacements for nested Markdown, or strip all whitespace before recognizing syntax.

## Common Mistakes

Unclosed fences swallowing the remainder silently; treating an escaped pipe as a column boundary; assuming csv flexible mode repairs or validates any record shape.

## Verification Checklist

- [ ] Quoted newline/comma, doubled quote, empty field and ragged-row cases have explicit expectations.
- [ ] Nested emphasis/lists, variable backtick runs and unclosed constructs preserve content.
- [ ] Encoding/length failures are deterministic and bounded.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://spec.commonmark.org/0.31.2/) — CommonMark Spec. Accessed 2026-09-16; retrieved. Evidence key: `commonmark`.
- [2](https://github.github.com/gfm/) — GitHub Flavored Markdown Spec. Accessed 2026-09-16; retrieved. Evidence key: `gfm`.
- [3](https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html) — ReaderBuilder in csv - Rust. Accessed 2026-09-16; retrieved. Evidence key: `csv`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
