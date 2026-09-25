# Skill: Batch processing

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S01, P6-S02 actually need — not a general treatise on the topic.

## Scope

P6-S01/S02 expansion, mirrored destinations, per-file results and strict scheduling semantics. Not parser-internal parallelism.

Blueprint/architecture aliases: `batch processing`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Expand explicit files and directories before conversion with the documented recursive/extension policy. Keep a provenance record for each item: source path, scan root, relative output path and expansion error if any.

2. Mirror recursive relative subdirectories beneath output. A missing explicit input is a per-input result, not a main-level abort that suppresses unrelated valid inputs.

3. Before workers write, identify output-stem/report/asset collisions across inputs, including aliases and platform case behavior where relevant. With overwrite enabled, two inputs still must not race to replace the same logical output.

4. Define stable input/result ordering independent of completion order. Reports must distinguish converted, intentionally skipped, failed and not-started entries; warnings are not failures unless policy says so.

5. Strict means stop handing out new jobs after the first detected failure. In-flight files may finish. With jobs=1 make this deterministic; with multiple workers do not assert a particular failing-file discovery order.

6. Prevent recursive scans from re-ingesting their own output tree according to the approved behavior, and surface unreadable-directory errors rather than silently flattening them away.

## Not This

Do not flatten all recursive paths, suppress nonexistent inputs, treat overwrite as collision arbitration, or promise strict instantly cancels running work.

## Common Mistakes

Duplicate canonical files scheduled twice; report namespace collides despite different final text extensions; progress shown as success for jobs never started.

## Verification Checklist

- [ ] Mixed roots preserve subdirectories and retain expansion failures.
- [ ] Collisions are deterministic before writes.
- [ ] Strict sequential and parallel isolation tests use the appropriate ordering guarantees.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html) — WalkDir in walkdir - Rust. Accessed 2026-09-16; retrieved. Evidence key: `walkdir`.
- [2](https://doc.rust-lang.org/std/thread/fn.scope.html) — scope in std::thread - Rust. Accessed 2026-09-16; retrieved. Evidence key: `threads`.
- [3](https://doc.rust-lang.org/book/ch11-03-test-organization.html) — Test Organization - The Rust Programming Language. Accessed 2026-09-16; retrieved. Evidence key: `tests`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
