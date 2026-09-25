# Skill: CLI/config design

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S08 actually need — not a general treatise on the topic.

## Scope

P0-S08/P6-S03 Clap parsing, profiles/config precedence, exit codes and human/machine output. Not a new configuration framework.

Blueprint/architecture aliases: `CLI design`, `CLI/config design`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Preserve absence versus explicit CLI input. Parse options as Option values or inspect Clap value provenance; do not let a CLI default overwrite a profile/config value.

2. Apply defaults → profile → config file → explicit CLI, per the existing architecture decision. P6-S03 contains a contradictory example: record/escalate it, do not silently reverse precedence.

3. Keep output as a directory, validate positive size values and checked MiB conversion, and report unsupported keys with controlled suggestions. Treat arbitrary strings as data, not shell fragments.

4. Implement the documented status contract: usage/config-parse errors 2, conversion failure 1, success/intentional skip 0. A Config error discovered during per-file conversion needs the documented CLI mapping, not accidental enum-based process exits.

5. Keep quiet errors-only and verbose reports on stderr. Ensure stdout remains usable for intended progress/preview content. Generate/help-check the CLI reference under both build profiles, including layout-only flags.

## Not This

Do not use a default false boolean as evidence that the user disabled a profile setting, or classify output skips as successfully rewritten files.

## Common Mistakes

Config and CLI spelling drift; overflow in max-size multiplication; leaking full private paths through dependency errors; documenting flags unavailable in the default build.

## Verification Checklist

- [ ] All profiles and precedence layers have end-to-end override tests.
- [ ] Both live help inventories match docs.
- [ ] Exit 0/1/2, quiet/verbose and preview cases are asserted.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html) — clap::_derive::_tutorial - Rust. Accessed 2026-09-16; retrieved. Evidence key: `clap`.
- [2](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) — Cargo.toml vs Cargo.lock - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `cargo`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
