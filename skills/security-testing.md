# Skill: Security testing

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S04 actually need — not a general treatise on the topic.

## Scope

P6-S04 threat-table negative tests and architecture §13 fuzz/corpus evidence. Not certification that hostile documents cannot crash the process.

Blueprint/architecture aliases: `security testing`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Map each architecture §9 row to at least one public-entry-point fixture and exact controlled error/warning assertion. Include ZIP truncation/expansion, XML depth/entities, PDF corruption/encryption, unsupported legacy containers, size limits, unknown formats and extension mismatch.

2. Exercise budgets before and during work, not only after successful extraction. Tiny compressed inputs, huge sparse/repeated coordinates, oversized bitmap headers and extreme token nesting probe different allocation paths.

3. Assert output/asset/report side effects on failures. Use user-owned disposable directories and bounded subprocess/OS supervision for dangerous tests; do not let a fuzz test overwrite real files or invoke arbitrary external commands.

4. Fuzz the named RTF/PDF/CSV targets with deterministic harnesses and meaningful semantic oracles. Review coverage to identify unreachable paths, minimize new crashes, and retain source-reproducible regressions. A caught panic can hide a fuzz finding if the harness treats it as ordinary success; assert the appropriate boundary behavior explicitly.

5. Record actual CPU-hours, corpus revision, target/toolchain/build flags and crash triage. Architecture requires ≥24 CPU-hours per named target; CI smoke time or test annotations do not meet that requirement.

6. Audit the resolved dependency/native supply chain separately from parser behavior. No Cargo.lock, no audit evidence; Rust memory safety alone does not constrain native faults, resource exhaustion or filesystem races.

## Not This

Do not treat no panic in one call as full threat coverage, run hostile tests without bounded isolation, or claim a lockfile/fuzzer proves absence of vulnerabilities.

## Common Mistakes

Error checked only on a helper unreachable from CLI; malicious fixture fails before reaching the intended parser; fuzz coverage never reaches valid deep states; compressed-size-only guards.

## Verification Checklist

- [ ] Every threat row has an exact oracle and verified entry point.
- [ ] Crash/decompression/allocation tests have real resource supervision when run.
- [ ] Fuzz hours/corpus coverage and dependency audit records are actual evidence, not placeholders.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html) — File Upload - OWASP Cheat Sheet Series. Accessed 2026-09-16; retrieved. Evidence key: `security`.
- [2](https://cheatsheetseries.owasp.org/cheatsheets/XML_External_Entity_Prevention_Cheat_Sheet.html) — XML External Entity Prevention - OWASP Cheat Sheet Series. Accessed 2026-09-16; retrieved. Evidence key: `xxe`.
- [3](https://rust-fuzz.github.io/book/cargo-fuzz/coverage.html) — Coverage - Rust Fuzz Book. Accessed 2026-09-16; retrieved. Evidence key: `fuzz`.
- [4](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) — catch_unwind in std::panic - Rust. Accessed 2026-09-16; retrieved. Evidence key: `panic`.
- [5](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) — Cargo.toml vs Cargo.lock - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `cargo`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
