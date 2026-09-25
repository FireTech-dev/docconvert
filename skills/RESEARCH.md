# Research record and use limits

**Research date:** 2026-09-16. **Deliverable:** 34 template-based skills covering all
32 distinct detailed Relevant Skills topics and two additional architecture-map skills.
There are 42 detailed blueprint steps; P2-S03 says “none new” and inherits spreadsheet
lowering knowledge. Roadmap labels such as HTML, EPUB, RTF, Testing and Image headers
are mapped as aliases, not duplicated into contradictory files.

## Method

1. Read the uploaded `skill-writer.md` and `skill.template.md`; preserve byte-identical
   copies in `roles/skill-writer.md` and `project/skill.template.md`.
2. Inventory the blueprint’s Relevant Skills fields, the actual step objectives,
   requirements/security/tests, architecture §12 and the project’s known source gaps.
3. Perform online searches, then retrieve authoritative documentation wherever
   available. Prefer language/library maintainers, Microsoft, W3C, Unicode, OASIS,
   Ecma, PKWARE, ITU-hosted format specifications, OWASP and GitHub.
4. Use fixed-version references for materially version-sensitive APIs when available;
   label moving documentation URLs and baseline-standard choices. No dependency was
   installed, resolved or treated as locally verified.
5. Write actionable rules, anti-patterns, named mistakes, checklists and sources in
   the exact supplied section order. Separate facts, project thresholds, current
   source deviations and Architect decisions.
6. Inspect file existence, section shape, complete step/topic mapping, cited-source
   records, relative links and unchanged application/specification bytes. These are
   documentation checks, not application tests or independent expert certification.

## Source authority and recency

- **Rust/Cargo/standard library:** official live documentation retrieved, including
  ownership, scoped threads, mutex poisoning, Command, exclusive creation, rename,
  features and lockfiles. Moving rustdoc displayed its own version; that is not a
  claim about the user’s toolchain. Native and Windows-specific caveats take priority
  over simplified slogans about memory safety or argv.
- **Rust dependencies:** maintainers’ docs.rs pages for Clap, quick-xml, csv, zip,
  WalkDir, Calamine and Logos. Do not reuse an exact signature blindly after resolving
  future versions. PDF paths explicitly reference pdfium-render 0.9.4, lopdf 0.45.0
  and pdf-extract 0.12.0, re-retrieved for this research, not installed.
- **Markup/container standards:** ECMA-376’s parts have different edition dates;
  W3C EPUB 3.3 and PNG third edition, XML 1.0, WHATWG HTML, CommonMark 0.31.2 and GFM
  were consulted. ODF 1.3 is an explicit baseline, not an assertion it is the latest
  possible ODF revision. Newer-revision support requires a project decision.
- **Unicode:** fixed UAX #29 revision 47 and UAX #11. HTML-comment boilerplate was
  removed when checking publication status; hidden draft boilerplate is not evidence
  that the visible published document is a draft. Width tailoring is explicitly not
  equated with counting characters.
- **Images:** PNG/GIF/WebP and Microsoft BMP documentation, TIFF 6.0 baseline sections
  from ITU’s hosted specification, JPEG marker handling in libjpeg-turbo and JP2
  container structure from the Library of Congress with OpenJPEG implementation
  corroboration. Maintainer source is research evidence, not a new dependency.
- **RTF:** Microsoft’s 1.9.1 archived specification was discoverable through detailed
  search excerpts, but direct PDF retrieval failed. Microsoft’s older 1.5 text mirrored
  by Biblioscape was read for stable grammar and fallback rules. Crucially, its old
  lowercase-only control-word restriction is superseded by 1.9.1: the skill does
  **not** adopt that outdated restriction. Advanced grammar still needs full normative
  reference review when implemented; no complete 1.9.1-spec read is claimed.
- **Testing/release:** official Rust testing documentation, Rust Fuzz Book, Proptest
  maintainer methodology, GitHub Actions security guidance and Diátaxis. Proptest
  and Diátaxis are reference material, not dependency/architecture changes.

## Retrieval transparency

`research-sources.json` preserves keyed URLs, access date, status and short inspected
excerpts. It includes failed attempts and successful replacements; use the latest
successful record for a key. Excerpts are excerpts, not complete archived standards.
Some plain-text extraction may include navigation/table-of-contents text, so it is
not a substitute for reading the linked relevant sections.

- The initial WordprocessingML `/wordprocessing/` URL returned 404; the correct
  Microsoft `/word/` page was located and retrieved.
- The PDF Association reading-order article returned 403 to direct retrieval; its
  authoritative search excerpt was available. It is labeled **search excerpt only**.
- The Microsoft RTF 1.9.1 PDF is **search excerpt only**; the older corroborating
  mirror and the explicit newer-rule correction are recorded separately.
- TIFF PDF front matter and relevant baseline-field content were read through the
  page tool; other pages are not claimed read. An attempted intermediate chunk failed.
- Search aggregators/forums/AI-generated summaries were not promoted to authorities
  merely because they appeared in results. No unverified download/hash was supplied.

## What “best” means here

This pack aims for **authoritative, task-specific, auditable guidance**, not an
unprovable universal ranking. Standards govern format semantics; maintainers govern
API contracts; the project governs dependencies, output policy and heuristic thresholds.
Where those disagree or remain uncertain, the skill flags the issue instead of deciding
it quietly. See `ARCHITECT-QUESTIONS.md` for unresolved choices.

Re-research API/native/security/CI guidance whenever toolchain, dependencies or target
platforms change, and before release. Revalidate format guidance when broadening input
conformance. The research does not establish that the existing source implements all
of this guidance; no build, runtime, fuzzing or CI result was produced in this task.
