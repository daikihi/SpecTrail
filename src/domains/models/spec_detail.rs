/// [@st-spec-annotation-structure-file] layer: abstract, type: File, name: spec_detail.rs
/// This file defines the spec-detail-layer annotations, representing concrete functional or structural specifications.

use crate::domains::models::layer::Layer;
use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: SpecDetailName
/// This struct represents the name identifier for spec detail annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecDetailName(pub String);

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: SpecDetailType
/// This enum represents the type of spec detail annotation as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecDetailType {
    Func,
    NonFunc,
    Test,
    Infra,
    Convention, // spec.mdでConventionが使われているため追加
    Rule, // spec.mdでRuleが使われているため追加
}

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: SpecDetailLink
/// This enum represents the link type for spec detail annotation as described in the specification.
#[derive(Debug, Clone)]
pub enum SpecDetailLink {
    Abstract(Box<AbstractAnnotation>),
    Implementation(Box<ImplementationAnnotation>),
}

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: SpecDetailAnnotation
/// This struct represents the spec detail annotation model as described in the specification.
#[derive(Debug, Clone)]
pub struct SpecDetailAnnotation {
    pub name: SpecDetailName,
    pub r#type: SpecDetailType,
    pub layer: Layer,
    pub links: Vec<SpecDetailLink>,
}
