/// [@st-code-domain-services-annotation-trace-resolver-file] layer: abstract, type: File, name: trace_resolver.rs, links: [@st-manual-spec-mapping-engine, @st-manual-spec-mapping-match-spec-to-code]
/// This file resolves traceability relationships (`Trace`) between document annotations and code annotations.
use crate::domains::models::abstract_annotation::AbstractLink;
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::models::implementation::ImplementationLink;
use crate::domains::models::spec_detail::{SpecDetailLink, SpecDetailType};
use crate::domains::models::trace::{Trace, TraceDestination, TraceKind, TraceSource};
use std::collections::{HashMap, HashSet};

/// [@st-code-domain-services-annotation-trace-resolver] layer: abstract, type: Structure, name: TraceResolver, links: [@st-manual-spec-mapping-match-spec-to-code]
/// Builds `Trace` links connecting code annotations and document annotations via explicit links or matching IDs.
pub struct TraceResolver;

impl TraceResolver {
    /// Resolves deterministic, deduplicated `Trace` edges between document and code annotations.
    pub fn resolve(documents: &[DocumentAnnotation], codes: &[CodeAnnotation]) -> Vec<Trace> {
        let mut doc_index: HashMap<String, TraceDestination> = HashMap::new();
        for doc in documents {
            for m in &doc.metas {
                doc_index.insert(m.id.0.clone(), TraceDestination::Meta(m.clone()));
            }
            for a in &doc.abstracts {
                doc_index.insert(a.id.0.clone(), TraceDestination::Abstract(a.clone()));
            }
            for d in &doc.details {
                doc_index.insert(d.id.0.clone(), TraceDestination::SpecDetail(d.clone()));
            }
            for i in &doc.implementations {
                doc_index.insert(i.id.0.clone(), TraceDestination::Implementation(i.clone()));
            }
        }

        let mut code_index: HashMap<String, TraceDestination> = HashMap::new();
        for code in codes {
            for m in &code.metas {
                code_index.insert(m.id.0.clone(), TraceDestination::Meta(m.clone()));
            }
            for a in &code.abstracts {
                code_index.insert(a.id.0.clone(), TraceDestination::Abstract(a.clone()));
            }
            for d in &code.details {
                code_index.insert(d.id.0.clone(), TraceDestination::SpecDetail(d.clone()));
            }
            for i in &code.implementations {
                code_index.insert(i.id.0.clone(), TraceDestination::Implementation(i.clone()));
            }
        }

        let mut traces = Vec::new();
        let mut seen: HashSet<(String, String, TraceKind)> = HashSet::new();

        let mut push_trace = |src: TraceSource, dst: TraceDestination, kind: TraceKind| {
            let key = (src.id().to_string(), dst.id().to_string(), kind);
            if seen.insert(key) {
                traces.push(Trace { src, dst, kind });
            }
        };

        // 1. Explicit links from code annotations to document annotations
        for code in codes {
            for m in &code.metas {
                for target_id in &m.links {
                    if let Some(dst) = doc_index.get(&target_id.0) {
                        push_trace(TraceSource::Meta(m.clone()), dst.clone(), TraceKind::Derives);
                    }
                }
                if let Some(dst) = doc_index.get(&m.id.0) {
                    push_trace(TraceSource::Meta(m.clone()), dst.clone(), TraceKind::Derives);
                }
            }
            for a in &code.abstracts {
                for link in &a.links {
                    let (target_id, kind) = match link {
                        AbstractLink::Meta(id) => (&id.0, TraceKind::Derives),
                        AbstractLink::Abstract(id) => (&id.0, TraceKind::Implements),
                        AbstractLink::SpecDetail(id) => (&id.0, TraceKind::Implements),
                    };
                    if let Some(dst) = doc_index.get(target_id) {
                        push_trace(TraceSource::Abstract(a.clone()), dst.clone(), kind);
                    }
                }
                if let Some(dst) = doc_index.get(&a.id.0) {
                    push_trace(
                        TraceSource::Abstract(a.clone()),
                        dst.clone(),
                        TraceKind::Implements,
                    );
                }
            }
            for d in &code.details {
                let kind = if d.r#type == Some(SpecDetailType::Test) {
                    TraceKind::Verifies
                } else {
                    TraceKind::Implements
                };
                for link in &d.links {
                    let target_id = match link {
                        SpecDetailLink::Abstract(id) => &id.0,
                        SpecDetailLink::Implementation(id) => &id.0,
                    };
                    if let Some(dst) = doc_index.get(target_id) {
                        push_trace(TraceSource::SpecDetail(d.clone()), dst.clone(), kind);
                    }
                }
                if let Some(dst) = doc_index.get(&d.id.0) {
                    push_trace(TraceSource::SpecDetail(d.clone()), dst.clone(), kind);
                }
            }
            for i in &code.implementations {
                for link in &i.links {
                    let target_id = match link {
                        ImplementationLink::SpecDetail(id) => &id.0,
                        ImplementationLink::Abstract(id) => &id.0,
                    };
                    if let Some(dst) = doc_index.get(target_id) {
                        push_trace(
                            TraceSource::Implementation(i.clone()),
                            dst.clone(),
                            TraceKind::Implements,
                        );
                    }
                }
                if let Some(dst) = doc_index.get(&i.id.0) {
                    push_trace(
                        TraceSource::Implementation(i.clone()),
                        dst.clone(),
                        TraceKind::Implements,
                    );
                }
            }
        }

        // 2. Explicit links from document annotations to code annotations
        for doc in documents {
            for m in &doc.metas {
                for target_id in &m.links {
                    if let Some(dst) = code_index.get(&target_id.0) {
                        push_trace(TraceSource::Meta(m.clone()), dst.clone(), TraceKind::Derives);
                    }
                }
            }
            for a in &doc.abstracts {
                for link in &a.links {
                    let target_id = match link {
                        AbstractLink::Meta(id) => &id.0,
                        AbstractLink::Abstract(id) => &id.0,
                        AbstractLink::SpecDetail(id) => &id.0,
                    };
                    if let Some(dst) = code_index.get(target_id) {
                        push_trace(
                            TraceSource::Abstract(a.clone()),
                            dst.clone(),
                            TraceKind::Refines,
                        );
                    }
                }
            }
            for d in &doc.details {
                for link in &d.links {
                    let (target_id, kind) = match link {
                        SpecDetailLink::Abstract(id) => (&id.0, TraceKind::Refines),
                        SpecDetailLink::Implementation(id) => (&id.0, TraceKind::Implements),
                    };
                    if let Some(dst) = code_index.get(target_id) {
                        push_trace(TraceSource::SpecDetail(d.clone()), dst.clone(), kind);
                    }
                }
            }
            for i in &doc.implementations {
                for link in &i.links {
                    let target_id = match link {
                        ImplementationLink::SpecDetail(id) => &id.0,
                        ImplementationLink::Abstract(id) => &id.0,
                    };
                    if let Some(dst) = code_index.get(target_id) {
                        push_trace(
                            TraceSource::Implementation(i.clone()),
                            dst.clone(),
                            TraceKind::Implements,
                        );
                    }
                }
            }
        }

        traces.sort_by(|a, b| {
            a.src
                .id()
                .cmp(b.src.id())
                .then_with(|| a.dst.id().cmp(b.dst.id()))
                .then_with(|| a.kind.cmp(&b.kind))
        });

        traces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::models::abstract_annotation::{
        AbstractAnnotation, AbstractAnnotationId, AbstractName,
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
}
