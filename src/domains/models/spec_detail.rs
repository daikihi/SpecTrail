use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::implementation::ImplementationAnnotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecDetailName(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecDetailType {
    Func,
    NonFunc,
    Test,
    Infra,
}

#[derive(Debug, Clone)]
pub enum SpecDetailLink {
    Abstract(Box<AbstractAnnotation>),
    Implementation(Box<ImplementationAnnotation>),
}

#[derive(Debug, Clone)]
pub struct SpecDetailAnnotation {
    pub name: SpecDetailName,
    pub r#type: SpecDetailType,
    pub links: Vec<SpecDetailLink>,
}
