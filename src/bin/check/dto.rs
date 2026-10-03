use std::env;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckFormat {
    Text,
    Github,
    Json,
}

impl fmt::Display for CheckFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckFormat::Text => write!(f, "text"),
            CheckFormat::Github => write!(f, "github"),
            CheckFormat::Json => write!(f, "json"),
        }
    }
}

impl FromStr for CheckFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(CheckFormat::Text),
            "github" => Ok(CheckFormat::Github),
            "json" => Ok(CheckFormat::Json),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckRequestDto {
    pub strict: bool,
    pub warn_unimplemented: bool,
    pub format: CheckFormat,
    pub config_path: Option<String>,
}

impl CheckRequestDto {
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut strict = false;
        let mut warn_unimplemented = false;
        let mut format_opt = None;
        let mut config_path = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--strict" => {
                    strict = true;
                    i += 1;
                }
                "--warn-unimplemented" => {
                    warn_unimplemented = true;
                    i += 1;
                }
                "--format" => {
                    if i + 1 >= args.len() {
                        return Err("--format requires a value".to_string());
                    }
                    let fmt = args[i + 1].parse::<CheckFormat>()?;
                    format_opt = Some(fmt);
                    i += 2;
                }
                "--config" => {
                    if i + 1 >= args.len() {
                        return Err("--config requires a value".to_string());
                    }
                    config_path = Some(args[i + 1].clone());
                    i += 2;
                }
                unknown => {
                    return Err(format!("Unknown option: {}", unknown));
                }
            }
        }

        let format = format_opt.unwrap_or_else(|| {
            if env::var("GITHUB_ACTIONS").map(|v| v == "true").unwrap_or(false) {
                CheckFormat::Github
            } else {
                CheckFormat::Text
            }
        });

        Ok(CheckRequestDto {
            strict,
            warn_unimplemented,
            format,
            config_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_args(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_flags_correctly() {
        let args = to_args(&["check", "--strict", "--warn-unimplemented", "--format", "json"]);
        let req = CheckRequestDto::from_args(&args).unwrap();
        assert!(req.strict);
        assert!(req.warn_unimplemented);
        assert_eq!(req.format, CheckFormat::Json);
    }

    #[test]
    fn parses_defaults() {
        let args = to_args(&["check"]);
        let req = CheckRequestDto::from_args(&args).unwrap();
        assert!(!req.strict);
        assert!(!req.warn_unimplemented);
    }
}
