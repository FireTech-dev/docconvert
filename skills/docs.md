# Skill: Documentation

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S05 actually need — not a general treatise on the topic.

## Scope

P6-S05 user guide, CLI reference, format limitations, developer guide and source-state reporting. Not substituting documentation for missing implementation.

Blueprint/architecture aliases: `docs`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Separate tutorial/setup, task-oriented how-to, exact CLI/API reference and architectural explanation so users can find the right instruction. The supplied skill template governs skills regardless of documentation style preferences.

2. List every long/short flag, feature availability, default, precedence, exit status and output side effect. Check the reference against live Clap help in both builds; a hand-maintained inventory only checks names, not default/help semantics.

3. Provide exact user-run commands with prerequisites and target directories. Mark placeholder native URLs/hashes as placeholders; never present an invented checksum as verified provenance.

4. Give every input format a limitations section, including flat versus layout PDF, default PDF OCR restriction, literal Unicode math, XLS formulas/charts, ODF formula syntax and WMF/EMF preservation versus rendering.

5. Keep source-present, researched-API, statically-inspected, built, tested and release-accepted states distinct. Link outstanding conflicts and required evidence from agent.md and the run guide.

6. Update command examples when flags, fixture names or archive names change. Do not copy historical phase-complete wording into the current state unless evidence exists.

## Not This

Do not claim best/secure/full-fidelity without scope and evidence, call intended commands executed results, or obscure known gaps with generic best-effort labels.

## Common Mistakes

CLI flag names checked but default values drift; native setup assumes another machine’s PATH; header fixtures described as real scans; a previous archive named in the latest guide.

## Verification Checklist

- [ ] Every documented command target/path exists or is clearly a user-supplied prerequisite.
- [ ] CLI options and behavior contracts are checked in both profiles.
- [ ] Per-format limits and unverified status are prominent and linked.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://diataxis.fr/) — Diátaxis. Accessed 2026-09-16; retrieved. Evidence key: `diataxis`.
- [2](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html) — clap::_derive::_tutorial - Rust. Accessed 2026-09-16; retrieved. Evidence key: `clap`.
- [3](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) — Cargo.toml vs Cargo.lock - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `cargo`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
