/// [@st-code-domain-services-annotation-scanner-file] layer: abstract, type: File, name: scanner.rs
/// [@st-manual-meta-model-doc] layer: meta, type: Philosophy, name: Specification Model: Formal Definition
/// [@st-manual-meta-non-goals] layer: meta, type: Guideline, name: Non-goals
use crate::domains::models::abstract_annotation::{
    AbstractAnnotation, AbstractAnnotationId, AbstractName, AbstractType,
};
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::models::implementation::{
    ImplementationAnnotation, ImplementationAnnotationId, ImplementationArtifact,
    ImplementationLink, ImplementationSpecName, ImplementationStatus, ImplementationType,
};
use crate::domains::models::layer::Layer;
use crate::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName, MetaType};
use crate::domains::models::spec_detail::{
    SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailLink, SpecDetailName, SpecDetailType,
};
use crate::config::SpecTrailConfig;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// [@st-code-domain-services-annotation-scanner-annotation-scanner] layer: abstract, type: Structure, name: AnnotationScanner
pub struct AnnotationScanner;

impl AnnotationScanner {
    pub fn scan_code<P: AsRef<Path>>(path: P, extension: &str) -> Vec<CodeAnnotation> {
        let mut results = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // @todo target directory should be ignored
                    // This kind of directory naming depends on the programming language.
                    if path.file_name().and_then(|s| s.to_str()) == Some("target") {
                        continue;
                    }
                    results.extend(Self::scan_code(&path, extension));
                } else if path.extension().and_then(|s| s.to_str()) == Some(extension.trim_start_matches('.')) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some(container) = Self::parse_file_annotations::<CodeAnnotation>(
                            &content,
                            path.to_str().unwrap_or(""),
                        ) {
                            results.push(container);
                        }
                    }
                }
            }
        }
        results
    }

    pub fn scan_documents<P: AsRef<Path>>(path: P, extension: &str) -> Vec<DocumentAnnotation> {
        let mut results = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    results.extend(Self::scan_documents(&path, extension));
                } else if path.extension().and_then(|s| s.to_str()) == Some(extension.trim_start_matches('.')) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some(container) = Self::parse_file_annotations::<DocumentAnnotation>(
                            &content,
                            path.to_str().unwrap_or(""),
                        ) {
                            results.push(container);
                        }
                    }
                }
            }
        }
        results
    }

    fn parse_file_annotations<T: AnnotationContainer>(content: &str, file_path: &str) -> Option<T> {
        let mut container = T::default();
        let mut found = false;

        // [@tag] layer: ..., type: ..., name: ... [, links: [@tag1, @tag2]]
        let re = Regex::new(r"\[@(?P<id>[^\]]+)\]\s*layer:\s*(?P<layer>[^,]+),\s*type:\s*(?P<type>[^,]+),\s*name:\s*(?P<name>[^,\n]+)(?:,\s*links:\s*\[(?P<links>[^\]]+)\])?").unwrap();

        for cap in re.captures_iter(content) {
            found = true;
            let id = cap["id"].trim().to_string();
            let layer_str = cap["layer"].trim();
            let type_str = cap["type"].trim();
            let name = cap["name"].trim().to_string();
            // let links_str = cap.name("links").map(|m| m.as_str()); // 後の拡張用

            match layer_str {
                "meta" => {
                    let mut meta = MetaAnnotation {
                        id: MetaAnnotationId(id),
                        name: MetaName(name),
                        r#type: MetaType::from_str(type_str).ok(),
                        layer: Layer::Meta,
                        links: vec![],
                    };
                    if let Some(links_str) = cap.name("links").map(|m| m.as_str()) {
                        for link_id in links_str.split(',') {
                            let link_id = link_id.trim();
                            if !link_id.is_empty() {
                                // Temporarily maintain structure as a link to itself
                                meta.links.push(MetaAnnotation {
                                    id: MetaAnnotationId(link_id.to_string()),
                                    name: MetaName("".to_string()),
                                    r#type: None,
                                    layer: Layer::Meta,
                                    links: vec![],
                                });
                            }
                        }
                    }
                    container.add_meta(meta);
                }
                "abstract" => {
                    let mut abs = AbstractAnnotation {
                        id: AbstractAnnotationId(id),
                        name: AbstractName(name),
                        r#type: AbstractType::from_str(type_str).ok(),
                        layer: Layer::Abstract,
                        links: vec![],
                    };
                    if let Some(links_str) = cap.name("links").map(|m| m.as_str()) {
                        for link_id in links_str.split(',') {
                            let link_id = link_id.trim();
                            if !link_id.is_empty() {
                                abs.links.push(SpecDetailAnnotation {
                                    id: SpecDetailAnnotationId(link_id.to_string()),
                                    name: SpecDetailName("".to_string()),
                                    r#type: None,
                                    layer: Layer::SpecDetail,
                                    links: vec![],
                                });
                            }
                        }
                    }
                    container.add_abstract(abs);
                }
                "spec-detail" => {
                    let mut detail = SpecDetailAnnotation {
                        id: SpecDetailAnnotationId(id),
                        name: SpecDetailName(name),
                        r#type: SpecDetailType::from_str(type_str).ok(),
                        layer: Layer::SpecDetail,
                        links: vec![],
                    };
                    if let Some(links_str) = cap.name("links").map(|m| m.as_str()) {
                        for link_id in links_str.split(',') {
                            let link_id = link_id.trim();
                            if !link_id.is_empty() {
                                detail.links.push(SpecDetailLink::Abstract(Box::new(
                                    AbstractAnnotation {
                                        id: AbstractAnnotationId(link_id.to_string()),
                                        name: AbstractName("".to_string()),
                                        r#type: None,
                                        layer: Layer::Abstract,
                                        links: vec![],
                                    },
                                )));
                            }
                        }
                    }
                    container.add_detail(detail);
                }
                "implementation" => {
                    let mut impl_anno = ImplementationAnnotation {
                        id: ImplementationAnnotationId(id),
                        name: ImplementationSpecName(name),
                        r#type: ImplementationType::from_str(type_str).ok(),
                        layer: Layer::Implementation,
                        links: vec![],
                        artifact: ImplementationArtifact(file_path.to_string()),
                        status: ImplementationStatus::from_str(type_str).ok(),
                    };
                    if let Some(links_str) = cap.name("links").map(|m| m.as_str()) {
                        for link_id in links_str.split(',') {
                            let link_id = link_id.trim();
                            if !link_id.is_empty() {
                                impl_anno.links.push(ImplementationLink::Abstract(Box::new(
                                    AbstractAnnotation {
                                        id: AbstractAnnotationId(link_id.to_string()),
                                        name: AbstractName("".to_string()),
                                        r#type: None,
                                        layer: Layer::Abstract,
                                        links: vec![],
                                    },
                                )));
                            }
                        }
                    }
                    container.add_implementation(impl_anno);
                }
                _ => {}
            }
        }

        if found { Some(container) } else { None }
    }
}

trait AnnotationContainer: Default {
    fn add_meta(&mut self, annotation: MetaAnnotation);
    fn add_abstract(&mut self, annotation: AbstractAnnotation);
    fn add_detail(&mut self, annotation: SpecDetailAnnotation);
    fn add_implementation(&mut self, annotation: ImplementationAnnotation);
}

impl AnnotationContainer for CodeAnnotation {
    fn add_meta(&mut self, annotation: MetaAnnotation) {
        self.metas.push(annotation);
    }
    fn add_abstract(&mut self, annotation: AbstractAnnotation) {
        self.abstracts.push(annotation);
    }
    fn add_detail(&mut self, annotation: SpecDetailAnnotation) {
        self.details.push(annotation);
    }
    fn add_implementation(&mut self, annotation: ImplementationAnnotation) {
        self.implementations.push(annotation);
    }
}

impl AnnotationContainer for DocumentAnnotation {
    fn add_meta(&mut self, annotation: MetaAnnotation) {
        self.metas.push(annotation);
    }
    fn add_abstract(&mut self, annotation: AbstractAnnotation) {
        self.abstracts.push(annotation);
    }
    fn add_detail(&mut self, annotation: SpecDetailAnnotation) {
        self.details.push(annotation);
    }
    fn add_implementation(&mut self, annotation: ImplementationAnnotation) {
        self.implementations.push(annotation);
    }
}

impl Default for CodeAnnotation {
    fn default() -> Self {
        CodeAnnotation {
            metas: vec![],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        }
    }
}

impl Default for DocumentAnnotation {
    fn default() -> Self {
        DocumentAnnotation {
            metas: vec![],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        }
    }
}
