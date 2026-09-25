# Skill: Rust core

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S01, P0-S02, P0-S03 actually need — not a general treatise on the topic.

## Scope

Ownership, CDM enums, fallible extraction, asset identity and feature boundaries across the Rust library. Not permission to change the CDM or add crates.

Blueprint/architecture aliases: `Rust core`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Keep source buffers owned for at least as long as all views. A streaming event borrowing a scratch buffer cannot survive clearing/reusing that buffer; copy only the data that must outlive the event, or retain the approved immutable backing bytes.

2. Represent expected document failures with Result and controlled ConvertError messages. Use ? at fallible boundaries, not unwrap on input-dependent values. Match every Block/Inline variant when walking the CDM, including nested tables, lists and notes.

3. Use checked_add/checked_mul and checked conversions before allocating from dimensions, repetitions or declared lengths. Compare bytes after an asset hash match: a hash collision is not byte identity.

4. Use #[cfg(feature = "pdf-layout")] on optional imports/modules/fields. cfg!(...) selects a boolean but still requires both branches to type-check.

5. Keep catch_unwind narrowly around the prescribed decoder/per-file boundary. It catches unwinding panics, not aborts or native faults. Inspect panic profile before claiming containment; preserve the recorded abort/unwind conflict for review.

## Not This

Do not manufacture static lifetimes, use unsafe to bypass borrow errors, treat DefaultHasher as a durable cryptographic identity, or make panic recovery the ordinary error model.

## Common Mistakes

Clearing an event buffer while retaining borrowed text; forgetting a CDM variant; assuming an optional dependency disappears because its runtime branch is false.

## Verification Checklist

- [ ] User-run builds type-check both feature profiles independently.
- [ ] Malformed input returns controlled errors; CDM traversal cases include nested structures.
- [ ] Hash-collision handling compares bytes; dimensions are checked before allocations.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — What is Ownership? - The Rust Programming Language. Accessed 2026-09-16; retrieved. Evidence key: `rust`.
- [2](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) — catch_unwind in std::panic - Rust. Accessed 2026-09-16; retrieved. Evidence key: `panic`.
- [3](https://doc.rust-lang.org/cargo/reference/features.html) — Features - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `features`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
