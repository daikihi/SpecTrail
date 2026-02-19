use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;
/// [@st-code-domain-models-annotation-code-annotation-file] layer: abstract, type: File, name: code_annotation.rs
/// [@st-manual-meta-model-doc] layer: meta, type: Philosophy, name: Specification Model: Formal Definition
/// This file defines the data structure for code-space annotations, which aggregate meta, abstract, spec detail, and implementation annotations for code artifacts.
use crate::domains::models::meta::MetaAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

/// [@st-code-domain-models-annotation-code-annotation] layer: abstract, type: Structure, name: CodeAnnotation
/// This struct represents a code-space annotation, aggregating all annotation layers for code artifacts as described in the specification.
#[derive(Debug, Clone)]
pub struct CodeAnnotation {
    pub metas: Vec<MetaAnnotation>,
    pub abstracts: Vec<AbstractAnnotation>,
    pub details: Vec<SpecDetailAnnotation>,
    pub implementations: Vec<ImplementationAnnotation>,
}
