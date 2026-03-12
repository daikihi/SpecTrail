use super::code_annotation::CodeAnnotation;
use super::document_annotation::DocumentAnnotation;
use crate::domains::models::meta::{MetaAnnotation, MetaAnnotationId};
/// [@st-code-domain-models-annotation-meta-diff-file] layer: abstract, type: File, name: meta_diff.rs
/// This file defines the logic and data structure for comparing meta annotations between document and code annotations, supporting traceability and consistency checks.
use std::collections::{HashMap, HashSet};

/// [@st-code-domain-models-annotation-meta-annotation-diff] layer: abstract, type: Structure, name: MetaAnnotationDiff
/// This struct represents the diff of meta annotations between document and code as described in the specification.
#[derive(Debug, PartialEq, Eq)]
pub struct MetaAnnotationDiff {
    pub common: Vec<MetaAnnotation>,
    pub only_in_document: Vec<MetaAnnotation>,
    pub only_in_code: Vec<MetaAnnotation>,
}

impl MetaAnnotationDiff {
    pub fn compare(doc: &DocumentAnnotation, code: &CodeAnnotation) -> Self {
        let doc_by_id: HashMap<MetaAnnotationId, MetaAnnotation> = doc
            .metas
            .iter()
            .cloned()
            .map(|m| (m.id.clone(), m))
            .collect();
        let code_by_id: HashMap<MetaAnnotationId, MetaAnnotation> = code
            .metas
            .iter()
            .cloned()
            .map(|m| (m.id.clone(), m))
            .collect();

        let doc_ids: HashSet<_> = doc_by_id.keys().cloned().collect();
        let code_ids: HashSet<_> = code_by_id.keys().cloned().collect();

        let mut common_ids: Vec<_> = doc_ids.intersection(&code_ids).cloned().collect();
        let mut only_in_document_ids: Vec<_> = doc_ids.difference(&code_ids).cloned().collect();
        let mut only_in_code_ids: Vec<_> = code_ids.difference(&doc_ids).cloned().collect();

        common_ids.sort_by(|a, b| a.0.cmp(&b.0));
        only_in_document_ids.sort_by(|a, b| a.0.cmp(&b.0));
        only_in_code_ids.sort_by(|a, b| a.0.cmp(&b.0));

        let common: Vec<_> = common_ids
            .into_iter()
            .filter_map(|id| doc_by_id.get(&id).cloned())
            .collect();
        let only_in_document: Vec<_> = only_in_document_ids
            .into_iter()
            .filter_map(|id| doc_by_id.get(&id).cloned())
            .collect();
        let only_in_code: Vec<_> = only_in_code_ids
            .into_iter()
            .filter_map(|id| code_by_id.get(&id).cloned())
            .collect();

        Self {
            common,
            only_in_document,
            only_in_code,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.only_in_document.is_empty() && self.only_in_code.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::models::annotation::code_annotation::CodeAnnotation;
    use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
    use crate::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName, MetaType};

    #[test]
    fn test_compare_same_annotations() {
        let meta = MetaAnnotation {
            id: MetaAnnotationId("@st-test-same".to_string()),
            name: MetaName("test".to_string()),
            r#type: Some(MetaType::Rule),
            layer: crate::domains::models::layer::Layer::Meta,
            links: vec![],
        };
        let doc = DocumentAnnotation {
            metas: vec![meta.clone()],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };
        let code = CodeAnnotation {
            metas: vec![meta.clone()],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };

        let diff = MetaAnnotationDiff::compare(&doc, &code);
        assert!(diff.is_empty());
        assert_eq!(diff.common.len(), 1);
        assert_eq!(diff.common[0].id.0, "@st-test-same");
    }

    #[test]
    fn test_compare_different_annotations() {
        let meta1 = MetaAnnotation {
            id: MetaAnnotationId("@st-test-doc-only".to_string()),
            name: MetaName("doc_only".to_string()),
            r#type: Some(MetaType::Rule),
            layer: crate::domains::models::layer::Layer::Meta,
            links: vec![],
        };
        let meta2 = MetaAnnotation {
            id: MetaAnnotationId("@st-test-code-only".to_string()),
            name: MetaName("code_only".to_string()),
            r#type: Some(MetaType::Guideline),
            layer: crate::domains::models::layer::Layer::Meta,
            links: vec![],
        };
        let doc = DocumentAnnotation {
            metas: vec![meta1.clone()],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };
        let code = CodeAnnotation {
            metas: vec![meta2.clone()],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };

        let diff = MetaAnnotationDiff::compare(&doc, &code);
        assert!(!diff.is_empty());
        assert_eq!(diff.only_in_document.len(), 1);
        assert_eq!(diff.only_in_document[0].name.0, "doc_only");
        assert_eq!(diff.only_in_code.len(), 1);
        assert_eq!(diff.only_in_code[0].name.0, "code_only");
        assert_eq!(diff.common.len(), 0);
    }
}
