/// [@st-code-libs-file] layer: abstract, type: File, name: libs.rs
pub mod domains;
pub mod use_case;

pub mod config {
    include!("libs/config.rs");
}
