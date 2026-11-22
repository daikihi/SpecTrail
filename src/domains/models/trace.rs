/// [@st-code-domain-models-trace-file] layer: abstract, type: File, name: trace.rs, links: [@st-manual-data-model-trace-file, @st-manual-meta-model-doc]
/// This file defines the traceability structures, establishing semantic correspondence between different annotation layers.
use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;
use crate::domains::models::layer::Layer;
use crate::domains::models::line_number::LineNumber;
use crate::domains::models::meta::MetaAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

/// [@st-code-domain-models-trace-trace-source] layer: spec-detail, type: Func, name: TraceSource
/// This enum represents the source of a trace link as described in the specification.
#[derive(Debug, Clone)]
pub enum TraceSource {
    Meta(MetaAnnotation),
    Abstract(AbstractAnnotation),
    SpecDetail(SpecDetailAnnotation),
    Implementation(ImplementationAnnotation),
}

impl TraceSource {
    pub fn id(&self) -> &str {
        match self {
            TraceSource::Meta(a) => &a.id.0,
            TraceSource::Abstract(a) => &a.id.0,
            TraceSource::SpecDetail(a) => &a.id.0,
            TraceSource::Implementation(a) => &a.id.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TraceSource::Meta(a) => &a.name.0,
            TraceSource::Abstract(a) => &a.name.0,
            TraceSource::SpecDetail(a) => &a.name.0,
            TraceSource::Implementation(a) => &a.name.0,
        }
    }

    pub fn layer(&self) -> Layer {
        match self {
            TraceSource::Meta(a) => a.layer,
            TraceSource::Abstract(a) => a.layer,
            TraceSource::SpecDetail(a) => a.layer,
            TraceSource::Implementation(a) => a.layer,
        }
    }

    pub fn line(&self) -> LineNumber {
        match self {
            TraceSource::Meta(a) => a.line,
            TraceSource::Abstract(a) => a.line,
            TraceSource::SpecDetail(a) => a.line,
            TraceSource::Implementation(a) => a.line,
        }
    }
}

/// [@st-code-domain-models-trace-trace-destination] layer: spec-detail, type: Func, name: TraceDestination
/// This enum represents the destination of a trace link as described in the specification.
#[derive(Debug, Clone)]
pub enum TraceDestination {
    Meta(MetaAnnotation),
    Abstract(AbstractAnnotation),
    SpecDetail(SpecDetailAnnotation),
    Implementation(ImplementationAnnotation),
}

impl TraceDestination {
    pub fn id(&self) -> &str {
        match self {
            TraceDestination::Meta(a) => &a.id.0,
            TraceDestination::Abstract(a) => &a.id.0,
            TraceDestination::SpecDetail(a) => &a.id.0,
            TraceDestination::Implementation(a) => &a.id.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            TraceDestination::Meta(a) => &a.name.0,
            TraceDestination::Abstract(a) => &a.name.0,
            TraceDestination::SpecDetail(a) => &a.name.0,
            TraceDestination::Implementation(a) => &a.name.0,
        }
    }

    pub fn layer(&self) -> Layer {
        match self {
            TraceDestination::Meta(a) => a.layer,
            TraceDestination::Abstract(a) => a.layer,
            TraceDestination::SpecDetail(a) => a.layer,
            TraceDestination::Implementation(a) => a.layer,
        }
    }

    pub fn line(&self) -> LineNumber {
        match self {
            TraceDestination::Meta(a) => a.line,
            TraceDestination::Abstract(a) => a.line,
            TraceDestination::SpecDetail(a) => a.line,
            TraceDestination::Implementation(a) => a.line,
        }
    }
}

impl From<TraceSource> for TraceDestination {
    fn from(annotation: TraceSource) -> Self {
        match annotation {
            TraceSource::Meta(a) => Self::Meta(a),
            TraceSource::Abstract(a) => Self::Abstract(a),
            TraceSource::SpecDetail(d) => Self::SpecDetail(d),
            TraceSource::Implementation(i) => Self::Implementation(i),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceDirection {
    CodeToDocument,
    DocumentToCode,
}

impl TraceDirection {
    pub fn spaces(self) -> (&'static str, &'static str) {
        match self {
            Self::CodeToDocument => ("code", "document"),
            Self::DocumentToCode => ("document", "code"),
        }
    }
}

/// [@st-code-domain-models-trace-trace-kind] layer: spec-detail, type: Func, name: TraceKind
/// This enum represents the kind of traceability relationship as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TraceKind {
    Refines,
    Implements,
    Verifies,
    Derives,
}

impl std::fmt::Display for TraceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceKind::Refines => write!(f, "Refines"),
            TraceKind::Implements => write!(f, "Implements"),
            TraceKind::Verifies => write!(f, "Verifies"),
            TraceKind::Derives => write!(f, "Derives"),
        }
    }
}

/// [@st-code-domain-models-trace-trace] layer: spec-detail, type: Func, name: Trace
/// This struct represents the traceability mapping between specification and implementation.
#[derive(Debug, Clone)]
pub struct Trace {
    pub direction: TraceDirection,
    pub src_file: String,
    pub dst_file: String,
    pub src: TraceSource,
    pub dst: TraceDestination,
    pub kind: TraceKind,
}
