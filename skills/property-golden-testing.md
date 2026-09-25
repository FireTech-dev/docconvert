# Skill: Property-based / golden testing

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S06, P2-S05, P3-S03, P4-S03, P4-S12, P6-S04, P6-S05 actually need — not a general treatise on the topic.

## Scope

Architecture §12/§13 invariant, property and byte-golden testing. Reference to Proptest explains methodology, not approval to add that dependency.

Blueprint/architecture aliases: `Property-based / golden testing`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Define invariants on bounded generated structures: asset dedup returns the same identity only for equal bytes; a CDM walk visits nested content; span lowering neither overlaps occupancy nor loses anchor text; formula text survives lowering unchanged.

2. Generate valid structured cases and deliberately invalid boundary cases separately. Favor small recursive depth, bounded lengths and reproducible seeds; random bytes alone rarely exercise deep valid document paths.

3. When a generated case fails, minimize it while preserving the failure and retain a source-level reproduction. Record seed, generator version and expected behavior; shrinking is part of the methodology, not evidence that an unapproved crate was installed.

4. Use independently reviewed Markdown/TXT golden text for deterministic rendering. Include final newlines, escaping, whitespace and numeric alignment in the contract. Do not silently normalize differences that are meaningful to consumers.

5. A round trip is not always equality: PDF/layout/OCR extraction is lossy by design. State the narrower property, such as preservation of known text/formulas or stable output of an already-normalized CDM.

6. Keep property tests, fuzzing, golden comparisons and curated corpus testing as complementary evidence, not interchangeable counts.

## Not This

Do not define the oracle by calling the same renderer twice, add Proptest without dependency approval, or claim fuzz-hour requirements were met by a finite deterministic battery.

## Common Mistakes

Over-filtered generators that rarely produce cases; discarded shrinking seeds; snapshots hiding reordered columns because whitespace was normalized too broadly.

## Verification Checklist

- [ ] Every property states preconditions and failure oracle.
- [ ] Failures are reproducible from source/seed.
- [ ] Golden updates have independent review and do not normalize away data loss.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://proptest-rs.github.io/proptest/intro.html) — Introduction - Proptest. Accessed 2026-09-16; retrieved. Evidence key: `proptest`.
- [2](https://doc.rust-lang.org/book/ch11-03-test-organization.html) — Test Organization - The Rust Programming Language. Accessed 2026-09-16; retrieved. Evidence key: `tests`.
- [3](https://rust-fuzz.github.io/book/cargo-fuzz/coverage.html) — Coverage - Rust Fuzz Book. Accessed 2026-09-16; retrieved. Evidence key: `fuzz`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
