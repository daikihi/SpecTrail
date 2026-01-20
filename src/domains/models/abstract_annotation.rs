/// [@st-spec-annotation-structure-file] layer: abstract, type: File, name: abstract_annotation.rs
/// This file defines the abstract-layer annotations, representing high-level conceptual units of the system.

use crate::domains::models::layer::Layer;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: AbstractName
/// This struct represents the name identifier for abstract annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractName(pub String);

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: AbstractType
/// This enum represents the type of abstract annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractType {
    Page,
    Application,
    BackgroundComponent,
    Structure, // spec.mdでStructureが使われているため追加
    Convention, // spec.mdでConventionが使われているため追加
}

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: AbstractAnnotation
/// This struct represents the abstract annotation model as described in the specification.
#[derive(Debug, Clone)]
pub struct AbstractAnnotation {
    pub name: AbstractName,
    pub r#type: AbstractType,
    pub layer: Layer,
    pub links: Vec<SpecDetailAnnotation>,
}
