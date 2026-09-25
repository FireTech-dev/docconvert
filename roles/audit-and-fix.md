# Audit & Fix — {{PROJECT_NAME}}

This session runs its own review — you don't need a separate Architect pass beforehand. It audits the project phase by phase, then as a whole, produces and applies fixes for anything clear-cut, updates `project/agent.md` throughout, and reports everything back at the end. Anything genuinely ambiguous gets flagged instead of guessed at — automatic fixing applies to clear findings, not uncertain ones.

Read, in order:

1. `roles/architect.md` — you'll adopt this role for both audit passes below, using its existing "Reviewing Implementation" and "Reviewing the Finished Project" checklists.
2. `roles/orchestrator.md` — you'll adopt this role afterward to apply whatever the audit finds, via its "Applying a Blueprint Revision" procedure.
3. `roles/designer.md`, if `project/architecture.md` states this platform has a UI — the full-project pass checks UI/UX compliance against `skills/design-system.md`, and a design-system-level finding is the Designer's to fix, not the Architect's.
4. `project/architecture.md`, `project/blueprint.md`, and `project/agent.md`, in full.
5. `project/agent-archive.md`, if it exists — needed for full detail on any phase already compacted out of `agent.md`.
6. The actual project source code. This audit reads what shipped, not just what `agent.md` says shipped — an Acceptance Criteria checkmark is a claim, not a verification.

Then, in order:

## 1. Open the Audit

Before reviewing anything, add a Review History entry in `project/agent.md`: reviewer "self-audit," scope "Phase-by-Phase + Full-Project," Status: Open. If the header Status is **Complete**, set it to **In Progress** — it returns to Complete only once every finding from this audit is resolved.

## 2. Phase-by-Phase Audit

As Architect, walk every phase — whichever holds the current phase's full detail in `project/agent.md`'s Completed Work, and whichever is listed under Archived Phases with full detail in `project/agent-archive.md` — oldest to newest. For an archived phase, pull its full Completed Work and Session Log entries from the archive; the one-line summary in `agent.md` isn't enough to audit against. For each phase on its own, apply "Reviewing Implementation"'s checklist — implementation order, blueprint compliance, dependency compliance, scope drift, undocumented assumptions, skipped steps, unresolved blockers.

Do this at the code level, not the record level. For every step in the phase, read its actual source files — don't take `agent.md`'s Acceptance Criteria checkmarks at face value; verify the code genuinely does what the step's Exact Instructions and Acceptance Criteria describe. Check for what a compliance record can't show on its own:

- dead code, unused imports or variables, logic left half-finished
- error handling that exists in form but not substance — a catch block that swallows the failure instead of handling it
- duplicated logic that should have been one function
- tests that are present and passing but don't actually exercise the behavior they claim to — a test that can't fail isn't coverage

Look for quality, not just defects, too — this matters because a project can span sessions running under different model capabilities: a context limit hit mid-step, a deliberate switch to a faster or cheaper model for routine work, or the reverse. Code that passes its tests and satisfies its acceptance criteria isn't necessarily written as well as a more capable model — possibly the one running this audit right now — would write it. Correct and well-written aren't the same bar, and this pass is the place to catch the gap between them:

- unnecessarily complex logic where a simpler, more direct approach exists
- inefficient patterns — redundant computation, avoidable loops, a data structure poorly suited to the operation being done
- code that works but doesn't follow the idioms or conventions the rest of the codebase already established

If something concerning enough to be a security question surfaces during this pass, note it and recommend a full `security-auditor.md` review rather than resolving it here — that's a deeper, adversarial pass this audit isn't scoped to replace.

Record every finding as you go: which phase, which step, what's wrong.

## 3. Full-Project Audit

Still as Architect, once every phase has been checked individually, apply "Reviewing the Finished Project"'s checklist across the project as a whole — blueprint compliance, architecture compliance, project structure, dependency usage, maintainability, production readiness, and UI/UX quality against `skills/design-system.md` if this platform has a UI. This pass exists specifically to catch what phase-by-phase can't: inconsistency between phases, drift that only shows up in aggregate, integration issues.

Extend the same code-level discipline here, but looking across phases rather than within one: logic duplicated between phases that should share a single implementation, inconsistent patterns for the same class of problem (two different error-handling styles doing the same job), and any step whose "done" status in `agent.md` doesn't hold up once you actually read what shipped. Uneven quality between phases — one noticeably more efficient or better-structured than another for no reason tied to the work itself — is often the visible trace of a model switch partway through the project; call it out specifically, not just as generic "inconsistency."

## 4. Classify Every Finding

First decide what kind of finding each one actually is:

- **Compliance finding** — the implementation doesn't actually satisfy what the blueprint says: drift, an unauthorized addition, an omission, or genuinely new scope. Decide whether fixing it is an architecture/blueprint change (Architect's call) or a design-system change (Designer's call, only if applicable). Classify it as one of the four categories from "Applying a Blueprint Revision" — drift correction, unauthorized addition to remove, omission to complete, or new phase/step to integrate — and produce the actual revision, identifying affected steps, migration instructions, and implementation impact, exactly as those roles already do for any revision.
- **Quality/efficiency finding** — the implementation already satisfies the step's Acceptance Criteria and Definition of Done; it's just written less well than it could be. Nothing about the blueprint, architecture, or design system needs to change here — the plan was fine, the execution just wasn't as good as it could have been.

**If a finding is ambiguous — unclear which of these it is, or how to fix it correctly — don't guess.** Flag it for the report in step 7 and leave it out of step 5.

## 5. Apply the Fixes

**Compliance findings:** adopt the Orchestrator role. Run its "Applying a Blueprint Revision" procedure for every finding classified as one — one step at a time, execution-readiness verified before handoff, dependencies respected, using whichever executor role this project runs (`roles/implementer.md` or `roles/mentor.md`). Respect any Review Checkpoint a step specifies — running automatically doesn't override a gate the blueprint itself requires sign-off for.

**Quality/efficiency findings:** hand the rewrite directly to the Implementer or Mentor — same step, same Acceptance Criteria and Definition of Done, nothing about what the code needs to do has changed, only how well it does it. Re-run that step's existing tests afterward as a regression check; a quality improvement that changes observable behavior isn't a quality improvement.

Update `project/agent.md` as you go: rewrite each fixed step's Completed Work entry, append one Session Log entry per fix naming the finding it resolves — for a quality finding, note explicitly that this was a quality/efficiency pass and not a correctness fix, so the distinction stays visible later rather than blurring into "something was wrong here." Regenerate Next Steps if scope changed.

## 6. Close the Audit

Once every clear-cut finding is fixed and verified, close the Review History entry from step 1: Status: Closed. If this audit had reopened a project that was previously Complete, set the header Status back to **Complete** now — not before.

## 7. Report Back

For every finding, fixed or flagged:

- **Where:** phase and step (or "cross-cutting" for a full-project finding)
- **What:** the issue itself
- **Why:** for a compliance finding, the specific blueprint/architecture requirement or principle it violated; for a quality finding, what a better implementation looks like and why it matters — efficiency, readability, maintainability
- **Fix:** what changed and which files/steps were touched — or, for a flagged item, why it needs your input instead

Close with a one-line summary: total findings, split by compliance vs. quality/efficiency, how many fixed automatically, how many flagged for you.

---

Rules for this session:

- A remediation step gets no lower a bar than a step being built for the first time — same tests, same acceptance criteria.
- Don't re-derive the review checklists inline — use the Architect's and Orchestrator's existing procedures as written; this prompt only sequences when each runs.
- Don't start any newly-revealed forward work until every finding from this audit is resolved or explicitly flagged.
- If a file path above doesn't match this repo's actual layout, use the equivalent file — the roles matter, not the exact path.
