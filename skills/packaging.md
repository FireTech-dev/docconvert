# Skill: Packaging

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S05 actually need — not a general treatise on the topic.

## Scope

P6-S05 reproducible source/binary distribution, dependency/native provenance and measured release gates. Not authorization to publish or tag v1.0.

Blueprint/architecture aliases: `packaging`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Distinguish Cargo.toml version requirements from Cargo.lock resolution. For a binary release retain the reviewed lockfile, exact toolchain, target triple and enabled features; locked dependency resolution alone does not guarantee byte-reproducible binaries.

2. Build default and pdf-layout separately and label archives with OS/architecture/profile. Cargo can replace the same target binary when features change; package immediately from the intended verified build or use segregated target directories.

3. Record actual stripped executable bytes and hashes for each target. Enforce the project default ≤5 MB budget using an explicit byte-unit convention; do not substitute archive size for binary size. Layout binaries have no specified size cap but still require measurements.

4. For native PDFium record independent provenance/hash/license/ABI, whether bundled or externally sourced. The matching crate/native pair must be verified; a checksum from the same untrusted download is not independent authenticity proof.

5. Package complete required source, docs/templates/tests and manifest without caches, build outputs, secrets or generated dependency directories. Compare archive members against the workspace; verify originals are retained. Never put credential paths in a distributable snapshot.

6. No release tag until every architecture §14 gate has real evidence, including corpus, fuzzing, performance and both-profile platform CI.

## Not This

Do not publish wildcard-only dependencies as reproducible, reuse one profile’s binary for the other, invent measurements/provenance or bundle a native library without license review.

## Common Mistakes

Archive KB mistaken for stripped binary MB; wrong CPU architecture copied beside executable; source-only package described as a tested release.

## Verification Checklist

- [ ] Archive contents/hashes and source preservation are inspected.
- [ ] Every binary maps to exact source/lock/toolchain/target/features and measured bytes.
- [ ] Native provenance and all release gates are recorded before tagging.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) — Cargo.toml vs Cargo.lock - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `cargo`.
- [2](https://doc.rust-lang.org/cargo/reference/features.html) — Features - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `features`.
- [3](https://docs.rs/pdfium-render/0.9.4/pdfium_render/) — pdfium_render - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfium`.
- [4](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions) — Secure use reference - GitHub Docs. Accessed 2026-09-16; retrieved. Evidence key: `ci`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
