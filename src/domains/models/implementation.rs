/// [@st-code-domain-models-implementation-file] layer: abstract, type: File, name: implementation.rs
/// This file defines the implementation-layer annotations, describing how specifications are realized at the technical level.

use crate::domains::models::layer::Layer;
use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

/// [@st-code-domain-models-implementation-implementation-annotation-id] layer: abstract, type: Structure, name: ImplementationAnnotationId
/// This struct represents the unique identifier for an implementation annotation (id = tag) as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationAnnotationId(pub String);

/// [@st-code-domain-models-implementation-implementation-spec-name] layer: abstract, type: Structure, name: ImplementationSpecName
/// This struct represents the name identifier for implementation annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationSpecName(pub String);

/// [@st-code-domain-models-implementation-implementation-type] layer: abstract, type: Structure, name: ImplementationType
/// This enum represents the type of implementation annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationType {
    DatabaseSchema,
    DaoRepository,
    DomainEntity,
    ExternalApiGateway,
    WebInterfaceDataModel,
}

/// [@st-code-domain-models-implementation-implementation-link] layer: abstract, type: Structure, name: ImplementationLink
/// This enum represents the link type for implementation annotation as described in the specification.
#[derive(Debug, Clone)]
pub enum ImplementationLink {
    SpecDetail(Box<SpecDetailAnnotation>),
    Abstract(Box<AbstractAnnotation>),
}

/// [@st-code-domain-models-implementation-implementation-artifact] layer: abstract, type: Structure, name: ImplementationArtifact
/// This struct represents the artifact for implementation annotation as described in the specification.
#[derive(Debug, Clone)]
pub struct ImplementationArtifact(pub String);

/// [@st-code-domain-models-implementation-implementation-status] layer: abstract, type: Structure, name: ImplementationStatus
/// This enum represents the status for implementation annotation as described in the specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationStatus {
    Planned,
    InProgress,
    Completed,
}

/// [@st-code-domain-models-implementation-implementation-annotation] layer: abstract, type: Structure, name: ImplementationAnnotation
/// This struct represents the implementation annotation model as described in the specification.
#[derive(Debug, Clone)]
pub struct ImplementationAnnotation {
    pub id: ImplementationAnnotationId,
    pub name: ImplementationSpecName,
    pub r#type: ImplementationType,
    pub layer: Layer,
    pub links: Vec<ImplementationLink>,
    pub artifact: ImplementationArtifact,
    pub status: ImplementationStatus,
}
