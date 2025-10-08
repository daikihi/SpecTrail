use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SpecTrailConfig {
    pub source: SourceConfig,
    pub document: DocumentConfig,
}

impl SpecTrailConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: SpecTrailConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub head: String,
    pub extension: String,
}

#[derive(Debug, Deserialize)]
pub struct DocumentConfig {
    pub head: String,
    pub extension: String,
}
