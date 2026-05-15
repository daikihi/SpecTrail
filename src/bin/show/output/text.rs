use crate::show_response_adapter::ShowResponseView;
use crate::dto::ShowView;
use SpecTrail::domains::models::annotation::code_annotation::CodeAnnotation;
use SpecTrail::domains::models::annotation::document_annotation::DocumentAnnotation;

/// Render a ShowResponseView in plain text according to its view variant.
///
/// The function dispatches to the appropriate text renderer for the view:
/// summary, list/group, or detail.
///
/// # Examples
///
/// ```
/// use crate::show::{ShowResponseView, ShowView};
/// use crate::show::output::text::render_text;
///
/// let vm = ShowResponseView { view: ShowView::Summary, document_annotations: vec![], code_annotations: vec![] };
/// render_text(&vm);
/// ```
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

/// Prints a concise summary of annotation counts to stdout.
///
/// The summary includes the total number of annotations, the number of document files,
/// and the number of code files from the provided view model.
///
/// `view_model` is the show response view containing `document_annotations` and
/// `code_annotations` used to compute the counts.
fn render_summary(view_model: &ShowResponseView) {
    let doc_count = view_model.document_annotations.len();
    let code_count = view_model.code_annotations.len();
    let total_annotations = count_total_annotations(view_model);

    println!("Summary:");
    println!("  Total annotations: {}", total_annotations);
    println!("  Document files: {}", doc_count);
    println!("  Code files: {}", code_count);
}

/// Renders a grouped list of annotations to standard output.
///
/// Prints "No annotations found." when there are no document or code annotations.
/// Otherwise prints a "Document Annotations:" section if document annotations exist
/// and a "Code Annotations:" section if code annotations exist, each followed by
/// per-file counts for metas, abstracts, details, and implementations.
///
/// # Examples
///
/// ```
/// // Construct a view model and render its list representation.
/// let vm = ShowResponseView::default();
/// render_list(&vm);
/// ```
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

/// Prints detailed per-file annotation listings for the provided view model.
///
/// For each document and code annotation group this function prints the file name
/// and then lists each Meta, Abstract, Detail (with ID and Name) and Implementation
/// (ID only). If there are no annotations in either category, it prints
/// "No annotations found.".
///
/// # Examples
///
/// ```
/// let view_model = ShowResponseView {
///     document_annotations: Vec::new(),
///     code_annotations: Vec::new(),
/// };
/// render_detail(&view_model);
/// ```
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

/// Prints a numbered section for each annotation group showing the file name, counts, and a source label.
///
/// The entries are emitted in lexicographic order by `source_file()`. For each entry this prints:
/// the 1-based index and file name, counts for metas, abstracts, details, and implementations, and the provided `label`.
///
/// # Examples
///
/// ```
/// struct Mock {
///     file: String,
///     metas: usize,
///     abstracts: usize,
///     details: usize,
///     implementations: usize,
/// }
///
/// impl AnnotationGroupView for Mock {
///     fn source_file(&self) -> &str { &self.file }
///     fn meta_count(&self) -> usize { self.metas }
///     fn abstract_count(&self) -> usize { self.abstracts }
///     fn detail_count(&self) -> usize { self.details }
///     fn implementation_count(&self) -> usize { self.implementations }
/// }
///
/// let items = vec![
///     Mock { file: "b.txt".into(), metas: 1, abstracts: 0, details: 2, implementations: 0 },
///     Mock { file: "a.txt".into(), metas: 0, abstracts: 1, details: 0, implementations: 1 },
/// ];
///
/// append_file_section(&items, "document");
/// ```
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

/// Computes the total number of annotations across all document and code groups.
///
/// Sums the counts of `metas`, `abstracts`, `details`, and `implementations` for
/// each entry in `view_model.document_annotations` and `view_model.code_annotations`.
///
/// # Examples
///
/// ```
/// // given a populated `view: ShowResponseView`
/// let total = count_total_annotations(&view);
/// ```
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
    /// Source file path for this annotation group.
///
/// # Returns
///
/// `&str` slice of the source file path.
///
/// # Examples
///
/// ```
/// use crate::model::DocumentAnnotation;
///
/// let ann = DocumentAnnotation {
///     source_file: "docs/readme.md".to_string(),
///     metas: Vec::new(),
///     abstracts: Vec::new(),
///     details: Vec::new(),
///     implementations: Vec::new(),
/// };
/// assert_eq!(ann.source_file(), "docs/readme.md");
/// ```
fn source_file(&self) -> &str { &self.source_file }
    /// Number of meta annotations in this group.
///
/// # Returns
///
/// The count of meta annotations as a `usize`.
///
/// # Examples
///
/// ```
/// // `annotation` is any value implementing this method
/// let _count = annotation.meta_count();
/// ```
fn meta_count(&self) -> usize { self.metas.len() }
    /// Returns the number of abstracts in this annotation group.
///
/// # Returns
///
/// `usize` with the count of items in the `abstracts` collection.
///
/// # Examples
///
/// ```
/// struct LocalGroup { abstracts: Vec<()> }
/// impl LocalGroup {
///     fn abstract_count(&self) -> usize { self.abstracts.len() }
/// }
///
/// let g = LocalGroup { abstracts: vec![(), ()] };
/// assert_eq!(g.abstract_count(), 2);
/// ```
fn abstract_count(&self) -> usize { self.abstracts.len() }
    /// Gets the number of detail annotations in this group.
///
/// # Examples
///
/// ```
/// // Assuming `annotation` implements `detail_count`:
/// let n = annotation.detail_count();
/// assert_eq!(n, annotation.details.len());
/// ```
fn detail_count(&self) -> usize { self.details.len() }
    /// Get the number of implementations in this group.
///
/// # Examples
///
/// ```
/// struct Dummy { implementations: Vec<u8> }
/// impl Dummy { fn implementation_count(&self) -> usize { self.implementations.len() } }
/// let d = Dummy { implementations: vec![1, 2, 3] };
/// assert_eq!(d.implementation_count(), 3);
/// ```
fn implementation_count(&self) -> usize { self.implementations.len() }
}

impl AnnotationGroupView for DocumentAnnotation {
    /// Source file path for this annotation group.
///
/// # Returns
///
/// `&str` slice of the source file path.
///
/// # Examples
///
/// ```
/// use crate::model::DocumentAnnotation;
///
/// let ann = DocumentAnnotation {
///     source_file: "docs/readme.md".to_string(),
///     metas: Vec::new(),
///     abstracts: Vec::new(),
///     details: Vec::new(),
///     implementations: Vec::new(),
/// };
/// assert_eq!(ann.source_file(), "docs/readme.md");
/// ```
fn source_file(&self) -> &str { &self.source_file }
    /// Number of meta annotations in this group.
///
/// # Returns
///
/// The count of meta annotations as a `usize`.
///
/// # Examples
///
/// ```
/// // `annotation` is any value implementing this method
/// let _count = annotation.meta_count();
/// ```
fn meta_count(&self) -> usize { self.metas.len() }
    /// Returns the number of abstracts in this annotation group.
///
/// # Returns
///
/// `usize` with the count of items in the `abstracts` collection.
///
/// # Examples
///
/// ```
/// struct LocalGroup { abstracts: Vec<()> }
/// impl LocalGroup {
///     fn abstract_count(&self) -> usize { self.abstracts.len() }
/// }
///
/// let g = LocalGroup { abstracts: vec![(), ()] };
/// assert_eq!(g.abstract_count(), 2);
/// ```
fn abstract_count(&self) -> usize { self.abstracts.len() }
    /// Gets the number of detail annotations in this group.
///
/// # Examples
///
/// ```
/// // Assuming `annotation` implements `detail_count`:
/// let n = annotation.detail_count();
/// assert_eq!(n, annotation.details.len());
/// ```
fn detail_count(&self) -> usize { self.details.len() }
    /// Get the number of implementations in this group.
///
/// # Examples
///
/// ```
/// struct Dummy { implementations: Vec<u8> }
/// impl Dummy { fn implementation_count(&self) -> usize { self.implementations.len() } }
/// let d = Dummy { implementations: vec![1, 2, 3] };
/// assert_eq!(d.implementation_count(), 3);
/// ```
fn implementation_count(&self) -> usize { self.implementations.len() }
}
