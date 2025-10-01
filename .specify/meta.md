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

#### 1.2.2 AbstractAnnotation

#### 1.2.3 SpecDetailAnnotation

#### 1.2.4 ImplementationAnnotation











-----WIP
----


/// @MetaAnnotation ( @Name "Definition of Name", @Type Data Structure, Layer null)
Each specification should have a clear and readable name to identify it. While many software systems use sequential numbers or random UUIDs for identification, SpecTrail takes a different approach.

SpecTrail annotations are applied not only to implementation code, but also to specification documents. Therefore, using opaque identifiers like UUIDs makes it difficult to understand the meaning and context of a specification when reading the document.

To ensure clarity and traceability, SpecTrail uses human-readable names as identifiers for specifications.

define type

define layer


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
