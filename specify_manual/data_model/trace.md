/// [@st-manual-data-model-trace-file] layer: abstract, type: File, name: trace.md

# Trace

## Overview
Represents traceability relationships between specification and implementation.

## Structure
/// [@st-manual-data-model-trace-trace-source] layer: spec-detail, type: Func, name: TraceSource
- `TraceSource`
  - `Meta(MetaAnnotation)`
  - `Abstract(AbstractAnnotation)`
  - `SpecDetail(SpecDetailAnnotation)`
  - `Implementation(ImplementationAnnotation)`
/// [@st-manual-data-model-trace-trace-destination] layer: spec-detail, type: Func, name: TraceDestination
- `TraceDestination`
  - `Meta(MetaAnnotation)`
  - `Abstract(AbstractAnnotation)`
  - `SpecDetail(SpecDetailAnnotation)`
  - `Implementation(ImplementationAnnotation)`
/// [@st-manual-data-model-trace-trace-kind] layer: spec-detail, type: Func, name: TraceKind
- `TraceKind`
  - `Refines`
  - `Implements`
  - `Verifies`
  - `Derives`
/// [@st-manual-data-model-trace-trace] layer: spec-detail, type: Func, name: Trace
- `Trace`
  - `direction: TraceDirection` (CodeToDocument or DocumentToCode)
  - `src_file`, `dst_file`: paths of the files containing the annotations
  - `src: TraceSource`
  - `dst: TraceDestination`
  - `kind: TraceKind`

## Usage
Records typed relationships between annotations for consistency checks and visualization.

### Matching rules

- An explicit link produces a trace to an annotation in the other space only when its ID and layer match the resolved link target. For example, a document link to a Meta definition cannot reach an Implementation annotation with the same ID.
- A code annotation and a document annotation with the same ID also produce a code-to-document trace, even when their layers differ. This allows an Implementation in code to realize an Abstract requirement in a document.
- Each matching occurrence produces a trace. Both endpoints include their space, file, one-based line, layer, and ID. Repeated links between the same two occurrences with the same kind produce one trace.
- `check --warn-unimplemented` reports a non-Meta document annotation when it has no trace to code.
