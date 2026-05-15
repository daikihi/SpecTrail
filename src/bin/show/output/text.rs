use crate::show_response_adapter::ShowResponseView;
use crate::dto::ShowView;
use SpecTrail::domains::models::annotation::code_annotation::CodeAnnotation;
use SpecTrail::domains::models::annotation::document_annotation::DocumentAnnotation;

pub fn render_text(view_model: &ShowResponseView) {
    match view_model.view {
        ShowView::Summary => {
            render_summary(view_model);
        }
        ShowView::List => {
            render_list(view_model);
        }
        ShowView::Group => {
            // Currently same as list
            render_list(view_model);
        }
        ShowView::Detail => {
            render_detail(view_model);
        }
    }
}

fn render_summary(view_model: &ShowResponseView) {
    let doc_count = view_model.document_annotations.len();
    let code_count = view_model.code_annotations.len();
    let total_annotations = count_total_annotations(view_model);

    println!("Summary:");
    println!("  Total annotations: {}", total_annotations);
    println!("  Document files: {}", doc_count);
    println!("  Code files: {}", code_count);
}

fn render_list(view_model: &ShowResponseView) {
    if view_model.document_annotations.is_empty() && view_model.code_annotations.is_empty() {
        println!("No annotations found.");
        return;
    }

    if !view_model.document_annotations.is_empty() {
        println!("Document Annotations:");
        append_file_section(&view_model.document_annotations, "document");
    }

    if !view_model.code_annotations.is_empty() {
        println!("Code Annotations:");
        append_file_section(&view_model.code_annotations, "code");
    }
}

fn render_detail(view_model: &ShowResponseView) {
    if view_model.document_annotations.is_empty() && view_model.code_annotations.is_empty() {
        println!("No annotations found.");
        return;
    }

    for doc in &view_model.document_annotations {
        println!("File: {} (document)", doc.source_file);
        for meta in &doc.metas {
            println!("  [Meta] ID: {}, Name: {}", meta.id.0, meta.name.0);
        }
        for abs in &doc.abstracts {
            println!("  [Abstract] ID: {}, Name: {}", abs.id.0, abs.name.0);
        }
        for det in &doc.details {
            println!("  [Detail] ID: {}, Name: {}", det.id.0, det.name.0);
        }
        for imp in &doc.implementations {
            println!("  [Implementation] ID: {}", imp.id.0);
        }
        println!();
    }

    for code in &view_model.code_annotations {
        println!("File: {} (code)", code.source_file);
        for meta in &code.metas {
            println!("  [Meta] ID: {}, Name: {}", meta.id.0, meta.name.0);
        }
        for abs in &code.abstracts {
            println!("  [Abstract] ID: {}, Name: {}", abs.id.0, abs.name.0);
        }
        for det in &code.details {
            println!("  [Detail] ID: {}, Name: {}", det.id.0, det.name.0);
        }
        for imp in &code.implementations {
            println!("  [Implementation] ID: {}", imp.id.0);
        }
        println!();
    }
}

fn append_file_section(annotations: &[impl AnnotationGroupView], label: &str) {
    let mut items: Vec<_> = annotations.iter().collect();
    items.sort_by(|a, b| a.source_file().cmp(b.source_file()));

    for (index, annotation) in items.into_iter().enumerate() {
        println!("[{}] {}", index + 1, annotation.source_file());
        println!("    metas: {}", annotation.meta_count());
        println!("    abstracts: {}", annotation.abstract_count());
        println!("    details: {}", annotation.detail_count());
        println!("    implementations: {}", annotation.implementation_count());
        println!("    source: {}", label);
        println!();
    }
}

fn count_total_annotations(view_model: &ShowResponseView) -> usize {
    let doc_sum: usize = view_model.document_annotations.iter().map(|a| 
        a.metas.len() + a.abstracts.len() + a.details.len() + a.implementations.len()
    ).sum();
    let code_sum: usize = view_model.code_annotations.iter().map(|a| 
        a.metas.len() + a.abstracts.len() + a.details.len() + a.implementations.len()
    ).sum();
    doc_sum + code_sum
}

trait AnnotationGroupView {
    fn source_file(&self) -> &str;
    fn meta_count(&self) -> usize;
    fn abstract_count(&self) -> usize;
    fn detail_count(&self) -> usize;
    fn implementation_count(&self) -> usize;
}

impl AnnotationGroupView for CodeAnnotation {
    fn source_file(&self) -> &str { &self.source_file }
    fn meta_count(&self) -> usize { self.metas.len() }
    fn abstract_count(&self) -> usize { self.abstracts.len() }
    fn detail_count(&self) -> usize { self.details.len() }
    fn implementation_count(&self) -> usize { self.implementations.len() }
}

impl AnnotationGroupView for DocumentAnnotation {
    fn source_file(&self) -> &str { &self.source_file }
    fn meta_count(&self) -> usize { self.metas.len() }
    fn abstract_count(&self) -> usize { self.abstracts.len() }
    fn detail_count(&self) -> usize { self.details.len() }
    fn implementation_count(&self) -> usize { self.implementations.len() }
}
