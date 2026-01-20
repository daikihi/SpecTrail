/// [@st-spec-annotation-structure-file] layer: abstract, type: File, name: meta_diff.rs
/// This file defines the logic and data structure for comparing meta annotations between document and code annotations, supporting traceability and consistency checks.

use std::collections::HashSet;
use crate::domains::models::meta::MetaAnnotation;
use super::document_annotation::DocumentAnnotation;
use super::code_annotation::CodeAnnotation;

/// [@st-spec-annotation-structure] layer: abstract, type: Structure, name: MetaAnnotationDiff
/// This struct represents the diff of meta annotations between document and code as described in the specification.
#[derive(Debug, PartialEq, Eq)]
pub struct MetaAnnotationDiff {
    pub common: Vec<MetaAnnotation>,
    pub only_in_document: Vec<MetaAnnotation>,
    pub only_in_code: Vec<MetaAnnotation>,
}

impl MetaAnnotationDiff {
    pub fn compare(doc: &DocumentAnnotation, code: &CodeAnnotation) -> Self {
        let doc_metas: HashSet<_> = doc.metas.iter().cloned().collect();
        let code_metas: HashSet<_> = code.metas.iter().cloned().collect();

        let common: Vec<_> = doc_metas.intersection(&code_metas).cloned().collect();
        let only_in_document: Vec<_> = doc_metas.difference(&code_metas).cloned().collect();
        let only_in_code: Vec<_> = code_metas.difference(&doc_metas).cloned().collect();

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
    use crate::domains::models::meta::{MetaAnnotation, MetaName, MetaType};
    use crate::domains::models::layer::Layer;
    use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
    use crate::domains::models::annotation::code_annotation::CodeAnnotation;

    #[test]
    fn test_compare_same_annotations() {
        let meta = MetaAnnotation {
            name: MetaName("test".to_string()),
            r#type: MetaType::Rule,
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
        assert_eq!(diff.common[0].name.0, "test");
    }

    #[test]
    fn test_compare_different_annotations() {
        let meta1 = MetaAnnotation {
            name: MetaName("doc_only".to_string()),
            r#type: MetaType::Rule,
            layer: crate::domains::models::layer::Layer::Meta,
            links: vec![],
        };
        let meta2 = MetaAnnotation {
            name: MetaName("code_only".to_string()),
            r#type: MetaType::Guideline,
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
