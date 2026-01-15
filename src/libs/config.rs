use serde::Deserialize;
use std::{fs, io};
use std::path::Path;

/**
 * configuration loader from a toml file using with parameter
 */
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
