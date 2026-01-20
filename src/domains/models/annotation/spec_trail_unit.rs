use super::code_annotation::CodeAnnotation;
use super::document_annotation::DocumentAnnotation;

#[derive(Debug, Clone)]
pub enum SpecTrailUnit {
    Code(CodeAnnotation),
    Document(DocumentAnnotation),
}
