/// [@st-code-domain-services-annotation-resolver-file] layer: abstract, type: File, name: resolver.rs
/// This file provides the resolver that transforms RawAnnotations into fully typed domain models.
use crate::domains::models::abstract_annotation::{
    AbstractAnnotation, AbstractAnnotationId, AbstractName, AbstractType,
};
use crate::domains::models::implementation::{
    ImplementationAnnotation, ImplementationAnnotationId, ImplementationArtifact,
    ImplementationLink, ImplementationSpecName, ImplementationStatus, ImplementationType,
};
use crate::domains::models::layer::Layer;
use crate::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName, MetaType};
use crate::domains::models::spec_detail::{
    SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailLink, SpecDetailName, SpecDetailType,
};
use crate::domains::services::annotation::raw_annotation::RawAnnotation;
use std::collections::HashMap;
use std::str::FromStr;

/// [@st-code-domain-services-annotation-resolver-annotation-resolver] layer: abstract, type: Structure, name: AnnotationResolver
/// Responsible for validating layers/types and resolving links between annotations.
pub struct AnnotationResolver;

/// [@st-code-domain-services-annotation-resolver-resolved-annotation] layer: abstract, type: Structure, name: ResolvedAnnotation
/// A domain annotation that has been resolved and categorized by its layer.
#[derive(Debug)]
pub enum ResolvedAnnotation {
    Meta(MetaAnnotation, String),
    Abstract(AbstractAnnotation, String),
    SpecDetail(SpecDetailAnnotation, String),
    Implementation(ImplementationAnnotation, String),
}

impl ResolvedAnnotation {
    /* Returns the source file path where this annotation was found. */
    pub fn source_file(&self) -> &str {
        match self {
            ResolvedAnnotation::Meta(_, f) => f,
            ResolvedAnnotation::Abstract(_, f) => f,
            ResolvedAnnotation::SpecDetail(_, f) => f,
            ResolvedAnnotation::Implementation(_, f) => f,
        }
    }
}

/// [@st-code-domain-services-annotation-resolver-resolve-result] layer: abstract, type: Structure, name: ResolveResult
/// Holds the results of a resolution operation, including any resolved annotations and warnings.
pub struct ResolveResult {
    pub annotations: Vec<ResolvedAnnotation>,
    pub warnings: Vec<ResolveWarning>,
}

/// [@st-code-domain-services-annotation-resolver-resolve-warning] layer: abstract, type: Structure, name: ResolveWarning
/// Represents an issue found during resolution, such as a missing link target or invalid layer name.
#[derive(Debug)]
pub struct ResolveWarning {
    pub source_annotation_id: String,
    pub message: String,
}

impl AnnotationResolver {
    /* Resolves a list of RawAnnotations into a ResolveResult.
     *
     * This method performs two main steps:
     * 1. Building an index of all annotations by ID to allow for link resolution.
     * 2. Iterating through each RawAnnotation, converting its string fields to domain Enums,
     *    and resolving its links using the index. */
    pub fn resolve(raw_annotations: Vec<RawAnnotation>) -> ResolveResult {
        let mut annotations = Vec::new();
        let mut warnings = Vec::new();

        let index: HashMap<String, &RawAnnotation> =
            raw_annotations.iter().map(|a| (a.id.clone(), a)).collect();

        for raw in &raw_annotations {
            let layer = match raw.layer.as_str() {
                "meta" => Layer::Meta,
                "abstract" => Layer::Abstract,
                "spec-detail" => Layer::SpecDetail,
                "implementation" => Layer::Implementation,
                _ => {
                    warnings.push(ResolveWarning {
                        source_annotation_id: raw.id.clone(),
                        message: format!("Unknown layer '{}', skipping @{}", raw.layer, raw.id),
                    });
                    continue;
                }
            };

            match layer {
                Layer::Meta => {
                    let meta_type = match MetaType::from_str(&raw.annotation_type) {
                        Ok(t) => Some(t),
                        Err(_) => {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Unknown MetaType '{}' for @{}",
                                    raw.annotation_type, raw.id
                                ),
                            });
                            None
                        }
                    };

                    let mut links = Vec::new();
                    for link_id in &raw.links {
                        let link_id: &String = link_id;
                        let link_id_stripped = link_id.strip_prefix('@').unwrap_or(link_id);
                        if let Some(target) = index.get(link_id_stripped) {
                            links.push(MetaAnnotation {
                                id: MetaAnnotationId(target.id.clone()),
                                name: MetaName(target.name.clone()),
                                r#type: MetaType::from_str(&target.annotation_type).ok(),
                                layer: Layer::Meta,
                                links: vec![],
                            });
                        } else {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Link target '{}' not found (referenced from @{})",
                                    link_id, raw.id
                                ),
                            });
                        }
                    }

                    annotations.push(ResolvedAnnotation::Meta(
                        MetaAnnotation {
                            id: MetaAnnotationId(raw.id.clone()),
                            name: MetaName(raw.name.clone()),
                            r#type: meta_type,
                            layer: Layer::Meta,
                            links,
                        },
                        raw.source_file.clone(),
                    ));
                }
                Layer::Abstract => {
                    let abs_type = match AbstractType::from_str(&raw.annotation_type) {
                        Ok(t) => Some(t),
                        Err(_) => {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Unknown AbstractType '{}' for @{}",
                                    raw.annotation_type, raw.id
                                ),
                            });
                            None
                        }
                    };

                    let mut links = Vec::new();
                    for link_id in &raw.links {
                        let link_id: &String = link_id;
                        let link_id_stripped = link_id.strip_prefix('@').unwrap_or(link_id);
                        if let Some(target) = index.get(link_id_stripped) {
                            links.push(SpecDetailAnnotation {
                                id: SpecDetailAnnotationId(target.id.clone()),
                                name: SpecDetailName(target.name.clone()),
                                r#type: SpecDetailType::from_str(&target.annotation_type).ok(),
                                layer: Layer::SpecDetail,
                                links: vec![],
                            });
                        } else {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Link target '{}' not found (referenced from @{})",
                                    link_id, raw.id
                                ),
                            });
                        }
                    }

                    annotations.push(ResolvedAnnotation::Abstract(
                        AbstractAnnotation {
                            id: AbstractAnnotationId(raw.id.clone()),
                            name: AbstractName(raw.name.clone()),
                            r#type: abs_type,
                            layer: Layer::Abstract,
                            links,
                        },
                        raw.source_file.clone(),
                    ));
                }
                Layer::SpecDetail => {
                    let detail_type = match SpecDetailType::from_str(&raw.annotation_type) {
                        Ok(t) => Some(t),
                        Err(_) => {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Unknown SpecDetailType '{}' for @{}",
                                    raw.annotation_type, raw.id
                                ),
                            });
                            None
                        }
                    };

                    let mut links = Vec::new();
                    for link_id in &raw.links {
                        let link_id: &String = link_id;
                        let link_id_stripped = link_id.strip_prefix('@').unwrap_or(link_id);
                        if let Some(target) = index.get(link_id_stripped) {
                            /* Currently treating all links from SpecDetail as Abstract links
                                (matching existing scanner.rs behavior). */
                            links.push(SpecDetailLink::Abstract(Box::new(AbstractAnnotation {
                                id: AbstractAnnotationId(target.id.clone()),
                                name: AbstractName(target.name.clone()),
                                r#type: AbstractType::from_str(&target.annotation_type).ok(),
                                layer: Layer::Abstract,
                                links: vec![],
                            })));
                        } else {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Link target '{}' not found (referenced from @{})",
                                    link_id, raw.id
                                ),
                            });
                        }
                    }

                    annotations.push(ResolvedAnnotation::SpecDetail(
                        SpecDetailAnnotation {
                            id: SpecDetailAnnotationId(raw.id.clone()),
                            name: SpecDetailName(raw.name.clone()),
                            r#type: detail_type,
                            layer: Layer::SpecDetail,
                            links,
                        },
                        raw.source_file.clone(),
                    ));
                }
                Layer::Implementation => {
                    let impl_type = match ImplementationType::from_str(&raw.annotation_type) {
                        Ok(t) => Some(t),
                        Err(_) => {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Unknown ImplementationType '{}' for @{}",
                                    raw.annotation_type, raw.id
                                ),
                            });
                            None
                        }
                    };

                    let mut links = Vec::new();
                    for link_id in &raw.links {
                        let link_id: &String = link_id;
                        let link_id_stripped = link_id.strip_prefix('@').unwrap_or(link_id);
                        if let Some(target) = index.get(link_id_stripped) {
                            links.push(ImplementationLink::Abstract(Box::new(
                                AbstractAnnotation {
                                    id: AbstractAnnotationId(target.id.clone()),
                                    name: AbstractName(target.name.clone()),
                                    r#type: AbstractType::from_str(&target.annotation_type).ok(),
                                    layer: Layer::Abstract,
                                    links: vec![],
                                },
                            )));
                        } else {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                message: format!(
                                    "Link target '{}' not found (referenced from @{})",
                                    link_id, raw.id
                                ),
                            });
                        }
                    }

                    annotations.push(ResolvedAnnotation::Implementation(
                        ImplementationAnnotation {
                            id: ImplementationAnnotationId(raw.id.clone()),
                            name: ImplementationSpecName(raw.name.clone()),
                            r#type: impl_type,
                            layer: Layer::Implementation,
                            links,
                            artifact: ImplementationArtifact(raw.source_file.clone()),
                            status: ImplementationStatus::from_str(&raw.annotation_type).ok(),
                        },
                        raw.source_file.clone(),
                    ));
                }
            }
        }

        ResolveResult {
            annotations,
            warnings,
        }
    }
}
