use crate::domains::models::spec_detail::SpecDetailAnnotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractType {
    Page,
    Application,
    BackgroundComponent,
}

#[derive(Debug, Clone)]
pub struct AbstractAnnotation {
    pub name: AbstractName,
    pub r#type: AbstractType,
    pub links: Vec<SpecDetailAnnotation>,
}
