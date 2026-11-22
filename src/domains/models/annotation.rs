pub mod code_annotation;
pub mod document_annotation;
pub mod meta_diff;
pub mod spec_trail_unit;

use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;
use crate::domains::models::meta::MetaAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

/// Trait abstracting over DocumentAnnotation and CodeAnnotation file containers.
pub trait AnnotationFile {
    fn source_file(&self) -> &str;
    fn metas(&self) -> &[MetaAnnotation];
    fn abstracts(&self) -> &[AbstractAnnotation];
    fn details(&self) -> &[SpecDetailAnnotation];
    fn implementations(&self) -> &[ImplementationAnnotation];

    fn total_count(&self) -> usize {
        self.metas().len()
            + self.abstracts().len()
            + self.details().len()
            + self.implementations().len()
    }
}

impl AnnotationFile for DocumentAnnotation {
    fn source_file(&self) -> &str {
        &self.source_file
    }
    fn metas(&self) -> &[MetaAnnotation] {
        &self.metas
    }
    fn abstracts(&self) -> &[AbstractAnnotation] {
        &self.abstracts
    }
    fn details(&self) -> &[SpecDetailAnnotation] {
        &self.details
    }
    fn implementations(&self) -> &[ImplementationAnnotation] {
        &self.implementations
    }
}

impl AnnotationFile for CodeAnnotation {
    fn source_file(&self) -> &str {
        &self.source_file
    }
    fn metas(&self) -> &[MetaAnnotation] {
        &self.metas
    }
    fn abstracts(&self) -> &[AbstractAnnotation] {
        &self.abstracts
    }
    fn details(&self) -> &[SpecDetailAnnotation] {
        &self.details
    }
    fn implementations(&self) -> &[ImplementationAnnotation] {
        &self.implementations
    }
}
