/// [@st-manual-data-model-trace-file] layer: abstract, type: File, name: trace.md

# Trace

## Overview
Represents traceability relationships between specification and implementation.

## Structure
/// [@st-manual-data-model-trace-trace-source] layer: spec-detail, type: Func, name: TraceSource
- `TraceSource`
  - `Abstract(AbstractAnnotation)`
  - `SpecDetail(SpecDetailAnnotation)`
  - `Implementation(ImplementationAnnotation)`
/// [@st-manual-data-model-trace-trace-destination] layer: spec-detail, type: Func, name: TraceDestination
- `TraceDestination`
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
  - `src: TraceSource`
  - `dst: TraceDestination`
  - `kind: TraceKind`

## Usage
Records typed relationships between annotations for consistency checks and visualization.
