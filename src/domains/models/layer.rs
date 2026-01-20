/// [@st-spec-annotation-structure-file] layer: abstract, type: File, name: layer.rs
/// This file defines the Layer enumeration, classifying annotations into meta, abstract, spec-detail, and implementation strata.

/// [@st-spec-data-model] layer: abstract, type: Structure, name: Layer
/// This enum represents the layer concept as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Meta,
    Abstract,
    SpecDetail,
    Implementation,
}
