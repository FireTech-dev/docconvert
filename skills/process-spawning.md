# Skill: Process spawning

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P5-S02 actually need — not a general treatise on the topic.

## Scope

P5-S02 Tesseract-compatible external OCR invocation and error reporting. Not engine training, network installation or a new subprocess architecture.

Blueprint/architecture aliases: `process spawning`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Keep the approved protocol as distinct arguments: Command::new(executable).arg(input_path).arg(output_stem).arg("-l").arg(language). The default text output is output_stem.txt; do not read recognized text from stdout unless a different protocol is approved.

2. Choose the executable from trusted user configuration/PATH, never document content. Do not build sh -c/cmd /C strings. On Windows, batch files have special argument parsing and historical vulnerabilities: prefer a reviewed native .exe; argv API use is not a blanket guarantee for arbitrary .cmd/.bat wrappers.

3. Off means do not call the adapter. Auto missing executable degrades with a warning; Force missing executable is fatal. A present engine that exits nonzero is a distinct Ocr error; bound/sanitize reported stderr without exposing private paths.

4. Use output only under an explicitly acknowledged blocking/unbounded-capture limitation. A timeout/capped-stream design requires reviewed lifecycle handling: drain pipes, terminate, wait/reap and clean up on all branches. Do not silently substitute a new protocol inside this skill.

5. Tesseract plain text supplies no per-word confidence. Keep confidence=None; TSV is a different explicitly requested output mode. Fake engines test argument boundaries and output-file handling, not recognition accuracy.

## Not This

Do not concatenate language/path values into a shell command, presume .cmd is the same as a native executable, or manufacture confidence scores from successful exit status.

## Common Mistakes

Appending .txt before passing outputbase, yielding .txt.txt; errors from missing language data mislabeled missing executable; pipe deadlocks in an unreviewed timeout patch.

## Verification Checklist

- [ ] Paths/languages with spaces and metacharacters remain single arguments.
- [ ] Off/Auto/Force, nonzero exit and output-file absence each have expectations.
- [ ] Windows native/batch distinction and blocking-capture limitation are documented.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/std/process/struct.Command.html) — Command in std::process - Rust. Accessed 2026-09-16; retrieved. Evidence key: `command`.
- [2](https://tesseract-ocr.github.io/tessdoc/Command-Line-Usage.html) — Command Line Usage | tessdoc. Accessed 2026-09-16; retrieved. Evidence key: `tesseract`.
- [3](https://github.com/rust-lang/rust/security/advisories/GHSA-2xg3-7mm6-98jj) — `std::process::Command` batch files argument escaping could be bypassed with trailing whitespace or periods · Advisory · rust-lang/rust · GitHub. Accessed 2026-09-16; retrieved. Evidence key: `rust-batch-advisory`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
