# Skill: Continuous integration

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S05 actually need — not a general treatise on the topic.

## Scope

P6-S05 source validation matrix, trustworthy workflow execution and artifact evidence. Not proof that a written YAML workflow has passed.

Blueprint/architecture aliases: `CI`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Cover Linux x86_64/aarch64, macOS x86_64/aarch64 and Windows x86_64 for both default and layout profiles. Verify current runner labels and actual process target instead of assuming the label implies an architecture forever.

2. Require reviewed Cargo.lock and an exact toolchain. Run build/tests and live-help checks per profile; confirm the default dependency tree does not activate pdfium-render. Optional dependencies in the lockfile do not imply they were linked into the default executable.

3. Provision PDFium only in the user/CI setup phase from independently reviewed URL/hash/ABI records. Make missing setup a clear failure; do not silently skip native acceptance tests. Use the fake native OCR fixture for protocol tests and separate real-engine/corpus validation.

4. Give workflow tokens least privilege; do not expose secrets to untrusted pull-request code or execute such code in a privileged pull_request_target context. Avoid interpolating untrusted text directly into shell scripts.

5. Pin third-party actions to reviewed full commit SHAs and maintain those pins. Never invent a SHA. Tags in the existing workflow are a hardening gap, not immutable pins.

6. Upload uniquely named profile/target artifacts, logs, dependency tree and size evidence. A short fuzz smoke job can catch regressions but does not replace measured ≥24 CPU-hours per required target. Release publication remains a separate gated action.

## Not This

Do not call matrix YAML successful CI, use unreviewed downloads, expose repository secrets to arbitrary PR scripts, or make missing-library tests pass by ignoring them.

## Common Mistakes

Cache reused without feature/target awareness; runner CPU mismatch; immutable pin without provenance verification; overwritten artifact names mixing build profiles.

## Verification Checklist

- [ ] Ten target/profile combinations have actual identified outcomes.
- [ ] Workflow permissions/actions/native provenance are reviewed.
- [ ] Native tests fail clearly on missing prerequisites; release evidence remains separate.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions) — Secure use reference - GitHub Docs. Accessed 2026-09-16; retrieved. Evidence key: `ci`.
- [2](https://doc.rust-lang.org/cargo/reference/features.html) — Features - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `features`.
- [3](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) — Cargo.toml vs Cargo.lock - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `cargo`.
- [4](https://docs.rs/pdfium-render/0.9.4/pdfium_render/) — pdfium_render - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfium`.
- [5](https://rust-fuzz.github.io/book/cargo-fuzz/coverage.html) — Coverage - Rust Fuzz Book. Accessed 2026-09-16; retrieved. Evidence key: `fuzz`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
