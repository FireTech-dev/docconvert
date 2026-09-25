# Skill: Temp-file handling

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P5-S02 actually need — not a general treatise on the topic.

## Scope

OCR scratch ownership/cleanup and exclusive staging creation. Not a promise of cleanup after process kill, power loss or an OS-level sandbox.

Blueprint/architecture aliases: `temp-file handling`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Create a unique owned scratch directory with exclusive creation/retry; PID+counter names help uniqueness but are predictable, not secrets. On Unix use restrictive directory permissions; review equivalent Windows access inherited from the trusted temp parent.

2. Within the owned directory use create_new(true) for input/reservation files. Existence-check followed by create is a race; OpenOptions documents atomic exclusive creation, including rejection of an existing dangling symlink at the leaf.

3. Install a drop guard immediately after reservation so every ordinary early return attempts cleanup. Close file handles before starting the engine or deleting files where required by Windows sharing semantics.

4. Treat engine output paths as untrusted filesystem objects even when the engine is configured: constrain the expected basename, reject unexpected symlink output, bound text reads and track cleanup failures appropriately. A leaf check does not prevent ancestor replacement races.

5. On abnormal termination, leave identifiable owned artifacts for an explicit recovery procedure. Never bulk-delete all similarly named system-temp entries while other conversions may still be using them.

## Not This

Do not use a fixed /tmp/ocr.png, assume predictable names confer privacy, delete an output not created by this operation, or equate a Drop attempt with guaranteed cleanup.

## Common Mistakes

Guard created after a fallible write; open handle prevents deletion; cleanup errors hide a successful-but-sensitive leftover; global before/after directory assertions race with unrelated tests.

## Verification Checklist

- [ ] Success, spawn failure, nonzero exit, missing output and read failure attempt cleanup.
- [ ] Exclusive-creation collisions and symlink leaves are tested.
- [ ] Recovery docs distinguish ordinary errors from kills/power loss.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) — OpenOptions in std::fs - Rust. Accessed 2026-09-16; retrieved. Evidence key: `temp`.
- [2](https://doc.rust-lang.org/std/process/struct.Command.html) — Command in std::process - Rust. Accessed 2026-09-16; retrieved. Evidence key: `command`.
- [3](https://doc.rust-lang.org/std/fs/fn.rename.html) — rename in std::fs - Rust. Accessed 2026-09-16; retrieved. Evidence key: `rename`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
