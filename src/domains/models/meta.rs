/// [@st-code-domain-models-meta-file] layer: abstract, type: File, name: meta.rs
/// [@st-manual-meta-model-doc] layer: meta, type: Philosophy, name: Specification Model: Formal Definition
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

impl std::str::FromStr for MetaType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Philosophy" => Ok(MetaType::Philosophy),
            "Guideline" => Ok(MetaType::Guideline),
            "Convention" => Ok(MetaType::Convention),
            "Structure" => Ok(MetaType::Structure),
            "Rule" => Ok(MetaType::Rule),
            _ => Err(format!("Unknown MetaType: {}", s)),
        }
    }
}

/// [@st-code-domain-models-meta-meta-annotation] layer: abstract, type: Structure, name: MetaAnnotation
/// This struct represents the meta annotation model as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaAnnotation {
    pub id: MetaAnnotationId,
    pub name: MetaName,
    pub r#type: Option<MetaType>,
    pub layer: Layer,
    pub links: Vec<MetaAnnotation>,
}
