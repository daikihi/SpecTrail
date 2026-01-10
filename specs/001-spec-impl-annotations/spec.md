# Feature Specification: Link Documents and Implementations with Annotations

**Feature Branch**: `001-spec-impl-annotations`  
**Created**: 2025-12-30  
**Status**: Draft  
**Input**: User description: "This project aims to link Documents and implementation with Annotations so teams can understand the relationship between specifications and code and track whether items are implemented or unimplemented."

---

/// @MetaAnnotation @MetaName="Spec-Impl-Annotations Feature" @MetaType=Philosophy
/// @AbstractAnnotation @name="SpecTrailFeature" @type="Feature" @spec="specs/001-spec-impl-annotations/spec.md"
/// @SpecDetailAnnotation @id="FR-ROOT" @name="FeatureTrace" @type="func" @meta="feature-level"



/// @AbstractAnnotation @name="Concept" @type="Overview"
## Concept

/// @AbstractAnnotation @name="Goal" @type="Goal"
### 1.1 Goal of SpecTrail
SpecTrail's purpose is to help software development teams (designers, developers, QA, infrastructure/operations, etc.) by bridging the gap between specification documents and implementations (code). Specifications are usually written in natural language and can diverge from implementation; using Annotations to link specifications and implementations provides traceability and alignment.

### 1.2 User Assumption and Use Case
**Target Users**: Backend and frontend developers, QA, infrastructure/operations, technical writers, and any roles that need to map specifications to implementation. Project managers may also use this feature to track progress.

**Use Case Examples**:
- For toolchains without a UI (like the SpecTrail project itself), support the cycle where designers and developers discuss specifications and mark implementation intent with annotations.
- In web service development, annotate specification items and track implementation references and status to simplify review and maintenance.

### 1.3 System Abstraction
**1.3.1 SpecTrail Components**
- SpecTrail Batches: CLI-based entry points (scan, integrity check, report generation, etc.).
- SpecTrail Reporter: Reporting functions for annotation coverage and broken references.
- (Future) SpecTrail Engine / SpecTrail Server: Service for aggregation, search, and synchronization of annotations.

**1.3.2 Annotation Structure Overview**
A three-layer model (AbstractAnnotation, SpecDetailAnnotation, ImplementationAnnotation) is used to map document and code annotations for traceability (see Data Model section for details).

**1.3.3 How to Map Specification between Specification and Implementation**
Add annotations to both documents and code; the SpecTrail Engine maps them by name or using configured linking rules. For readability, annotation names should follow a clear and consistent naming convention.

**1.3.4 Design Concept**
The design approach is to prioritize a practical CLI-based workflow as the MVP (adding annotations, scanning, report output, PR diff view), with future plans for server/service and editor integration.

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Developer wants to quickly check the implementation status of specification items (Priority: P1)

Developers want to see which implementations are associated with each specification item and view the current implementation status (e.g., Implemented / Unimplemented / In Progress) in a list.

**Why this priority**: Provides high daily value during development and reviews by enabling early detection of spec-implementation mismatches.

**Independent Test**: Apply the 'Unimplemented' filter on the repository's specification list page and confirm only items without implementations are shown.

**Acceptance Scenarios**:

1. **Given** annotated specifications exist in the repository, **When** a developer applies the 'Unimplemented' filter, **Then** only unimplemented specifications are listed.
2. **Given** a specification item has an implementation reference, **When** the developer clicks the reference link, **Then** the corresponding file/line opens in the IDE/editor or browser.

---

### User Story 2 - Reviewer wants to review the mapping between specification and implementation (Priority: P2)

Reviewers should be able to use annotations in pull requests or changesets to verify added/changed specifications and their implementations.

**Why this priority**: Useful for quality assurance and tracking change diffs; reduces review time.

**Independent Test**: In the PR annotation list, verify that changed specifications have correct implementation references.

**Acceptance Scenarios**:

1. **Given** a PR contains specification annotations, **When** the reviewer checks the annotation list, **Then** each annotation shows its implementation reference (file:line) or an 'unimplemented' flag.

---

### User Story 3 - Maintainer wants to understand annotation coverage (Priority: P3)

Maintainers want to periodically check annotation coverage (what percentage of specifications have implementation references) and lists of unimplemented items.

**Why this priority**: Helps visualize technical debt and prioritize work.

**Independent Test**: Run the coverage report and verify the ratio of annotated items to total specification items.

**Acceptance Scenarios**:

1. **Given** the repository has 100 specification items, **When** the maintainer runs the coverage report, **Then** the report returns how many items are annotated and have implementation references.

---

### Edge Cases

- What to do when a spec's line/ID changes and annotation references become stale (e.g., warn, suggest re-linking).
- How to handle multiple implementation references for the same specification (multiple languages or multiple implementation paths).
- Detection and update strategies when large refactors move referenced targets.

---

## Requirements *(mandatory)*

### Functional Requirements (all written to be testable)

/// @SpecDetailAnnotation @id="FR-001" @name="AddAnnotation" @type="func" @spec_section="User Scenarios"
- **FR-001**: Developers can add annotations to any specification item (document).
  - **Acceptance**: After adding an annotation to a specification item, the item appears in lists as annotated.

/// @SpecDetailAnnotation @id="FR-002" @name="SupportDocumentAndCodeAnnotations" @type="func" @spec_section="Requirements"
- **FR-002**: Annotations must support both inline code annotations (comments/attributes) and document annotations (the format shown in this spec) — the goal is to handle both, possibly in phased adoption.
  - **Acceptance**:
    - When a DocumentAnnotation and a CodeAnnotation share the same normalized identifier, they are automatically linked and displayed together in lists/details.
    - Cases where only one side exists (document-only or code-only) are explicitly shown and filterable (e.g., "document-only", "code-only", "unimplemented").
    - Scans produce a list of "unlinked annotations" (document-only/code-only), and any conflicts for the same identifier are flagged for manual review.

/// @SpecDetailAnnotation @id="FR-003" @name="MultiStageStatusWithBatchDetection" @type="infra" @spec_section="Requirements"
- **FR-003**: Annotations must support a multi-stage status model (e.g., Implemented / In Progress / Unimplemented / Deprecated / Verified) and support batch-based autonomous detection (status estimation) by SpecTrail.
  - **Acceptance**:
    - On batch (scan) execution, the system can output an "estimated status" for annotations (e.g., based on presence of implementation references, related tests, PR merge status).
    - Estimated results are shown as "estimated status" in lists and reports; users can manually approve or modify the final status.
    - If automatic estimation detects serious inconsistencies (e.g., broken references, identifier conflicts), alerts/flags are emitted and included in a detailed review list.

- **FR-004**: Ability to scan the repository, perform integrity checks on annotations (verify referenced targets exist), and output results as a report.
  - **Acceptance**: After running integrity checks, any broken references are shown in a list labeled "broken reference".

- **FR-005**: Ability to list annotations with search and filters (status, spec tags, file, unimplemented, etc.).
  - **Acceptance**: For example, filtering by "Unimplemented" returns only unimplemented items.

- **FR-006**: Annotations must be versionable and support viewing diffs for annotations added/changed per PR.
  - **Acceptance**: Annotations added in a PR are shown as diffs.

### Key Entities *(include if the feature involves data)*

- **Specification Item**: An individual item in a document or specification (ID, title, document, scope)
- **Annotation**: An annotation attached to a specification item (annotation ID, target spec ID, implementation references / unimplemented flag, status, metadata)
- **Implementation Reference**: A reference pointing to an implementation (file path, line number, symbol name, repository URL, etc.)
- **Report / Coverage**: Aggregated data such as annotation coverage and broken references.

---

## Version Annotation (Recommended)

**Purpose**: Trace the introduction and changes of annotations at PR/commit granularity and make diffs and introduction versions explicit.

**Proposed fields**:
- manifest level:
  - `version`: string (semver or tag) — manifest snapshot identifier (optional).
  - `generated_by`: { "tool": string, "tool_version": string, "timestamp": string (date-time) }
- annotation level:
  - `version`: string | null — annotation-specific introduction version (e.g., `v1.2.0`, `PR#123`, `2025-12-31`).
  - `introduced_by`: { "type": "pr"|"commit"|"manual", "id": string, "author"?: string }
  - `introduced_at`: string (ISO-8601 timestamp)
  - `history`: array of { "version": string, "changed_by": string, "when": string, "note"?: string }

**Schema proposal (addition to contracts/manifest.schema.json)**:
- Add `version` and `generated_by` at the root (both optional).
- Add `version`, `introduced_by`, `introduced_at`, and `history` to the annotation object.

**Operational rules**:
- When scanning in PR context (CI / pre-merge), the scanner sets `introduced_by` to the PR ID in the manifest output.
- For local single scans, `version` may be omitted/null, but users may set it manually.
- The diff feature uses differences in `annotation.version` and `introduced_by` to present changes.

**Acceptance Criteria (additional proposals)**:
- **SC-005**: `contracts/manifest.schema.json` accepts `version` and `generated_by`, and the scanner can emit them.
- **SC-006**: In PR scans, newly added annotations get `introduced_by` and it is reflected in the manifest.
- **SC-007**: Annotation diffs show additions/changes of `annotation.version`.

---

## Success Criteria *(mandatory, measurable & technology-agnostic)*

- **SC-001**: Developers can retrieve the list of 'Unimplemented' items (filter response should be fast; target: results returned within 2 seconds).
- **SC-002**: Reports produce numeric annotation coverage (e.g., annotated items / total items).
- **SC-003**: Integrity checks detect broken references and surface critical broken references in lists.
- **SC-004**: Annotation diffs are viewable during PR review so annotations can be considered as part of approvals.

---

## Assumptions

- Initially focus on the repository's primary languages and document formats (e.g., Markdown); consider multi-language support later.
- Annotations are expected to be added by humans, with lightweight automatic detection (matching class/function names to specs) provided as an assist feature.

---

## Specification Model: Formal Definition

**Canonical metamodel**: `specify_manual/meta.md` (main branch) is the authoritative source for the SpecTrail formal metamodel. A synced, convenience copy is available at `specs/001-spec-impl-annotations/metamodel.md` for local reference. Always treat the `specify_manual/meta.md` version as the master; changes to the metamodel should be made there and synchronized.

The following is an example formal definition (Formal Definition). It describes the structure of the spec and expresses relationships between document and implementation annotations in a mathematical/logical way.

/// @MetaAnnotation @MetaName="Definition of SpecTrailUnit" @MetaType=Philosophy
### 1.1 SpecTrailUnit
Let SpecTrailUnit = { CodeAnnotation, DocumentAnnotation }

CodeAnnotation is an entity of SpecTrail annotation written in actual programming code.  
DocumentAnnotation is an entity of SpecTrail annotation written in specification documents.

SpecTrailUnit represents a pair of annotations—one from code, one from documentation—  
that together form a traceable unit within the SpecTrail system.

/// @MetaAnnotation @MetaName="Definition of SpecTrailAnnotation" @MetaType=Philosophy
### 1.2 SpecTrailAnnotation
SpecTrail defines two complementary annotation spaces:

Let  
SpecTrailUnit = { DocumentAnnotation, CodeAnnotation }

Both represent collections of annotations that share a common structural schema
(MetaAnnotation, AbstractAnnotation, SpecDetailAnnotation),
but exist in different ontological domains.

- **DocumentAnnotation** exists in the *specification domain* —  
  annotations written within natural language or semi-structured documents.

- **CodeAnnotation** exists in the *implementation domain* —  
  annotations embedded within programming code or related metadata.

Formally:

```
DocumentAnnotation = { Mᴰ, Aᴰ, Dᴰ }
CodeAnnotation = { Mᶜ, Aᶜ, Dᶜ }
```

where each set corresponds to the three core layers of annotation structure:

- **MetaAnnotation (M)** — expresses design principles and conventions.  
- **AbstractAnnotation (A)** — expresses conceptual or domain-level intent.  
- **SpecDetailAnnotation (D)** — expresses concrete functional or structural specifications.

Each of these annotation types shares a *common schema* across spaces,  
but instances differ because they belong to distinct representation domains (textual vs code).

---

#### 1.2.1 Structural Mapping

A **Trace relation** establishes correspondence between DocumentAnnotation and CodeAnnotation.  
That is, each annotation in DocumentAnnotation may have one or more semantic counterparts in CodeAnnotation.

```
∀ aᴰ ∈ DocumentAnnotation,
∃ aᶜ ∈ CodeAnnotation such that Trace(aᴰ, aᶜ)
```

The mapping is **not required to be one-to-one**;  
it allows partial, compositional, or derived mappings to represent real-world divergence  
between written specifications and implemented systems.

---

#### 1.2.2 Philosophical Note

The DocumentAnnotation and CodeAnnotation spaces are *isomorphic by structure* but *distinct by substance*.

They share the same logical annotation model but live in different ontological strata:
- The **Document space** describes *what is intended*.
- The **Code space** describes *what exists*.

SpecTrail does not collapse these into a single ontology.  
Instead, it maintains both and enforces structural symmetry and semantic traceability between them.


/// @MetaAnnotation @MetaName="Definition of MetaType" @MetaType=Philosophy
#### 1.2.3 MetaAnnotation
MetaAnnotation is an annotation used to describe design directions, naming rules, and system management principles.  
It does not define features directly, but supports the structure and philosophy behind specification design.

In most cases, MetaAnnotations do not appear in programming code.  
However, they help readers of specification documents understand why certain specifications are not reflected in the source code.

```
M = {m₁, m₂, ..., mₙ} is a finite set of all possible MetaAnnotations.

∀m ∈ M:  
MetaName is a finite set of identifiers for MetaAnnotations.  
MetaType is a finite set of conceptual categories.  
m = {n, t}, where n ∈ MetaName, t ∈ MetaType.
```

MetaType can take the following values: `Philosophy`, `Guideline`, `Convention`, `Structure`, `Rule`.

/// @MetaAnnotation @MetaName="Definition of AbstractAnnotation" #MetaType=Philosophy
#### 1.2.4 AbstractAnnotation
AbstractAnnotation defines the high-level concept of the target software.  
This concept answers questions like:  

- Why does the team want to create the software?
- What kind of use cases or user needs are being prioritized?

In the context of a web service, an AbstractAnnotation often corresponds to a single web page or screen-level concept.  
It represents the overall purpose or user-facing role of that page within the system.

In addition, an AbstractAnnotation may also describe what kinds of components are involved—such as APIs, batch processes, or background jobs.

An AbstractAnnotation may contain several SpecDetailAnnotations.  
These detail annotations emerge when the abstract concept is split into sub-specifications—similar to how a task is broken down into subtasks.

Each AbstractAnnotation must include a `@name` tag to identify its concept.  
We recommend using a consistent naming convention—such as `CamelCase` or screen-level identifiers (e.g., `UserAuthPage`, `ProductListView`)—to ensure clarity and traceability across the project.

We will define SpecDetailAnnotation separately,  
but it's important to note that each SpecDetailAnnotation is structurally linked to an AbstractAnnotation via `@spec`.

```
A = {a₁, a₂, ..., aₙ} is a finite set of AbstractAnnotations.

For each a ∈ A:  
a = {na, ta, link}, where  
- na ∈ AbstractName  
- ta ∈ AbstractType  
- link ⊆ SpecDetailAnnotation

Here,

na identifies the abstract concept.

ta represents its type, such as domain concept, use case, or system role.

link connects this abstract concept to one or more SpecDetailAnnotations that concretize it.
```

/// @MetaAnnotation @MetaName="Definition of SpecDetailAnnotation" #MetaType=Philosophy
#### 1.2.5 SpecDetailAnnotation
SpecDetailAnnotation defines a specific behavior or functional aspect derived from an AbstractAnnotation.  
It represents a concrete specification that guides implementation.

In the context of a web service,  
a SpecDetailAnnotation often corresponds to an API specification, a user interaction flow, or a page-level behavior.

In the context of a one-shot batch process, a SpecDetailAnnotation often corresponds to a batch specification such as loading master data from a filesystem and inserting it into a database.

```
D = {d₁, d₂, ..., dₖ} is a finite set of SpecDetailAnnotations.

For each d ∈ D:  
d = {nd, td, link}, where  
- nd ∈ SpecDetailName  
- td ∈ SpecDetailType  
- link ⊆ {AbstractAnnotation ∪ ImplementationAnnotation}

Here,

nd identifies the detailed specification.

td specifies its type, such as entity, relation, operation, or rule.

link connects this detail both upward (to AbstractAnnotation) and downward (to ImplementationAnnotation), forming a bidirectional specification trace.
```

/// @MetaAnnotation @MetaName="Definition of ImplementationAnnotation" #MetaType=Philosophy
#### 1.2.6 ImplementationAnnotation

ImplementationAnnotation defines a concrete implementation-level specification that realizes a particular SpecDetailAnnotation.
It provides the semantic bridge between specification and source code, representing how a detailed specification is technically realized.

Unlike SpecDetailAnnotation, which describes what should be done,
ImplementationAnnotation focuses on the conceptual role of the implementation rather than its physical code location or language.
Information such as file paths or programming languages is managed separately by CodeAnnotation and connected through trace relations.

A single SpecDetailAnnotation may correspond to multiple ImplementationAnnotations,
each describing a distinct implementation aspect (e.g., database access, repository design, gateway integration).

ImplementationAnnotation is typically used to express:

Database-related implementation semantics (tables, columns, constraints)

Design of data access layers (DAO modules)

Structure definitions for domain entities

Repository or gateway interface specifications

Web API design and data model definitions

Any other conceptually distinct technical realization of a specification

```
I = {i₁, i₂, ..., iₗ} is a finite set of ImplementationAnnotations.

For each i ∈ I:  
i = {ni, ti, link, art, status}, where  
- ni ∈ ImplementationSpecName  
- ti ∈ ImplementationType  
- link ⊆ {SpecDetailAnnotation ∪ AbstractAnnotation}
- art ∈ ImplementationArtifact  
- status ∈ ImplementationStatus
```

Here,
- ni identifies the ImplementationAnnotation.

- ti classifies its role, such as DAO, Repository, Gateway, or API.

- link associates this implementation with its related SpecDetailAnnotation(s) and AbstractAnnotation(s).

- art defines the semantic target of the implementation, such as database, domain, external_system, or web_interface.

- status represents the implementation’s maturity or verification state, such as draft, implemented, or verified.

#### 1.2.7 Annotation Trace
In combination with link references, Traces form the formal mapping between conceptual, detailed, and implementation layers, ensuring full bidirectional traceability within the SpecTrail system.

```
T = {t₁, t₂, ..., tₘ} is a finite set of Traces.

For each t ∈ T:  
t = {src, dst, kind}, where  
- src ∈ {A ∪ D ∪ I}  
- dst ∈ {A ∪ D ∪ I}  
- kind ∈ TraceKind
```

Here,

src and dst denote the source and destination of the trace link.

kind indicates the semantics of the relationship (e.g., refines, implements, verifies, derives).
Traces thus form the structural backbone of the SpecTrail, enabling complete bidirectional traceability across all layers.

/// @MetaAnnotation @MetaName="SpecDetailType Vocabulary" #MetaType=Structure
#### 2.1.1 SpecDetailType

SpecDetailType defines the structural category of a SpecDetailAnnotation.  
It helps clarify what kind of implementation or behavior the detail refers to.

Available types include:

- `func`: Functional specification—describes logic, behavior, and expected outcomes.  
- `non-func`: Non-functional specification—covers static structures such as enums, data types, and configuration schemas.  
- `test`: Test specification—defines validation logic, test cases, and expected assertions.  
- `infra`: Infrastructure specification—includes database schemas, gateways, file formats, and system-level configurations.

---

## Open Questions / [NEEDS CLARIFICATION]

- **Q1 (FR-002)**: Resolved — Annotations will support both inline code annotations and document annotations (phased adoption possible).

- **Q2 (FR-003)**: Resolved — Status will be multi-stage (e.g., Implemented / In Progress / Unimplemented / Deprecated / Verified) and SpecTrail will support batch-based autonomous detection (estimated status output).

---

## Next steps

1. Please confirm the above **Q1/Q2** choices. Once confirmed, finalize the specification and proceed to break down implementation tasks.
2. Define the minimum implementation plan (MVP) to deliver priority P1 and prepare estimates.

```