
/// @MetaAnnotation @MetaName="Research Notes" @MetaType=Guideline
/// @AbstractAnnotation @name="Research" @type="Phase0"
# research.md — [Project Name] Research Notes

**Created**: YYYY-MM-DD  
**Purpose**: Document design rationale, technical investigations, decisions, alternatives, and unresolved questions to ensure transparency and maintainability.

---

## 1. Conceptual Design
- **Background**: Describe the purpose of this project and the problem it aims to solve.
- **Model Design**: Define the meta-model, annotation structure, and ontological distinctions (intent vs realization).
- **Research Items**:
  - Isomorphism between `DocumentAnnotation` and `CodeAnnotation`.
  - ID naming conventions (UUID, kebab-case, etc.).
- **Decisions**:
  - Adopt a shared schema and link annotations via `SpecId`.
- **Open Issues**:
  - Rules for ID generation.
  - Handling one-to-many or many-to-many mappings.
  - Synchronization strategy between documentation and code.

---

## 2. Implementation Strategy
### 2.1 Parsing Engine
- **Decision**: Use a regex-based scanner for MVP to extract annotations from Markdown and code; plan migration to `tree-sitter` for robust parsing.
- **Rationale**: Regex is quick and simple for MVP; `tree-sitter` offers accuracy but adds complexity.
- **Alternatives**:
  - `tree-sitter` (accurate AST parsing, multi-language).
  - Language-specific parsers (high maintenance cost).

### 2.2 Storage & Indexing
- **Decision**: Store canonical manifest files in `specs/` (JSON/YAML) as the source of truth; optionally maintain a local SQLite index for fast CLI queries.
- **Rationale**: Git-friendly and auditable; SQLite improves performance.
- **Alternatives**:
  - Pure JSON index (simpler but slower for large repos).
  - Remote DB/service (out of scope for MVP).

### 2.3 Status Detection
- **Decision**: Implement rule-based heuristics to compute `estimated_status` (e.g., implemented, in-progress, unimplemented).
- **Signals**: PR merge status, presence of tests, explicit `@status` metadata.

### 2.4 Manifest Schema
- **Decision**: Define `contracts/manifest.schema.json` with fields:
  - `id`, `type`, `source`, `location`, `status`, `meta`, `links`, `timestamps`.
- **Rationale**: Enables validation, contract tests, and interoperability.
- **Alternatives**:
  - YAML-only schema (human-friendly but less standard for tooling).

### 2.5 Git/PR Integration
- **Decision**: MVP supports local `git diff` for annotation changes; optional GitHub API integration for PR-level status.
- **Rationale**: Simple and CI-friendly.

### 2.6 CLI UX
- **Decision**: Provide commands:
    ```bash
    spec-trail scan --path . --out specs/annotations.json
    spec-trail report --coverage
    spec-trail check --integrity
    spec-trail diff --pr <id> --format json
    ```
- **Rationale**: CLI-first UX for automation and CI integration.

---

## 3. Operational Considerations
- CI/CD integration with JSON output.
- Design mockups for `report-ui` (HTML or dashboard).

---

## 4. Open Questions
1. Should IDs be auto-generated or manually assigned?
2. How should CI handle unimplemented specs (error vs warning)?
3. How to resolve conflicts when multiple annotations exist on the same function?
4. Efficient synchronization between `specs/` and `specify_manual/`.

---

## 5. Next Steps
1. Prototype regex-based scanner for Markdown and code.
2. Draft `manifest.schema.json` and add contract tests.
3. Implement basic status heuristic engine.
4. Define JSON structure for `report-ui` and create a simple HTML report.

---

## 6. References
- [OpenZeppelin Upgradeable Contracts](https://docs.openzeppelin.com/contracts/4.x/upgradeable)
- tree-sitter
- JSON Schema
