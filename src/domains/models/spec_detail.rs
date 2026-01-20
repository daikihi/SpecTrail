use crate::domains::models::layer::Layer;
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
    Convention, // spec.mdでConventionが使われているため追加
    Rule, // spec.mdでRuleが使われているため追加
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
    pub layer: Layer,
    pub links: Vec<SpecDetailLink>,
}
