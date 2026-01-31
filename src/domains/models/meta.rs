/// [@st-code-domain-models-meta-file] layer: abstract, type: File, name: meta.rs
/// This file defines the meta-layer annotations, which represent high-level philosophies, guidelines, and rules.

use crate::domains::models::layer::Layer;

/// [@st-code-domain-models-meta-meta-annotation-id] layer: abstract, type: Structure, name: MetaAnnotationId
/// This struct represents the unique identifier for a meta annotation (id = tag) as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaAnnotationId(pub String);

/// [@st-code-domain-models-meta-meta-name] layer: abstract, type: Structure, name: MetaName
/// This struct represents the name identifier for meta annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaName(pub String);

/// [@st-code-domain-models-meta-meta-type] layer: abstract, type: Structure, name: MetaType
/// This enum represents the type of meta annotation as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaType {
    Philosophy,
    Guideline,
    Convention,
    Structure,
    Rule,
}

/// [@st-code-domain-models-meta-meta-annotation] layer: abstract, type: Structure, name: MetaAnnotation
/// This struct represents the meta annotation model as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaAnnotation {
    pub id: MetaAnnotationId,
    pub name: MetaName,
    pub r#type: MetaType,
    pub layer: Layer,
    pub links: Vec<MetaAnnotation>,
}
