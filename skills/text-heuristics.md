# Skill: Text heuristics

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P4-S02 actually need — not a general treatise on the topic.

## Scope

P4-S02 reconstruction from flat decoded PDF text. Not geometric reading-order recovery or a claim that short lines are objectively headings.

Blueprint/architecture aliases: `text heuristics`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Normalize line endings deliberately while keeping page separators if supplied. Split paragraphs on real blank-line boundaries; do not collapse all whitespace before detecting lists/code-like text.

2. Implement the blueprint’s isolated-short-line/no-trailing-punctuation heading rule as a named, testable project heuristic. Exclude known bullet/number markers from ordinary paragraph joining. Preserve the text when a classification is uncertain.

3. Warn unconditionally that default PDF reading order is approximate. Content-stream order may interleave columns; neither font size nor coordinates can be recovered reliably from an already flattened string.

4. Keep scanned placeholders associated with known page numbers. If whole-document decoding lacks reliable page segmentation, do not assign chunks to pages by guesswork or evenly divide the text.

5. Aggregate repetitive scanned-content warnings according to the project policy without removing the per-page placeholders/content. Keep OCR-needed, no text, decoder failure and blank page diagnostics distinguishable.

## Not This

Do not call text-only heuristics equivalent to 4.B, infer headings solely from uppercase text, or rewrite hyphenated words without a documented rule.

## Common Mistakes

Bullet lines joined into prose; page boundaries invented from arbitrary newlines; a large number of scans causing warning spam rather than a summary.

## Verification Checklist

- [ ] Prose, isolated headings, bullets and misleading short lines have expectations.
- [ ] Approximate-reading-order warning is asserted for every successful flat conversion.
- [ ] Mixed-page attribution never invents missing decoder boundaries.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/pdf-extract/0.12.0/pdf_extract/fn.extract_text_from_mem.html) — extract_text_from_mem in pdf_extract - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfextract`.
- [2](https://pdfa.org/what-you-may-be-missing-when-you-search-pdf-documents/) — PDF Association: What you may be missing when you search PDF documents. Accessed 2026-09-16; search excerpt only. Evidence key: `pdforder`.
- [3](https://spec.commonmark.org/0.31.2/) — CommonMark Spec. Accessed 2026-09-16; retrieved. Evidence key: `commonmark`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
