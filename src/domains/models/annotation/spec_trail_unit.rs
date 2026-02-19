/// [@st-code-domain-models-annotation-spec-trail-unit-file] layer: abstract, type: File, name: spec_trail_unit.rs
/// [@st-manual-meta-spectrail-unit] layer: meta, type: Convention, name: SpecTrailUnit
/// This file defines the enum for annotation units, representing either code or document annotation as a traceable entity.
use super::code_annotation::CodeAnnotation;
use super::document_annotation::DocumentAnnotation;

/// [@st-code-domain-models-annotation-spec-trail-unit] layer: abstract, type: Structure, name: SpecTrailUnit
/// This enum represents a unit of annotation, either code or document, as described in the specification.
#[derive(Debug, Clone)]
pub enum SpecTrailUnit {
    Code(CodeAnnotation),
    Document(DocumentAnnotation),
}
