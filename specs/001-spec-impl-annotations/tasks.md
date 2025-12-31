# Tasks: 001-spec-impl-annotations

This file lists the prioritized tasks to finish the feature "Link Documents and Implementations with Annotations". The English spec (`spec.md`) is the source of truth; the Japanese file (`spec.ja.md`) is a translation.

## Overview
Tasks are ordered by priority. Work on one task at a time and update `specs/001-spec-impl-annotations/tasks.md` as progress is made.

---

## 1. Add manifest schema & contract tests (IN-PROGRESS)
- Path: `contracts/manifest.schema.json`, `tests/contracts/`
- Description: Add `version` and `generated_by` to manifest root. Add `annotation.version`, `introduced_by`, `introduced_at`, and `history` to annotation objects. Provide JSON Schema and write contract tests/fixtures to validate SC-005.
- Acceptance: Schema validates sample manifests; CI runs the schema validation tests successfully.
- Est: 1-2 days

---

## 2. Add manifest examples & fixtures
- Path: `specs/001-spec-impl-annotations/examples/`
- Description: Add typical manifests for (local scan, PR scan with `introduced_by`, broken reference case).
- Acceptance: Fixtures used by contract and integration tests.
- Est: 0.5 day

---

## 3. Scanner spike & heuristics
- Path: `src/scanner/spike`, `specs/001-spec-impl-annotations/research.md`
- Description: Implement a regex-based scanner spike that emits manifests and computes `estimated_status` using a documented ruleset.
- Acceptance: Spike outputs manifests matching schema and demonstrates estimated status on fixtures.
- Est: 1-2 days

---

## 4. Define CLI UX & output contracts
- Path: `quickstart.md`, `docs/cli.md`
- Description: Document commands (`scan`, `report`, `diff`, `check`), flags, output formats (JSON/human), and exit codes.
- Acceptance: CLI examples reproduce manifest and diff outputs.
- Est: 0.5 day

---

## 5. CI integration & PR gating
- Path: `.github/workflows/spectrail-scan.yml`
- Description: Add a GH Action to run scans in PRs, publish manifest artifact, and define warning vs failure thresholds.
- Acceptance: PR scans produce artifact and annotate PR with findings.
- Est: 1 day

---

## 6. Test suite: unit/integration/contract
- Path: `tests/` (unit/integration/contract)
- Description: Add parser unit tests, integration tests running scanner on fixtures, and contract tests validating manifests against schema.
- Acceptance: CI shows tests passing and fails on schema violations.
- Est: 1-2 days

---

## 7. Add CodeAnnotation example snippets
- Path: `examples/code-annotations/`
- Description: Add Rust/JS/Markdown examples of DocumentAnnotation and CodeAnnotation pairs for scanner tests and docs.
- Acceptance: Examples are runnable/input for integration tests.
- Est: 0.5 day

---

## 8. Specify diff semantics & output
- Path: `specs/001-spec-impl-annotations/spec.md`, `docs/cli.md`
- Description: Define what the `diff` shows (added/removed annotations, per-field changes), format of output and examples.
- Acceptance: Examples aligned with schema and CLI output.
- Est: 0.5 day

---

## 9. Define identifier naming & normalization rules
- Path: `spec.md` (Naming Conventions section)
- Description: Add canonical identifier rules, normalization, and collision resolution.
- Acceptance: Documented rules and examples.
- Est: 0.5 day

---

## 10. Error handling & reconciliation policy
- Path: `spec.md` (Operations/Policy section)
- Description: Define behavior for broken references, stale annotations, auto-suggest vs auto-fix rules, alerts.
- Acceptance: Policies with thresholds and operational steps.
- Est: 0.5 day

---

## 11. Non-functional requirements & performance targets
- Path: `spec.md` (NFR section)
- Description: Add measurable NFRs: scan targets, filter latency, logging/audit retention.
- Acceptance: NFRs documented and referenced in tests/benchmarks.
- Est: 0.5 day

---

## 12. Observability & metrics
- Path: `specs/001-spec-impl-annotations/research.md`, `src/scanner`
- Description: Define minimal metrics and instrument the scanner to emit them.
- Acceptance: Metrics appear in logs or JSON output from scanner.
- Est: 0.5 day

---

## 13. Update checklists & traceability items
- Path: `specs/001-spec-impl-annotations/checklists/`
- Description: Add checklist items for schema/version tests and traceability references to relevant spec sections.
- Acceptance: Checklists updated and reference SC-005/SC-006/SC-007.
- Est: 0.25 day

---

## 14. Localization workflow & sync process
- Path: `specs/001-spec-impl-annotations/README.md` or `tasks.md`
- Description: Document the process to keep English as master and update translations (`spec.ja.md`).
- Acceptance: A documented sync process and a short note in repo.
- Est: 0.25 day

---

> Note: The list is prioritized to unblock implementation (start with schema & tests). If you want, I can start with the schema work and open a PR with the changes. Reply with which task to start (task number or short name).