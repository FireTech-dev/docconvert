# Security Audit Report — {{PROJECT_NAME}}

> Generated {{DATE}} by the Security Auditor. Every finding below was traced to a real, reachable path and verified with a safe test in an isolated environment before being listed — nothing here is speculative.

## Summary

| | Count |
| --- | --- |
| Findings — Critical | {{N}} |
| Findings — High | {{N}} |
| Findings — Medium | {{N}} |
| Findings — Low | {{N}} |
| Findings — Informational | {{N}} |

{{One paragraph: overall posture in plain language, and whether any Critical/High finding is currently unresolved.}}

## Findings

Ordered by severity, most severe first.

---

### [Severity] — {{Short title}}

**Location:** {{file / function / entry point}}

**Reachable path:** {{how untrusted input actually gets from its entry point to the dangerous operation}}

**Impact:** {{what a real attacker could actually do with this}}

**Verification:** {{the test written, and what was observed when it ran in the isolated environment}}

**Recommended fix:** {{correction}}

**Status:** {{Open | Mirrored to agent.md Known Issues | Fix in progress | Resolved, regression test passing}}

---

{{Repeat per finding}}

## Vulnerability Classes Checked

{{One line per class from security-auditor.md's checklist — what was checked and the result, even where nothing was found, so a clean pass is visible and distinguishable from "not checked."}}

## Re-Audit

This report reflects the project as of {{DATE}} only. Once a fix lands for any Critical or High finding, re-run its verification test as a regression check — don't close it out on the strength of the patch alone.
