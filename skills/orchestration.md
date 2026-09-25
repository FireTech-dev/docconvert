# Skill: Orchestration

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S08 actually need — not a general treatise on the topic.

## Scope

The single-file detect → extract → CDM/assets → choose → render → publish/report pipeline and source-only execution gates. Not permission to bypass review or run the application.

Blueprint/architecture aliases: `orchestration`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Keep one owner for per-file options, warnings and assets; pass explicit contexts to extractors rather than introducing hidden global state. Detection does not imply extraction success.

2. Check input-size policy before parser entry, but also preserve downstream expansion budgets. Render using the final asset namespace; do not serialize stale report paths before renaming assets.

3. Preflight the complete output set for collisions and source-overwrite hazards. Stage beside destinations with exclusive creation; publish only complete data. Multi-file output/report/asset publication is not a single filesystem transaction: model rollback/recovery states explicitly.

4. Document platform/filesystem limits of rename, hard links and sync. A symlink check before opening is subject to races; it is not a race-proof sandbox. Preserve backups when rollback fails instead of deleting the recovery evidence.

5. Preview performs no final output publication; explicitly enabled OCR may still invoke a process/use temporary files. Tests must use the actual public pipeline, not only private helpers.

6. Keep user restrictions in force: write validation commands for the user, never represent static inspection as a passing build.

## Not This

Do not publish the report before the conversion outputs are secured, swallow rollback errors, or mark blueprint acceptance complete because files exist.

## Common Mistakes

Source-relative Markdown links broken by passthrough relocation; treating atomic rename of one file as atomicity of the whole output set; stale pre-namespace report assets.

## Verification Checklist

- [ ] Failure injection covers staging, publication and rollback with preserved recovery artifacts.
- [ ] Preview/skip/error paths have defined side effects.
- [ ] No build/test claim appears without actual evidence.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) — OpenOptions in std::fs - Rust. Accessed 2026-09-16; retrieved. Evidence key: `temp`.
- [2](https://doc.rust-lang.org/std/fs/fn.rename.html) — rename in std::fs - Rust. Accessed 2026-09-16; retrieved. Evidence key: `rename`.
- [3](https://doc.rust-lang.org/book/ch11-03-test-organization.html) — Test Organization - The Rust Programming Language. Accessed 2026-09-16; retrieved. Evidence key: `tests`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
