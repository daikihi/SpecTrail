/// [@st-spec-annotation-structure-file] layer: abstract, type: File, name: meta.rs
/// This file defines the meta-layer annotations, which represent high-level philosophies, guidelines, and rules.

use crate::domains::models::layer::Layer;

/// [@st-spec-data-model] layer: abstract, type: Structure, name: MetaName
/// This struct represents the name identifier for meta annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaName(pub String);

/// [@st-spec-data-model] layer: abstract, type: Structure, name: MetaType
/// This enum represents the type of meta annotation as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaType {
    Philosophy,
    Guideline,
    Convention,
    Structure,
    Rule,
}

/// [@st-spec-data-model] layer: abstract, type: Structure, name: MetaAnnotation
/// This struct represents the meta annotation model as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaAnnotation {
    pub name: MetaName,
    pub r#type: MetaType,
    pub layer: Layer,
    pub links: Vec<MetaAnnotation>,
}
