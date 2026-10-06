/// [@st-code-domain-models-annotation-spec-trail-unit-file] layer: abstract, type: File, name: spec_trail_unit.rs
/// [@st-manual-meta-spectrail-unit] layer: meta, type: Convention, name: SpecTrailUnit
/// This file defines the enum for annotation units, representing one code or document annotation aggregate.
use super::code_annotation::CodeAnnotation;
use super::document_annotation::DocumentAnnotation;

/// [@st-code-domain-models-annotation-spec-trail-unit] layer: abstract, type: Structure, name: SpecTrailUnit
/// This enum represents one tagged annotation aggregate from either the code or document domain.
#[derive(Debug, Clone)]
pub enum SpecTrailUnit {
    Code(CodeAnnotation),
    Document(DocumentAnnotation),
}
