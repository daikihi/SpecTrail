# research.md — Spec-Impl-Annotations

**Created**: 2025-12-31
**Purpose**: Resolve technical unknowns and record decisions, rationale, and alternatives.

## Task 1: Parsing approach for inline annotations
- Decision: Start with a pragmatic, **regex-based scanner** for Markdown and simple inline annotation patterns in code (language-agnostic), and plan `tree-sitter` integration as a follow-up spike for languages where accuracy is required.
- Rationale: Regex-based scanning is quick to implement and works well for clearly delimited annotation syntaxes (e.g., `/// @MetaAnnotation` or `@spec` blocks). It allows a working MVP sooner while keeping a clear migration path to `tree-sitter` for robust parsing.
- Alternatives considered:
  - `tree-sitter` (accurate AST-level parsing, multi-language) — higher complexity and heavier dependency; recommended for Phase 1 if parsing edge cases become significant.
  - Language-specific parsers — too costly to maintain across many languages for MVP.

## Task 2: Storage & Indexing strategy
- Decision: Canonical manifest files stored in `specs/` (JSON or YAML) as the human-editable source of truth; maintain an optional **local SQLite index** for fast queries in the CLI (created/updated by the scanner).
- Rationale: Manifest files are simple and Git-friendly for traceability. SQLite offers fast query/filtering for interactive use and CI. Starting with manifest-only in MVP keeps the implementation simple and auditable.
- Alternatives considered:
  - Pure JSON index (no SQLite): simpler but can be slow for large repositories.
  - Remote DB/service (server): powerful but out of scope for MVP.

## Task 3: Status auto-detection heuristics (batch)
- Decision: Implement rule-based heuristic engine in the batch scanner that computes a **confidence score** and produces a `estimated_status` (e.g., `implemented|in-progress|unimplemented|deprecated|verified`). Rules include presence of implementation references, PR merge status, existence of tests, and explicit `@status` metadata in code/docs.
- Rationale: Rule-based heuristics are transparent and debuggable; they can be extended later with ML or more advanced analysis if needed.
- Alternatives considered:
  - Statistical/ML-based detection: too heavy for MVP; requires labeled data.
  - Manual-only status: simple but loses value of automation.

## Task 4: Manifest schema design (contract)
- Decision: Define a JSON Schema (`contracts/manifest.schema.json`) for normalized annotations: each entry includes `id`, `type`, `source` (document|code), `location` (file path + optional line), `status`, `meta`, `links` (traces), and timestamps. Provide schema and contract tests.
- Rationale: A formal schema enables validation, contract tests, and interoperability with other tools (e.g., editors, CI). JSON Schema is widely supported.
- Alternatives considered:
  - YAML-only schema: YAML is fine for humans but JSON Schema is more standard for validation in tooling.

## Task 5: Git/PR integration for diffs
- Decision: MVP will support local `git` diffs (detect added/removed/modified annotations via `git diff` or range diffs). For GitHub, add optional GitHub API integration to fetch PR-level changes and PR merge status for status heuristics.
- Rationale: Local `git diff` is simple and works in any CI/Dev environment; GitHub integration adds value but is optional.
- Alternatives considered:
  - Deep integration with hosting providers: useful but deferred to Phase 1.

## Task 6: CLI UX patterns
- Decision: Implement commands: `spec-trail scan --path . --out specs/annotations.json`, `spec-trail report --coverage`, `spec-trail check --integrity`, `spec-trail diff --pr <id>` with flags for `--format json|text`. Default for CI: `--format json`.
- Rationale: CLI-first UX aligns with the project's tooling focus and is scriptable for CI.

---

## Next Steps (Actionable)
1. Implement a small spike: `scan` that detects annotations in Markdown and Rust files with regex and emits a normalized manifest for a sample sub-tree. (Owner: dev, 1–2 days)  
2. Draft `contracts/manifest.schema.json` and add contract tests. (Owner: dev, 1 day)  
3. Prototype status heuristic: simple rule engine for `implemented|unimplemented` and add `estimated_status` to manifest output. (Owner: dev, 1–2 days)
4. Re-evaluate need for `tree-sitter` after handling edge cases in spike (decision point for Phase 1).


Decision footprint: All tasks have a clear next spike/prototype action; none remain as open NEEDS CLARIFICATION blocking Phase 1.