use crate::domains::models::layer::Layer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaName(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaType {
    Philosophy,
    Guideline,
    Convention,
    Structure,
    Rule,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaAnnotation {
    pub name: MetaName,
    pub r#type: MetaType,
    pub layer: Layer,
    pub links: Vec<MetaAnnotation>,
}
