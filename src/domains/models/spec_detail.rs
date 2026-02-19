use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;
/// [@st-code-domain-models-spec-detail-file] layer: abstract, type: File, name: spec_detail.rs
/// [@st-manual-meta-model-doc] layer: meta, type: Philosophy, name: Specification Model: Formal Definition
/// This file defines the spec-detail-layer annotations, representing concrete functional or structural specifications.
use crate::domains::models::layer::Layer;
use std::str::FromStr;

/// [@st-code-domain-models-spec-detail-spec-detail-annotation-id] layer: abstract, type: Structure, name: SpecDetailAnnotationId
/// This struct represents the unique identifier for a spec detail annotation (id = tag) as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecDetailAnnotationId(pub String);

/// [@st-code-domain-models-spec-detail-spec-detail-name] layer: abstract, type: Structure, name: SpecDetailName
/// This struct represents the name identifier for spec detail annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecDetailName(pub String);

/// [@st-code-domain-models-spec-detail-spec-detail-type] layer: abstract, type: Structure, name: SpecDetailType
/// This enum represents the type of spec detail annotation as described in the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecDetailType {
    Func,
    NonFunc,
    Test,
    Infra,
    Convention,
    Rule,
}

impl std::str::FromStr for SpecDetailType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Func" => Ok(SpecDetailType::Func),
            "NonFunc" => Ok(SpecDetailType::NonFunc),
            "Test" => Ok(SpecDetailType::Test),
            "Infra" => Ok(SpecDetailType::Infra),
            "Convention" => Ok(SpecDetailType::Convention),
            "Rule" => Ok(SpecDetailType::Rule),
            _ => Err(format!("Unknown SpecDetailType: {}", s)),
        }
    }
}

/// [@st-code-domain-models-spec-detail-spec-detail-link] layer: abstract, type: Structure, name: SpecDetailLink
/// This enum represents the link type for spec detail annotation as described in the specification.
#[derive(Debug, Clone)]
pub enum SpecDetailLink {
    Abstract(Box<AbstractAnnotation>),
    Implementation(Box<ImplementationAnnotation>),
}

/// [@st-code-domain-models-spec-detail-spec-detail-annotation] layer: abstract, type: Structure, name: SpecDetailAnnotation
/// This struct represents the spec detail annotation model as described in the specification.
#[derive(Debug, Clone)]
pub struct SpecDetailAnnotation {
    pub id: SpecDetailAnnotationId,
    pub name: SpecDetailName,
    pub r#type: SpecDetailType,
    pub layer: Layer,
    pub links: Vec<SpecDetailLink>,
}
