/// [@st-code-domain-models-layer-file] layer: abstract, type: File, name: layer.rs
/// [@st-manual-meta-model-doc] layer: meta, type: Philosophy, name: Specification Model: Formal Definition
/// This file defines the Layer enumeration, classifying annotations into meta, abstract, spec-detail, and implementation strata.

/// [@st-code-domain-models-layer-layer] layer: abstract, type: Structure, name: Layer
/// This enum represents the layer concept as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Meta,
    Abstract,
    SpecDetail,
    Implementation,
}
