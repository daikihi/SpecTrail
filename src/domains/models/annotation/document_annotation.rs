use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;
/// [@st-code-domain-models-annotation-document-annotation-file] layer: abstract, type: File, name: document_annotation.rs, links: [@st-manual-data-model-annotation-file, @st-manual-meta-model-doc]
/// This file defines the data structure for document-space annotations, aggregating meta, abstract, spec detail, and implementation annotations for document artifacts.
use crate::domains::models::meta::MetaAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

/// [@st-code-domain-models-annotation-document-annotation] layer: abstract, type: Structure, name: DocumentAnnotation, links: [@st-manual-data-model-annotation-document-annotation]
/// This struct represents a document-space annotation, aggregating all annotation layers for document artifacts as described in the specification.
#[derive(Debug, Clone)]
pub struct DocumentAnnotation {
    pub source_file: String,
    pub metas: Vec<MetaAnnotation>,
    pub abstracts: Vec<AbstractAnnotation>,
    pub details: Vec<SpecDetailAnnotation>,
    pub implementations: Vec<ImplementationAnnotation>,
}
