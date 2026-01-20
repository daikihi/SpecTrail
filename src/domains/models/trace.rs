/// [@st-spec-annotation-structure-file] layer: abstract, type: File, name: trace.rs
/// This file defines the traceability structures, establishing semantic correspondence between different annotation layers.

use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;

/// [@st-spec-mapping-match-spec-to-code] layer: spec-detail, type: Func, name: TraceSource
/// This enum represents the source of a trace link as described in the specification.
#[derive(Debug, Clone)]
pub enum TraceSource {
    Abstract(AbstractAnnotation),
    SpecDetail(SpecDetailAnnotation),
    Implementation(ImplementationAnnotation),
}

/// [@st-spec-mapping-match-spec-to-code] layer: spec-detail, type: Func, name: TraceDestination
/// This enum represents the destination of a trace link as described in the specification.
#[derive(Debug, Clone)]
pub enum TraceDestination {
    Abstract(AbstractAnnotation),
    SpecDetail(SpecDetailAnnotation),
    Implementation(ImplementationAnnotation),
}

/// [@st-spec-mapping-match-spec-to-code] layer: spec-detail, type: Func, name: TraceKind
/// This enum represents the kind of traceability relationship as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceKind {
    Refines,
    Implements,
    Verifies,
    Derives,
}

/// [@st-spec-mapping-match-spec-to-code] layer: spec-detail, type: Func, name: Trace
/// This struct represents the traceability mapping between specification and implementation.
#[derive(Debug, Clone)]
pub struct Trace {
    pub src: TraceSource,
    pub dst: TraceDestination,
    pub kind: TraceKind,
}
