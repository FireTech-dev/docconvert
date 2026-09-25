# Skill: Integration testing

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S06, P2-S05, P3-S03, P4-S03, P4-S12, P5-S03, P6-S03 actually need — not a general treatise on the topic.

## Scope

Public library/CLI verification for every phase and both PDF profiles. Not proof of correctness from test counts or source inspection.

Blueprint/architecture aliases: `integration testing`, `testing`, `Testing`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Generate fixtures from source with controlled metadata and explicit semantic intent. Keep parser-header fixtures separate from complete standards-valid documents and real OCR images.

2. Use public extraction/conversion APIs for behavior contracts and the compiled CLI for exit codes, stdout/stderr, profile precedence and filesystem side effects. A helper unit test cannot establish that dispatch wired the helper correctly.

3. For each fixture record source semantic facts, expected CDM, expected rendered bytes and warning/error contract. Test text and formula preservation independently of output presentation.

4. Run default and pdf-layout suites separately on the user/CI side. A missing native PDFium library must not silently turn layout acceptance into a passing skip. Fake OCR verifies argv/output protocol only; real recognition quality needs a separate corpus.

5. Assert no partial output on failure, expected skip/overwrite behavior, cleanup after ordinary errors and isolation of unrelated files. Use dedicated temporary directories per test; avoid process-wide environment mutation in parallel tests.

6. Keep goldens reviewed, not auto-blessed after failures. Normalize nondeterministic report fields narrowly and test their shape separately. Under current restrictions, write commands/results fields but do not execute tests.

## Not This

Do not use only nonempty-output assertions, ignore native tests silently, compare only warning counts, or call injected-panic coverage proof of an actual malformed-font parser panic.

## Common Mistakes

Header-only PNG used as OCR-quality fixture; temporary-directory scans racing with other tests; expected output produced by the same algorithm being tested.

## Verification Checklist

- [ ] Each requirement maps to an observable assertion and fixture generator.
- [ ] Both native and default suites have explicit prerequisites/status.
- [ ] Failure paths check errors and filesystem effects, not merely no panic.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/book/ch11-03-test-organization.html) — Test Organization - The Rust Programming Language. Accessed 2026-09-16; retrieved. Evidence key: `tests`.
- [2](https://tesseract-ocr.github.io/tessdoc/Command-Line-Usage.html) — Command Line Usage | tessdoc. Accessed 2026-09-16; retrieved. Evidence key: `tesseract`.
- [3](https://rust-fuzz.github.io/book/cargo-fuzz/coverage.html) — Coverage - Rust Fuzz Book. Accessed 2026-09-16; retrieved. Evidence key: `fuzz`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
