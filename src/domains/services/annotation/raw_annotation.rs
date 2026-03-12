/// [@st-code-domain-services-annotation-raw-annotation-file] layer: abstract, type: File, name: raw_annotation.rs
/// This file defines the raw representation of an annotation as extracted during the initial parsing phase.

/// [@st-code-domain-services-annotation-raw-annotation-raw-annotation] layer: abstract, type: Structure, name: RawAnnotation
/// A simple data structure holding the raw text of an annotation before it is resolved into a domain model.
pub struct RawAnnotation {
    pub id: String,
    pub layer: String,
    pub annotation_type: String,
    pub name: String,
    pub links: Vec<String>,  /* Raw ID strings, e.g., ["@st-foo", "@st-bar"] */
    pub source_file: String, /* Path to the file where this annotation was found */
}
