# Architect questions — research findings, not silent implementation changes

Date: 2026-09-16. These records distinguish technical facts from project choices.
**Application code and dependency configuration were not changed for this task.**
Existing revision-3 gaps remain open; writing a skill does not authorize a redesign.

| ID | Affected steps/skills | Finding and decision needed |
| --- | --- | --- |
| SK-AQ-01 | P4-S01/P6-S02; rust-core, ffi | Required catch_unwind cannot contain aborts/native faults. Revision 3 already selected unwind over the original abort optimization. Review that recorded choice and its measured binary-size impact; determine whether a future native-process isolation design is needed. No sandbox has been added. |
| SK-AQ-02 | P0-S06; html-semantics, streaming-xml | HTML’s recovery algorithm is not XML parsing with lowercase/void-tag substitutions. Reconcile full HTML fidelity expectations with the approved lightweight dependency/parser architecture; do not silently add a browser/DOM parser. |
| SK-AQ-03 | P5-S01; image-header-formats | Fixed-small-prefix reads cannot always reach JPEG SOF/TIFF IFD/JP2 metadata. Specify scan/read budgets and unknown-dimension behavior. A bounded header reader is not complete-image validation. |
| SK-AQ-04 | P5-S02; process-spawning, temp-file-handling | The prescribed blocking Command::output captures streams in memory and provides no timeout. Decide on any bounded-capture/termination policy explicitly. Windows batch wrappers have special argument parsing: decide whether to support them or require native executables; do not claim argv APIs eliminate every injection risk. |
| SK-AQ-05 | P4-S01; pdf-internals, file-format-signatures | Raw /Encrypt scanning can false-positive; operator-presence scanning can miss Form text or match data. Decide how conservative detection should be and how to reject encryption before loader auto-decryption. Do not promote approximate byte scans to semantic correctness claims. |
| SK-AQ-06 | P4-S07/S08; pdf-layout-typography | Current source adds an outer-10% margin restriction and a conservative two-band table rule not specified by the original thresholds. Review these as source deviations. They are not independently proven “best” thresholds. Full layout/table/column fixtures must decide fidelity, not the skill writer. |
| SK-AQ-07 | P4-S11; unicode, pdf-layout-typography | Unicode/font ranges alone do not satisfy the required ordinary-arrow false-positive test. Approve a contextual detection rule and warning aggregation behavior; do not fabricate equation semantics. |
| SK-AQ-08 | P0-S08/P6-S03; cli-design | Architecture/P0 precedence and a P6 example conflict. The existing recorded choice is defaults → profile → config → explicit CLI; retain it until the Architect resolves the source specification conflict. |
| SK-AQ-09 | P0-S07; markdown-ascii-layout, unicode | Scalar-count TXT wrapping is not terminal-column or grapheme-aware layout. Decide whether to retain/document approximation or authorize a width/segmentation implementation and its dependencies. Do not add Unicode crates inside a skill. |
| SK-AQ-10 | P0/P1/P4/P6; zip-container-anatomy, security-testing | Input-size or declared ZIP-size checks do not alone bound decompression, repeated cells, normalized XML or native allocations. Record explicit limits, rejection behavior and residual allocation risks; do not assert resource safety from the input limit alone. |
| SK-AQ-11 | P0-S08/P6; orchestration, filesystem-traversal | Multi-file outputs are not one atomic transaction and path prechecks are race-prone. Review recoverability/platform assumptions, particularly hard links and ancestor symlinks, before expanding the hostile-filesystem threat model. |
| SK-AQ-12 | P6-S04/S05; property-golden-testing, packaging, ci | Source-only delivery still lacks resolved lock/toolchain/native evidence and complete corpus/fuzz/CI/size results. Proptest is researched as a methodology reference, not approved as a dependency. CI action tags are not immutable pins; reviewers must choose real verified commit SHAs. |

## Evidence and scope

Each row links by skill filename to its technical sources in that skill’s **Sources**
section. Blueprint ratios, page counts, release budget and fuzz-hour requirements are
project decisions, not external standards. Questions do not override the preserved
blueprint or architecture. No “best overall parser/OCR engine/framework” selection
has been made: those dependency choices were already assigned by the Architect.
