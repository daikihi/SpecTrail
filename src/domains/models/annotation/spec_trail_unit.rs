/// [@st-code-domain-models-annotation-spec-trail-unit-file] layer: abstract, type: File, name: spec_trail_unit.rs, links: [@st-manual-data-model-annotation-file, @st-manual-meta-spectrail-unit]
/// This file defines the enum for annotation units, representing either code or document annotation as a traceable entity.
use super::code_annotation::CodeAnnotation;
use super::document_annotation::DocumentAnnotation;

/// [@st-code-domain-models-annotation-spec-trail-unit] layer: abstract, type: Structure, name: SpecTrailUnit, links: [@st-manual-data-model-annotation-spec-trail-unit, @st-manual-meta-spectrail-unit]
/// This enum represents a unit of annotation, either code or document, as described in the specification.
#[derive(Debug, Clone)]
pub enum SpecTrailUnit {
    Code(CodeAnnotation),
    Document(DocumentAnnotation),
}
