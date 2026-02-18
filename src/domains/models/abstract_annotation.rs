/// [@st-code-domain-models-abstract-annotation-file] layer: abstract, type: File, name: abstract_annotation.rs
/// This file defines the abstract-layer annotations, representing high-level conceptual units of the system.
use crate::domains::models::layer::Layer;
use crate::domains::models::spec_detail::SpecDetailAnnotation;
use std::str::FromStr;

/// [@st-code-domain-models-abstract-annotation-abstract-annotation-id] layer: abstract, type: Structure, name: AbstractAnnotationId
/// This struct represents the unique identifier for an abstract annotation (id = tag) as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractAnnotationId(pub String);

/// [@st-code-domain-models-abstract-annotation-abstract-name] layer: abstract, type: Structure, name: AbstractName
/// This struct represents the name identifier for abstract annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractName(pub String);

/// [@st-code-domain-models-abstract-annotation-abstract-type] layer: abstract, type: Structure, name: AbstractType
/// This enum represents the type of abstract annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractType {
    Page,
    Application,
    BackgroundComponent,
    Structure,
    Convention,
    Philosophy,
    Guideline,
}

impl std::str::FromStr for AbstractType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Page" => Ok(AbstractType::Page),
            "Application" => Ok(AbstractType::Application),
            "BackgroundComponent" => Ok(AbstractType::BackgroundComponent),
            "Structure" => Ok(AbstractType::Structure),
            "Convention" => Ok(AbstractType::Convention),
            "Philosophy" => Ok(AbstractType::Philosophy),
            "Guideline" => Ok(AbstractType::Guideline),
            "File" => Ok(AbstractType::Structure), // 便宜上
            _ => Err(format!("Unknown AbstractType: {}", s)),
        }
    }
}

/// [@st-code-domain-models-abstract-annotation-abstract-annotation] layer: abstract, type: Structure, name: AbstractAnnotation
/// This struct represents the abstract annotation model as described in the specification.
#[derive(Debug, Clone)]
pub struct AbstractAnnotation {
    pub id: AbstractAnnotationId,
    pub name: AbstractName,
    pub r#type: AbstractType,
    pub layer: Layer,
    pub links: Vec<SpecDetailAnnotation>,
}
