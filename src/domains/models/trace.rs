use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;

#[derive(Debug, Clone)]
pub enum TraceSource {
    Abstract(AbstractAnnotation),
    SpecDetail(SpecDetailAnnotation),
    Implementation(ImplementationAnnotation),
}

#[derive(Debug, Clone)]
pub enum TraceDestination {
    Abstract(AbstractAnnotation),
    SpecDetail(SpecDetailAnnotation),
    Implementation(ImplementationAnnotation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceKind {
    Refines,
    Implements,
    Verifies,
    Derives,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub src: TraceSource,
    pub dst: TraceDestination,
    pub kind: TraceKind,
}
