use serde::Deserialize;
use std::{fs, io};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SpecTrailConfig {
    pub source: SourceConfig,
    pub document: DocumentConfig,
}

impl SpecTrailConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content_res: io::Result<String> = fs::read_to_string(path);
        match content_res {
            Ok(content) => {
                let config_res: Result<SpecTrailConfig, toml::de::Error> = toml::from_str(&content);
                match config_res {
                    Ok(config) => Ok(config),
                    Err(e) => {
                        println!("{:?}", e);
                        Err(Box::new(e))
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
                Err(Box::new(e))
            }
        }
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
