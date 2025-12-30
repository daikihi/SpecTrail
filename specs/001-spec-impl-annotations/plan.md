# Implementation Plan: Spec-Impl-Annotations

**Branch**: `001-spec-impl-annotations` | **Date**: 2025-12-31 | **Spec**: `spec.md`
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a CLI-first MVP that enables bidirectional traceability between specification documents and code via annotations. The MVP will provide a batch scanner to detect document and code annotations, produce a coverage and integrity report, and surface PR-level annotation diffs for reviewers. Initial approach favors Rust-based CLI tools (existing repo is Rust) using a manifest schema for normalized annotations and a simple indexing layer for fast queries. Auto-detection heuristics (status inference) will be implemented in the batch scanner as described in FR-003.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust (Cargo-based repository). Target toolchain: Rust 1.70+ (confirm exact CI toolchain) — ACTION: verify CI toolchain version.  
**Primary Dependencies**: `clap` (CLI parsing), `serde`/`serde_json`, `walkdir` (filesystem traversal), `regex`, `git2` (git integration) — **RESEARCH TASK**: evaluate `tree-sitter` vs `regex` for robust multi-language inline annotation parsing.  
**Storage**: Primary storage for annotations: normalized manifest file(s) in JSON/YAML inside `specs/` and a local index (SQLite or lightweight file index) for performance — **NEEDS CLARIFICATION / RESEARCH**: SQLite vs JSON index tradeoffs.  
**Testing**: `cargo test` for unit tests; integration tests for scanning and sample repos. Include contract tests for manifest schema.  
**Target Platform**: Cross-platform CLI (macOS, Linux, Windows via MSYS).  
**Project Type**: Single CLI tool integrated in repository (`src/cli`, `src/scanner`, `src/model`, `src/report`).  
**Performance Goals**: Scanning and producing a filterable result for a medium-sized repository (≤10k files) should complete within 2–5 seconds on a developer workstation; interactive filters should return in <2s for typical queries.  
**Constraints**: Must be language-agnostic for annotation detection; MVP supports Markdown and repository primary language (initially Rust), later multiple languages.  
**Scale/Scope**: MVP: single-repo support, Markdown + 1 programming language; Phase 1: expand language support and improve indexing/scale.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Constitution review (based on `spec-kit/memory/constitution.md`):

- **CLI Interface**: Complies — Feature implements a CLI batch scanner and reporter. ✅
- **Library-First / Modularity**: Complies — Implementation will be structured into reusable library crates (`scanner`, `index`, `report`) and a lightweight CLI wrapper to facilitate reuse. ✅
- **Test-First**: Complies — Plan requires unit/integration tests and contract tests for manifest schema. Tests will be added before feature completion. ✅
- **Observability & Simplicity**: Complies — Output formats include both human-readable and machine-readable (JSON) for observability and automation. ✅

**Gate result**: PASS — No constitution violations identified; proceed to Phase 0 research.

## Project Structure

### Documentation (this feature)

```text
specs/001-spec-impl-annotations/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── manifest.schema.json
└── tasks.md
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
