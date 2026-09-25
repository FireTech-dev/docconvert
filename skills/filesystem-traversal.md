# Skill: Filesystem traversal

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S01 actually need — not a general treatise on the topic.

## Scope

P6-S01 directory walking and output path safety. Not a race-proof hostile-filesystem sandbox.

Blueprint/architecture aliases: `filesystem traversal`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Use the approved walkdir with an explicit follow_links(false), depth policy and file-type filter. Inspect root symlink behavior separately: WalkDir has follow_root_links, so disabling descendant links alone does not establish a root-link policy.

2. Propagate traversal errors as per-input results. Do not discard Result errors through flatten() when the CLI contract requires reporting inaccessible paths.

3. Compute relative output paths against the original scan root, then validate they do not escape the output root. Distinguish lexical path normalization, canonicalized existing paths and nonexistent future output paths.

4. Use symlink_metadata for inspecting a link itself, metadata when intentionally following it; review every use of is_file/is_dir/canonicalize for implicit following behavior. Avoid string-prefix containment checks: /outside is not inside /out.

5. Generated basenames reduce document-controlled path attacks but do not fix ancestor symlinks, replacement races, case-folding collisions or platform reserved names. Keep the threat-model limits explicit and escalate descriptor-relative/openat-style hardening if required.

## Not This

Do not follow links by accident at the root, silently drop walk errors, compare containment as text prefixes, or assume a preflight path check remains true at publication time.

## Common Mistakes

Output nested inside scan root reprocessed; symlink loops; different relative spellings evade collision keys; Unix-only path assumptions on Windows.

## Verification Checklist

- [ ] Root/descendant links, inaccessible directories and mixed-case extensions have cases.
- [ ] Recursive output stays within the intended relative tree.
- [ ] No filesystem safety claim exceeds the implemented race protections.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html) — WalkDir in walkdir - Rust. Accessed 2026-09-16; retrieved. Evidence key: `walkdir`.
- [2](https://doc.rust-lang.org/std/fs/fn.rename.html) — rename in std::fs - Rust. Accessed 2026-09-16; retrieved. Evidence key: `rename`.
- [3](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) — OpenOptions in std::fs - Rust. Accessed 2026-09-16; retrieved. Evidence key: `temp`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
