use serde::Deserialize;
use std::fs;
use std::path::Path;

/**
 * configuration loader from a toml file using with parameter
 */
#[derive(Debug, Deserialize)]
pub struct SpecTrailConfig {
    pub source: SourceConfig,
    pub document: DocumentConfig,
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
#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub head: String,
    pub extension: String,
}

/**
 * Using for document specs
 */
#[derive(Debug, Deserialize)]
pub struct DocumentConfig {
    pub head: String,
    pub extension: String,
}

/**
 * Using for annotation specs
 */
#[derive(Debug, Deserialize)]
pub struct AnnotationConfig {
    pub prefix: String,
}

#[cfg(test)]
mod tests_spec_trail_config {
    use super::*;

    #[test]
    fn it_works() {
        let config_result: Result<SpecTrailConfig, _> = SpecTrailConfig::from_file("src/config/config.toml");
        assert!(config_result.is_ok());
        let config = config_result.unwrap();
        println!("{:#?}", config);
        assert_eq!(config.source.head, ".specify/");
    }

    #[test]
    fn it_fails() {
        let config = SpecTrailConfig::from_file("src/config/config_fail.toml");
        assert!(config.is_err());
    }
}
