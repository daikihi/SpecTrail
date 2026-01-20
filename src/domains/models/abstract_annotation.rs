use crate::domains::models::layer::Layer;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractType {
    Page,
    Application,
    BackgroundComponent,
    Structure, // spec.mdでStructureが使われているため追加
    Convention, // spec.mdでConventionが使われているため追加
}

#[derive(Debug, Clone)]
pub struct AbstractAnnotation {
    pub name: AbstractName,
    pub r#type: AbstractType,
    pub layer: Layer,
    pub links: Vec<SpecDetailAnnotation>,
}
