/// [@st-code-domain-services-annotation-trace-resolver-file] layer: abstract, type: File, name: trace_resolver.rs, links: [@st-manual-spec-mapping-engine, @st-manual-spec-mapping-match-spec-to-code]
/// This file resolves traceability relationships (`Trace`) between document annotations and code annotations.
use crate::domains::models::annotation::AnnotationFile;
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::models::layer::Layer;
use crate::domains::models::line_number::LineNumber;
use crate::domains::models::spec_detail::SpecDetailType;
use crate::domains::models::trace::{Trace, TraceDirection, TraceKind, TraceSource};
use std::collections::{BTreeMap, HashMap};

struct Occurrence<'a> {
    file: &'a str,
    annotation: TraceSource,
}

impl Occurrence<'_> {
    fn location(&self) -> (&str, LineNumber, Layer) {
        (self.file, self.annotation.line(), self.annotation.layer())
    }
}

type Index<'a> = HashMap<String, Vec<Occurrence<'a>>>;

fn index_files<T: AnnotationFile>(files: &[T]) -> Index<'_> {
    let mut index = Index::new();
    for file in files {
        let mut insert = |annotation: TraceSource| {
            index
                .entry(annotation.id().to_owned())
                .or_default()
                .push(Occurrence {
                    file: file.source_file(),
                    annotation,
                });
        };
        for meta in file.metas() {
            insert(TraceSource::Meta(meta.clone()));
        }
        for abstract_annotation in file.abstracts() {
            insert(TraceSource::Abstract(abstract_annotation.clone()));
        }
        for detail in file.details() {
            insert(TraceSource::SpecDetail(detail.clone()));
        }
        for implementation in file.implementations() {
            insert(TraceSource::Implementation(implementation.clone()));
        }
    }
    index
}

// Explicit links carry a target layer. The code-to-document ID match accepts any layer.
fn visit_targets(
    source: &TraceSource,
    direction: TraceDirection,
    mut visit: impl FnMut(&str, Option<Layer>, TraceKind),
) {
    let matching_id_kind = match source {
        TraceSource::Meta(m) => {
            for id in &m.links {
                visit(&id.0, Some(Layer::Meta), TraceKind::Derives);
            }
            TraceKind::Derives
        }
        TraceSource::Abstract(a) => {
            for link in &a.links {
                let (id, layer) = link.target();
                let kind = match (direction, layer) {
                    (TraceDirection::DocumentToCode, _) => TraceKind::Refines,
                    (TraceDirection::CodeToDocument, Layer::Meta) => TraceKind::Derives,
                    _ => TraceKind::Implements,
                };
                visit(id, Some(layer), kind);
            }
            TraceKind::Implements
        }
        TraceSource::SpecDetail(d) => {
            let code_kind = if d.r#type == Some(SpecDetailType::Test) {
                TraceKind::Verifies
            } else {
                TraceKind::Implements
            };
            for link in &d.links {
                let (id, layer) = link.target();
                let kind = match (direction, layer) {
                    (TraceDirection::CodeToDocument, _) => code_kind,
                    (TraceDirection::DocumentToCode, Layer::Abstract) => TraceKind::Refines,
                    _ => TraceKind::Implements,
                };
                visit(id, Some(layer), kind);
            }
            code_kind
        }
        TraceSource::Implementation(i) => {
            for link in &i.links {
                let (id, layer) = link.target();
                visit(id, Some(layer), TraceKind::Implements);
            }
            TraceKind::Implements
        }
    };
    if direction == TraceDirection::CodeToDocument {
        visit(source.id(), None, matching_id_kind);
    }
}

/// [@st-code-domain-services-annotation-trace-resolver] layer: abstract, type: Structure, name: TraceResolver, links: [@st-manual-spec-mapping-match-spec-to-code]
/// Builds `Trace` links connecting code annotations and document annotations via explicit links or matching IDs.
pub struct TraceResolver;

impl TraceResolver {
    /// Resolves deterministic, deduplicated traces between annotation occurrences.
    pub fn resolve(documents: &[DocumentAnnotation], codes: &[CodeAnnotation]) -> Vec<Trace> {
        let documents = index_files(documents);
        let codes = index_files(codes);
        let mut traces = BTreeMap::new();
        for (direction, sources, targets) in [
            (TraceDirection::CodeToDocument, &codes, &documents),
            (TraceDirection::DocumentToCode, &documents, &codes),
        ] {
            for src in sources.values().flatten() {
                visit_targets(&src.annotation, direction, |id, layer, kind| {
                    let Some(candidates) = targets.get(id) else {
                        return;
                    };
                    for dst in candidates {
                        if layer.is_some_and(|layer| layer != dst.annotation.layer()) {
                            continue;
                        }
                        let key = (
                            src.annotation.id(),
                            dst.annotation.id(),
                            kind,
                            direction,
                            src.location(),
                            dst.location(),
                        );
                        traces.entry(key).or_insert_with(|| Trace {
                            direction,
                            src_file: src.file.to_owned(),
                            dst_file: dst.file.to_owned(),
                            src: src.annotation.clone(),
                            dst: dst.annotation.clone().into(),
                            kind,
                        });
                    }
                });
            }
        }
        traces.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::models::abstract_annotation::{
        AbstractAnnotation, AbstractAnnotationId, AbstractLink, AbstractName,
    };
    use crate::domains::models::implementation::{
        ImplementationAnnotation, ImplementationAnnotationId, ImplementationArtifact,
        ImplementationSpecName,
    };
    use crate::domains::models::layer::Layer;
    use crate::domains::models::line_number::LineNumber;
    use crate::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName};
    use crate::domains::models::spec_detail::{
        SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailName,
    };

    #[test]
    fn resolves_explicit_code_to_document_traces() {
        let doc = DocumentAnnotation {
            source_file: "specify_manual/spec.md".to_string(),
            metas: vec![MetaAnnotation {
                id: MetaAnnotationId("st-doc-meta".to_string()),
                name: MetaName("Doc Meta".to_string()),
                r#type: None,
                layer: Layer::Meta,
                links: vec![],
                line: LineNumber(1),
            }],
            abstracts: vec![],
            details: vec![SpecDetailAnnotation {
                id: SpecDetailAnnotationId("st-doc-detail".to_string()),
                name: SpecDetailName("Doc Detail".to_string()),
                r#type: Some(SpecDetailType::Func),
                layer: Layer::SpecDetail,
                links: vec![],
                line: LineNumber(10),
            }],
            implementations: vec![],
        };

        let code = CodeAnnotation {
            source_file: "src/feature.rs".to_string(),
            metas: vec![],
            abstracts: vec![AbstractAnnotation {
                id: AbstractAnnotationId("st-code-feature".to_string()),
                name: AbstractName("Feature".to_string()),
                r#type: None,
                layer: Layer::Abstract,
                links: vec![
                    AbstractLink::Meta(MetaAnnotationId("st-doc-meta".to_string())),
                    AbstractLink::SpecDetail(SpecDetailAnnotationId("st-doc-detail".to_string())),
                ],
                line: LineNumber(1),
            }],
            details: vec![],
            implementations: vec![],
        };

        let traces = TraceResolver::resolve(&[doc], &[code]);
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].src.id(), "st-code-feature");
        assert_eq!(traces[0].dst.id(), "st-doc-detail");
        assert_eq!(traces[0].kind, TraceKind::Implements);
        assert_eq!(traces[1].src.id(), "st-code-feature");
        assert_eq!(traces[1].dst.id(), "st-doc-meta");
        assert_eq!(traces[1].kind, TraceKind::Derives);
    }

    #[test]
    fn resolves_matching_id_traces_between_document_and_code() {
        let doc = DocumentAnnotation {
            source_file: "spec.md".to_string(),
            metas: vec![],
            abstracts: vec![AbstractAnnotation {
                id: AbstractAnnotationId("ss-req-1".to_string()),
                name: AbstractName("Requirement 1".to_string()),
                r#type: None,
                layer: Layer::Abstract,
                links: vec![],
                line: LineNumber(5),
            }],
            details: vec![],
            implementations: vec![],
        };

        let code = CodeAnnotation {
            source_file: "src/domain.rs".to_string(),
            metas: vec![],
            abstracts: vec![],
            details: vec![],
            implementations: vec![ImplementationAnnotation {
                id: ImplementationAnnotationId("ss-req-1".to_string()),
                name: ImplementationSpecName("Requirement 1".to_string()),
                r#type: None,
                layer: Layer::Implementation,
                links: vec![],
                artifact: ImplementationArtifact("src/domain.rs".to_string()),
                status: None,
                line: LineNumber(2),
            }],
        };

        let traces = TraceResolver::resolve(&[doc], &[code]);
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].src.id(), "ss-req-1");
        assert_eq!(traces[0].src.layer(), Layer::Implementation);
        assert_eq!(traces[0].dst.id(), "ss-req-1");
        assert_eq!(traces[0].dst.layer(), Layer::Abstract);
        assert_eq!(traces[0].kind, TraceKind::Implements);
    }

    #[test]
    fn preserves_distinct_directions_for_shared_ids_while_deduplicating_same_direction() {
        let doc = DocumentAnnotation {
            source_file: "specify_manual/meta.md".to_string(),
            metas: vec![
                MetaAnnotation {
                    id: MetaAnnotationId("meta-x".to_string()),
                    name: MetaName("Doc X".to_string()),
                    r#type: None,
                    layer: Layer::Meta,
                    links: vec![MetaAnnotationId("meta-y".to_string())],
                    line: LineNumber(1),
                },
                MetaAnnotation {
                    id: MetaAnnotationId("meta-y".to_string()),
                    name: MetaName("Doc Y".to_string()),
                    r#type: None,
                    layer: Layer::Meta,
                    links: vec![],
                    line: LineNumber(2),
                },
            ],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };

        let mut code = CodeAnnotation {
            source_file: "src/meta.rs".to_string(),
            metas: vec![
                MetaAnnotation {
                    id: MetaAnnotationId("meta-x".to_string()),
                    name: MetaName("Code X".to_string()),
                    r#type: None,
                    layer: Layer::Meta,
                    // Repeated link in same direction must be deduplicated
                    links: vec![
                        MetaAnnotationId("meta-y".to_string()),
                        MetaAnnotationId("meta-y".to_string()),
                    ],
                    line: LineNumber(10),
                },
                MetaAnnotation {
                    id: MetaAnnotationId("meta-y".to_string()),
                    name: MetaName("Code Y".to_string()),
                    r#type: None,
                    layer: Layer::Meta,
                    links: vec![],
                    line: LineNumber(20),
                },
            ],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };

        // A second annotation with the same ID at a different line is a separate endpoint.
        code.metas.push(MetaAnnotation {
            line: LineNumber(21),
            ..code.metas[1].clone()
        });
        let traces = TraceResolver::resolve(&[doc], &[code]);
        let x_to_y: Vec<_> = traces
            .iter()
            .filter(|t| t.src.id() == "meta-x" && t.dst.id() == "meta-y")
            .collect();

        // Both Code(meta-x) -> Doc(meta-y) and Doc(meta-x) -> Code(meta-y) are kept;
        // duplicate Code(meta-x) -> Doc(meta-y) is deduplicated.
        assert_eq!(x_to_y.len(), 3);
        assert_eq!(x_to_y[0].src.name(), "Code X");
        assert_eq!(x_to_y[0].dst.name(), "Doc Y");
        assert_eq!(x_to_y[0].kind, TraceKind::Derives);
        assert_eq!(x_to_y[1].src.name(), "Doc X");
        assert_eq!(x_to_y[1].dst.name(), "Code Y");
        assert_eq!(x_to_y[1].kind, TraceKind::Derives);
        assert_eq!(x_to_y[0].direction, TraceDirection::CodeToDocument);
        assert_eq!(x_to_y[1].direction, TraceDirection::DocumentToCode);
        assert_eq!(x_to_y[2].direction, TraceDirection::DocumentToCode);
        assert_eq!(x_to_y[1].dst.line(), LineNumber(20));
        assert_eq!(x_to_y[2].dst.line(), LineNumber(21));
    }
}
