/// [@st-code-domain-models-layer-file] layer: abstract, type: File, name: layer.rs, links: [@st-manual-data-model-layer-file, @st-manual-meta-model-doc]
/// This file defines the Layer enumeration, classifying annotations into meta, abstract, spec-detail, and implementation strata.

/// [@st-code-domain-models-layer-layer] layer: abstract, type: Structure, name: Layer, links: [@st-manual-data-model-layer-layer]
/// This enum represents the layer concept as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer {
    Meta,
    Abstract,
    SpecDetail,
    Implementation,
}
