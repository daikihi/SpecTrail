use crate::domains::models::meta::MetaAnnotation;
use crate::domains::models::abstract_annotation::AbstractAnnotation;
use crate::domains::models::spec_detail::SpecDetailAnnotation;

#[derive(Debug, Clone)]
pub struct CodeAnnotation {
    pub metas: Vec<MetaAnnotation>,
    pub abstracts: Vec<AbstractAnnotation>,
    pub details: Vec<SpecDetailAnnotation>,
}
