# Skill: Concurrency

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P6-S02 actually need — not a general treatise on the topic.

## Scope

P6-S02 scoped whole-file worker queues and strict stop/result coordination. Not replacing the architecture with async actors or assuming PDFium runs concurrently.

Blueprint/architecture aliases: `concurrency`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Use thread::scope and a shared Mutex-protected queue as specified. Pop the next job and check the stop flag in one short critical section; release the lock before reading, parsing, OCR, rendering or writing.

2. Choose the approved auto worker count from available_parallelism capped at eight; preserve documented explicit jobs behavior. Each worker owns a separate conversion context/asset manager.

3. Store results with their expansion indices and sort for deterministic reporting. Serialize progress output or collect it on the controlling thread; never interleave partial lines from workers.

4. On error set stop state under the synchronization protocol and retain the failed result. Do not discard already completed results or forcibly stop in-flight files under the current strict contract.

5. A scoped-thread panic can propagate at scope exit. Apply the approved per-file unwind boundary, review hook diagnostics and avoid panicking inside shared critical sections. Mutex poisoning is a signal to inspect invariants, not permission to blindly continue with any corrupted state.

6. Keep native PDFium’s documented serialized access. More Rust workers do not automatically yield parallel native parsing; process-level isolation/parallelism is a separate architectural proposal.

## Not This

Do not hold a queue lock across conversion, implement stop with an unsynchronized boolean, swallow poisoned-state corruption, or use thread count as proof of throughput.

## Common Mistakes

Join panic bypasses BatchResult; strict race assigns jobs after stop due to split locking; worker warning vectors shared accidentally; unbounded threads for large jobs values.

## Verification Checklist

- [ ] jobs=1 ordering is deterministic; multiple workers retain every result.
- [ ] No shared queue lock spans conversion work.
- [ ] Panic/strict/poisoning assumptions and native serialization are reviewed.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://doc.rust-lang.org/std/thread/fn.scope.html) — scope in std::thread - Rust. Accessed 2026-09-16; retrieved. Evidence key: `threads`.
- [2](https://doc.rust-lang.org/std/sync/struct.Mutex.html) — Mutex in std::sync - Rust. Accessed 2026-09-16; retrieved. Evidence key: `mutex`.
- [3](https://docs.rs/pdfium-render/0.9.4/pdfium_render/) — pdfium_render - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfium`.
- [4](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) — catch_unwind in std::panic - Rust. Accessed 2026-09-16; retrieved. Evidence key: `panic`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
