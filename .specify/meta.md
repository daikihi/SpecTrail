# Specification Model: Formal Definition

## 1. SpecTrailAnnotation

/// @MetaAnnotation @MetaName="Definition of SpecTrailUnit" @MetaType=Philosophy
### 1.1 SpecTrailUnit
Let SpecTrailUnit = { CodeAnnotation, DocumentAnnotation }

CodeAnnotation is an entity of SpecTrail annotation written in actual programming code.  
DocumentAnnotation is an entity of SpecTrail annotation written in specification documents.

SpecTrailUnit represents a pair of annotations—one from code, one from documentation—  
that together form a traceable unit within the SpecTrail system.

/// @MetaAnnotation @MetaName="Definition of SpecTrailAnnotation" @MetaType=Philosophy
### 1.2 SpecTrailAnnotation
Let SpecTrailAnnotation = {MetaAnnotation, AbstractAnnotation, SpecDetailAnnotation, ImplementationAnnotation}

/// @MetaAnnotation @MetaName="Definition of MetaType" @MetaType=Philosophy
#### 1.2.1 MetaAnnotation
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
#### 1.2.2 AbstractAnnotation
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
#### 1.2.3 SpecDetailAnnotation
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

#### 1.2.4 ImplementationAnnotation

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

#### 1.2.5 Annotation Trace
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
