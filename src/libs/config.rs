use serde::Deserialize;
use std::fs;
use std::path::Path;

/**
 * configuration loader from a toml file using with parameter
 */
/// [@st-code-libs-config-spec-trail-config] layer: abstract, type: Structure, name: SpecTrailConfig
#[derive(Debug, Deserialize)]
pub struct SpecTrailConfig {
    /// [@st-code-libs-config-spec-trail-config-source] layer: abstract, type: Structure, name: source
    pub source: SourceConfig,
    /// [@st-code-libs-config-spec-trail-config-document] layer: abstract, type: Structure, name: document
    pub document: DocumentConfig,
    /// [@st-code-libs-config-spec-trail-config-annotation] layer: abstract, type: Structure, name: annotation
    pub annotation: AnnotationConfig,
}

impl SpecTrailConfig {
    /// Loads a SpecTrailConfig from a TOML file at the specified path.
    ///
    /// On success, returns the parsed SpecTrailConfig. File I/O failures or TOML
    /// deserialization errors are propagated and returned as a boxed dynamic error.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// let toml = r#"
    /// [source]
    /// head = ".specify/"
    /// extension = "rs"
    /// [document]
    /// head = "docs/"
    /// extension = "md"
    /// [annotation]
    /// prefix = "@spec"
    /// "#;
    /// fs::write("test_config.toml", toml).unwrap();
    /// let config = SpecTrailConfig::from_file("test_config.toml").unwrap();
    /// assert_eq!(config.source.head, ".specify/");
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content: String = fs::read_to_string(path)?;
        let config: SpecTrailConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

/**
 * Using for source code specs
 */
/// [@st-code-libs-config-source-config] layer: abstract, type: Structure, name: SourceConfig
#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub head: String,
    pub extension: String,
}

/**
 * Using for document specs
 */
/// [@st-code-libs-config-document-config] layer: abstract, type: Structure, name: DocumentConfig
#[derive(Debug, Deserialize)]
pub struct DocumentConfig {
    pub head: String,
    pub extension: String,
}

/**
 * Using for annotation specs
 */
/// [@st-code-libs-config-annotation-config] layer: abstract, type: Structure, name: AnnotationConfig
#[derive(Debug, Deserialize)]
pub struct AnnotationConfig {
    pub prefix: String,
}

#[cfg(test)]
mod tests_spec_trail_config {
    use super::*;
    use std::env;

    fn write_temp_config(name: &str, content: &str) -> std::path::PathBuf {
        let path = env::temp_dir().join(name);
        fs::write(&path, content).expect("failed to write temp config");
        path
    }

    #[test]
    fn it_works() {
        let config_result: Result<SpecTrailConfig, _> =
            SpecTrailConfig::from_file("src/config/default.toml");
        assert!(config_result.is_ok());
        let config = config_result.unwrap();
        assert_eq!(config.source.head, "specify_manual/");
    }

    #[test]
    fn it_fails() {
        let config = SpecTrailConfig::from_file("src/config/missing.toml");
        assert!(config.is_err());
    }

    #[test]
    fn from_file_reads_all_fields_correctly() {
        let toml = r#"
[source]
head = "my_source/"
extension = ".rs"

[document]
head = "my_docs/"
extension = ".md"

[annotation]
prefix = "@my-prefix"
"#;
        let path = write_temp_config("spectrail_test_all_fields.toml", toml);
        let config = SpecTrailConfig::from_file(&path).expect("should parse successfully");
        assert_eq!(config.source.head, "my_source/");
        assert_eq!(config.source.extension, ".rs");
        assert_eq!(config.document.head, "my_docs/");
        assert_eq!(config.document.extension, ".md");
        assert_eq!(config.annotation.prefix, "@my-prefix");
    }

    #[test]
    fn from_file_fails_with_invalid_toml_syntax() {
        let path = write_temp_config("spectrail_test_invalid_toml.toml", "this is not valid toml @@@@");
        let result = SpecTrailConfig::from_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn from_file_fails_when_required_section_is_missing() {
        let toml = r#"
[source]
head = "src/"
extension = ".rs"
"#;
        let path = write_temp_config("spectrail_test_missing_section.toml", toml);
        let result = SpecTrailConfig::from_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn from_file_fails_when_field_is_missing_from_section() {
        // source.extension is intentionally omitted
        let toml = r#"
[source]
head = "src/"

[document]
head = "docs/"
extension = ".md"

[annotation]
prefix = "@st-"
"#;
        let path = write_temp_config("spectrail_test_missing_field.toml", toml);
        let result = SpecTrailConfig::from_file(&path);
        assert!(result.is_err());
    }
}