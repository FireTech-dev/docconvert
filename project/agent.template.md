# Project Agent

> **Project:** {{PROJECT_NAME}}
>
> **Version:** 1.0
>
> **Maintained By:** Orchestrator (creation, sequencing, status) & Implementer (step records)
>
> **Created From:** `project/agent.template.md`
>
> **Status:** Not Started | In Progress | Blocked | Complete
>
> **Created:** {{DATE}}
>
> **Last Updated:** {{DATE}}

---

# Current Status

Current Phase:

Current Stage:

Current Step:

Current Objective:

Overall Progress:

```
Phase 0  ████████░░ 80%
Phase 1  ░░░░░░░░░░ 0%
Phase 2  ░░░░░░░░░░ 0%
Phase 3  ░░░░░░░░░░ 0%
Phase 4  ░░░░░░░░░░ 0%
Phase 5  ░░░░░░░░░░ 0%
```

Repository Status

- Current Branch:
- Latest Commit:
- Working Tree:
- Last Successful Build:

---

# 1. Project Summary

Brief description of the project.

Purpose

Primary users

Core workflow

Example format (replace with this project's actual workflow)

1. First step in the primary user/system workflow.
2. Next step.
3. ...continue through the core flow this project delivers.

Use whatever workflow steps actually describe this project — the numbered list above is a format example, not a suggested content.

---

# 2. Current Architecture Snapshot

Current stack actually implemented, not planned. Start from the universal rows; add the platform-specific rows that actually apply and drop the rest — don't leave irrelevant rows blank.

| Layer | Technology | Notes |
| --- | --- | --- |
| Runtime | | |
| Language | | |
| Testing | | |

Add rows as relevant to the target platform stated in `project/architecture.md` — for example:

- **Web/API:** API layer, Database, Cache, Queue, Frontend framework, Deployment target
- **Mobile:** UI framework, Local storage, Push notification service, App store target(s)
- **Desktop:** UI framework, Packaging/installer format, Update mechanism
- **CLI:** Package/distribution registry, Config file format

---

# 3. UI/UX & Design System Status

Only applicable if the target platform has a user interface, per `project/architecture.md`. If not, write "Not Applicable" and skip the rest of this section.

Design System

- Typography, color, spacing defined:
- Reusable component patterns established:

Accessibility

- Standard targeted (e.g. WCAG level, platform accessibility API):
- Current compliance status:

Coverage

- Screens/views built against the design system:
- Known visual or interaction gaps:

---

# 4. Current Project Structure

```
project/
```

Include only important directories.

Example

```
apps/
packages/
services/
database/
project/
roles/
skills/
```

Annotate each folder.

---

# 5. Environment

Omit any subsection below that doesn't apply to the target platform (for example, Ports for a CLI tool or a mobile app).

## Environment Variables

Only if the platform actually uses them — many mobile, desktop, and CLI projects don't.

| Variable | Purpose | Default |
| --- | --- | --- |
| | | |

Example for a web/API project, illustrative only — not a default to fill in: `DATABASE_URL` (database connection), `JWT_SECRET` (session signing). Never store actual secrets here, only names and purposes.

---

## Ports

Only applies to networked services — omit entirely for mobile, desktop, or CLI projects with nothing listening on a port.

| Service | Port |
| --- | --- |
| | |

---

## Infrastructure

Only what's actually relevant to the target platform — a mobile/desktop project may have nothing here beyond CI.

Docker services, if any

CI

Deployment or distribution target

External services depended on

---

# 6. External Capabilities

Third-party packages, plugins, MCP servers, or other installed (not just read) capabilities — each one verified by the Capability Auditor before use. Bounded by nature; only add a row when something new is actually installed.

| Name | Source | Version | Verified | Status | Permissions Granted |
| --- | --- | --- | --- | --- | --- |
| | | | | | |

Status is one of: Verified / Conditionally Safe / Not Recommended. Remove a row only if the capability is actually removed from the project — a stale "Not Recommended" entry is exactly the kind of thing worth keeping visible.

---

# 7. Completed Work

Full per-step detail for the **current phase only**. This section should never hold more than one phase's worth of full detail — once a phase's exit criteria are met and it's been reviewed, it gets archived (see Orchestrator's "Archiving a Completed Phase"): compacted to one line here, full detail moved to `project/agent-archive.md`. This is what keeps this file's size roughly constant regardless of how large the overall project gets.

---

## Current Phase — Phase N

### Step PN.1

Objective

Files Created

Files Modified

Dependencies Added

Security Implemented

Tests Executed

Acceptance Criteria

Completion Date

Notes

---

Repeat for every completed step in the current phase.

## Archived Phases

One line per phase already archived. Full detail lives in `project/agent-archive.md`.

```
Phase 0 — Auth & onboarding — 6 steps, complete, reviewed clean {{DATE}} — see agent-archive.md
```

---

This section reflects current reality, not a chronological log — rewrite a current-phase entry if a step is later revised.

---

# 8. Current Step

This section represents work actively being performed.

Step ID

Objective

Files Expected

Dependencies

Relevant Skills

Acceptance Criteria

Status: Not Started | In Progress | Blocked | Ready For Review | Complete

---

# 9. Known Issues

List confirmed issues.

Example

- Redis Docker image incompatible with ARM.
- Test flaky on Windows.
- Migration ordering under investigation.

Only actual, current issues. Remove an entry as soon as it's resolved — don't leave it sitting here as history; that's what Session Log is for.

---

# 10. Assumptions

Implementation assumptions made without affecting architecture.

Example

- UUID v7 used because blueprint did not specify UUID version.
- Used native fetch because runtime supports it.

Do NOT include architectural decisions. Remove an assumption once it's been confirmed or formalized into the architecture — a settled assumption isn't an open one anymore.

---

# 11. Architect Questions

Questions requiring Architect input.

Each entry:

---

## AQ-001

Blueprint Step

Question

Why Blocking

Status: Open | Resolved

Architect Response

---

Remove resolved entries only after they are incorporated into the blueprint.

---

# 12. Security Checklist

Current status. Mark items that don't apply to this platform as N/A rather than leaving them unchecked — an unchecked box should always mean "still to do," never "doesn't apply."

Universal — applies regardless of platform:

- [ ] Secrets externalized (never hardcoded, never committed)
- [ ] Input validation on every external-facing entry point
- [ ] Authentication complete, where the platform has any
- [ ] Authorization complete, where the platform has any
- [ ] Sensitive data protected at rest and in transit, as applicable
- [ ] Logging configured for security-relevant events
- [ ] Dependency audit done

Platform-specific — keep only the group(s) that match `architecture.md`'s stated target platform; mark the rest N/A as a group rather than item by item:

- **Web / API:** SQL injection protections · CSRF protections · XSS protections · secure cookies · session expiration · rate limiting · CORS configured deliberately, not wide open · HTTPS assumptions documented · tenant isolation verified, for multi-tenant systems · replay protection, for token-based auth
- **Mobile:** secure local storage (Keychain / Keystore, not plain files or shared preferences) · platform permissions scoped to what's actually used · certificate pinning, if the threat model calls for it
- **Desktop:** secure update/patch mechanism · code signing verified · safe inter-process communication
- **CLI / tooling:** safe handling of untrusted piped input · no arbitrary code execution from config or plugin files

---

# 13. Testing Status

| Type        | Passing | Total |
| ----------- | ------- | ----- |
| Unit        |         |       |
| Integration |         |       |
| End-to-End  |         |       |
| Security    |         |       |

Latest Test Run

Coverage

Known Failures

---

# 14. Next Steps

Generated from the blueprint.

Only list the next actionable steps.

Example

1. P0.4
2. P0.5
3. P1.1

Do not include completed work.

---

# 15. Session Log

Append-only, for the **current phase only**. Never edit a previous entry.

---

## {{DATE}}

Session Summary

Completed Steps

Files Created

Files Modified

Dependencies Added

Tests Executed

Security Work

Problems Encountered

Important Notes

Repository Status

---

Repeat for every implementation session in the current phase. When the phase is archived (see Orchestrator's "Archiving a Completed Phase"), these entries move to `project/agent-archive.md` in full, and this section starts empty for the next phase — append-only still holds, just per phase instead of for the whole project.

---

# 16. Review History

Architect review history.

---

## Review {{DATE}}

Reviewer

Blueprint Version

Result: Approved | Changes Requested

Major Findings

Required Fixes

Status: Open | Closed

---

Keep Open reviews, and the single most recent Closed one, in full. Once a review is Closed and superseded by a newer one, compact it to one line and move the full entry to `project/agent-archive.md`:

```
{{DATE}} — {{Reviewer}} — {{Result}} — see agent-archive.md
```
