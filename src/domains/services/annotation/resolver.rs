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
use crate::domains::models::line_number::LineNumber;
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
#[derive(Debug, Clone)]
pub struct ResolveWarning {
    pub source_annotation_id: String,
    pub source_file: String,
    pub line: LineNumber,
    pub message: String,
}

impl AnnotationResolver {
    /* Resolves a list of RawAnnotations into a ResolveResult.
     *
     * Uses default ignored prefix ["_fixture_"]. */
    pub fn resolve(raw_annotations: Vec<RawAnnotation>) -> ResolveResult {
        Self::resolve_with_ignored_prefixes(raw_annotations, &["_fixture_"])
    }

    /* Resolves a list of RawAnnotations into a ResolveResult, ignoring annotations
     * whose ID starts with any of the ignored_prefixes.
     *
     * Also ignores broken links when the target ID starts with any of the ignored_prefixes. */
    pub fn resolve_with_ignored_prefixes<S: AsRef<str>>(
        raw_annotations: Vec<RawAnnotation>,
        ignored_prefixes: &[S],
    ) -> ResolveResult {
        let is_ignored = |id: &str| -> bool {
            let stripped = id.strip_prefix('@').unwrap_or(id);
            ignored_prefixes.iter().any(|prefix| stripped.starts_with(prefix.as_ref()))
        };

        let mut annotations = Vec::new();
        let mut warnings = Vec::new();

        let index: HashMap<String, &RawAnnotation> = raw_annotations
            .iter()
            .filter(|a| !is_ignored(&a.id))
            .map(|a| (a.id.clone(), a))
            .collect();

        for raw in &raw_annotations {
            if is_ignored(&raw.id) {
                continue;
            }

            let layer = match raw.layer.as_str() {
                "meta" => Layer::Meta,
                "abstract" => Layer::Abstract,
                "spec-detail" => Layer::SpecDetail,
                "implementation" => Layer::Implementation,
                _ => {
                    warnings.push(ResolveWarning {
                        source_annotation_id: raw.id.clone(),
                        source_file: raw.source_file.clone(),
                        line: raw.line,
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
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                                line: target.line,
                            });
                        } else if !is_ignored(link_id) {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                            line: raw.line,
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
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                                line: target.line,
                            });
                        } else if !is_ignored(link_id) {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                            line: raw.line,
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
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                                line: target.line,
                            })));
                        } else if !is_ignored(link_id) {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                            line: raw.line,
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
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                                    line: target.line,
                                },
                            )));
                        } else if !is_ignored(link_id) {
                            warnings.push(ResolveWarning {
                                source_annotation_id: raw.id.clone(),
                                source_file: raw.source_file.clone(),
                                line: raw.line,
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
                            line: raw.line,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_fixture_prefix_by_default() {
        let raw = vec![
            RawAnnotation {
                id: "_fixture_bar".to_string(),
                layer: "abstract".to_string(),
                annotation_type: "Page".to_string(),
                name: "Bar".to_string(),
                links: vec!["@_fixture_foo".to_string()],
                source_file: "src/bar.rs".to_string(),
                line: LineNumber::new(1),
            },
        ];

        let result = AnnotationResolver::resolve(raw);
        // Annotation itself is ignored and not returned in resolved annotations
        assert_eq!(result.annotations.len(), 0);
        // No broken link warnings emitted
        assert_eq!(result.warnings.len(), 0);
    }

    #[test]
    fn test_ignores_broken_links_to_fixture() {
        let raw = vec![
            RawAnnotation {
                id: "st-valid".to_string(),
                layer: "abstract".to_string(),
                annotation_type: "Page".to_string(),
                name: "Valid".to_string(),
                links: vec!["@_fixture_target".to_string()],
                source_file: "src/valid.rs".to_string(),
                line: LineNumber::new(1),
            },
        ];

        let result = AnnotationResolver::resolve(raw);
        assert_eq!(result.annotations.len(), 1);
        // Link to _fixture_ is skipped from warnings
        assert_eq!(result.warnings.len(), 0);
    }

    #[test]
    fn test_ignores_custom_prefixes() {
        let raw = vec![
            RawAnnotation {
                id: "mock_spec".to_string(),
                layer: "abstract".to_string(),
                annotation_type: "Page".to_string(),
                name: "Mock".to_string(),
                links: vec!["@dummy_target".to_string()],
                source_file: "src/mock.rs".to_string(),
                line: LineNumber::new(1),
            },
        ];

        let result = AnnotationResolver::resolve_with_ignored_prefixes(
            raw,
            &["mock_", "dummy_"],
        );
        assert_eq!(result.annotations.len(), 0);
        assert_eq!(result.warnings.len(), 0);
    }
}
