use crate::domains::models::layer::Layer;
use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationSpecName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationType {
    DatabaseSchema,
    DaoRepository,
    DomainEntity,
    ExternalApiGateway,
    WebInterfaceDataModel,
}

#[derive(Debug, Clone)]
pub enum ImplementationLink {
    SpecDetail(Box<SpecDetailAnnotation>),
    Abstract(Box<AbstractAnnotation>),
}

#[derive(Debug, Clone)]
pub struct ImplementationArtifact(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationStatus {
    Planned,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub struct ImplementationAnnotation {
    pub name: ImplementationSpecName,
    pub r#type: ImplementationType,
    pub layer: Layer,
    pub links: Vec<ImplementationLink>,
    pub artifact: ImplementationArtifact,
    pub status: ImplementationStatus,
}
