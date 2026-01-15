# Checklist: API / Contracts (Specification Quality)

**Purpose**: Validate contract and API-level requirements for `Spec-Impl-Annotations` before design/implementation
**Created**: 2025-12-31
**Feature**: `specs/001-spec-impl-annotations/spec.md`

## Requirement Completeness

- [ ] CHK001 - Are the manifest schema fields and required properties explicitly defined in `contracts/manifest.schema.json`? [Completeness, contracts/manifest.schema.json]
- [ ] CHK002 - Are error/validation response formats and failure modes documented for manifest validation and report generation? [Completeness, Spec §FR-004]
- [ ] CHK003 - Is versioning strategy for the manifest and contract changes defined (e.g., major/minor compatibility rules)? [Completeness, Spec §FR-006]

## Requirement Clarity

- [ ] CHK004 - Are all fields (types, units, allowed values) in the manifest schema described with examples? [Clarity, contracts/manifest.schema.json]
- [ ] CHK005 - Is the CLI/JSON output contract for `report`, `scan`, and `diff` commands clearly specified (formats, fields, example outputs)? [Clarity, quickstart.md]
- [ ] CHK006 - Are semantics of `status` and `estimated_status` (confidence, provenance) defined? [Clarity, Spec §FR-003, data-model.md]

## Requirement Consistency

- [ ] CHK007 - Are naming conventions and field semantics consistent between `data-model.md` and `contracts/manifest.schema.json`? [Consistency, data-model.md]
- [ ] CHK008 - Are error codes/messages and validation behaviors consistent across CLI and JSON outputs? [Consistency]

## Acceptance Criteria Quality

- [ ] CHK009 - Can the contract be programmatically validated (JSON Schema) and are failure conditions actionable? [Acceptance Criteria, contracts/manifest.schema.json]
- [ ] CHK010 - Are clear sample manifests included for typical and edge-case scenarios? [Measurability, contracts/manifest.schema.json]

## Scenario Coverage

- [ ] CHK011 - Are API/contract behaviors defined for examples: empty manifest, partial annotations, and corrupted manifest? [Coverage]
- [ ] CHK012 - Is the behavior for incremental updates (append-only manifests, diffing) described? [Coverage, FR-006]

## Edge Case Coverage

- [ ] CHK013 - Is handling of missing optional fields (e.g., line number) defined with canonical fallbacks? [Edge Case]
- [ ] CHK014 - Are constraints for very large manifests or binary artifacts specified (limits, streaming)? [Edge Case, Non-Functional]

## Non-Functional Requirements

- [ ] CHK015 - Are performance and size expectations for scanning and report generation documented (target times and scale)? [Non-Functional, Spec §SC-001]
- [ ] CHK016 - Are security requirements (access control for hosted APIs or GitHub integration) specified? [Security]

## Dependencies & Assumptions

- [ ] CHK017 - Are external dependency requirements (GitHub API, Git CLI) clearly documented including required permissions and rate considerations? [Dependencies]
- [ ] CHK018 - Is backward compatibility policy for manifest evolution documented and testable? [Assumption]

## Ambiguities & Conflicts

- [ ] CHK019 - Are any ambiguous contract terms or conflicting field definitions identified and marked for resolution? [Ambiguities]

---

Notes: Use this checklist during PR review and contract changes; include references to `contracts/manifest.schema.json` and `data-model.md` when marking items.