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

For each a ∈ A: a = {na, sa}, where

na ∈ AbstractName

sa ⊆ SpecDetailName

Here, na identifies the individual AbstractAnnotation, and sa is the set of SpecDetailAnnotations that belong to a.
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
d = {nd, ad, td, impls}, where  
- nd ∈ SpecDetailName  
- ad ∈ AbstractName  
- td ∈ SpecDetailType  
- impls ⊆ ImplName  

Here, `nd` identifies the individual SpecDetailAnnotation,  
`ad` links it to its parent AbstractAnnotation via `@spec`,  
`td` defines the type of specification—such as `func`, `non-func`, `test`, or `infra`,  
and `impls` is the set of ImplementationAnnotations that realize this detail.
```

#### 1.2.4 ImplementationAnnotation








/// @MetaAnnotation @MetaName="SpecDetailType Vocabulary" #MetaType=Structure
#### 2.1.1 SpecDetailType

SpecDetailType defines the structural category of a SpecDetailAnnotation.  
It helps clarify what kind of implementation or behavior the detail refers to.

Available types include:

- `func`: Functional specification—describes logic, behavior, and expected outcomes.  
- `non-func`: Non-functional specification—covers static structures such as enums, data types, and configuration schemas.  
- `test`: Test specification—defines validation logic, test cases, and expected assertions.  
- `infra`: Infrastructure specification—includes database schemas, gateways, file formats, and system-level configurations.



-----WIP
----


SpecTrailAnnotation consists of two disjoint sets:

C = { c₁, c₂, ..., cₙ } : the set of all possible CodeAnnotations

D = { d₁, d₂, ..., dₘ } : the set of all possible DocumentAnnotations

Constraint: CNc ∩ DNd = ∅. // complete s

Each annotation a ∈ (C ∪ D) is defined as a structured tuple:

a = { spec_id, type, layer }

Where:

spec_id ∈ SpecID

type ∈ Type

layer ∈ Layer

2. Annotation Matching
Let M ⊆ C × D be the set of matched annotations.

For any pair (c, d) ∈ M:

c.spec_id = d.spec_id

Then we define:

UnmatchedCode = C \ { c | ∃ d ∈ D, (c, d) ∈ M }

UnmatchedDocument = D \ { d | ∃ c ∈ C, (c, d) ∈ M }

These subsets represent implementation without specification, and specification without implementation, respectively.

3. Annotation Graph
Let G = (V, E) be a directed graph where:

V = C ∪ D

E = { (d → c) | (c, d) ∈ M }

This graph represents the traceability between documentation and code. Disconnected nodes in G correspond to unmatched annotations.
