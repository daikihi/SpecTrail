/// [@st-code-bin-show-dto-file] layer: abstract, type: File, name: dto.rs
use std::fmt;
use std::str::FromStr;

/// [@st-code-bin-show-dto-show-mode] layer: abstract, type: Structure, name: ShowMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowMode {
    List,
    Search,
}

impl fmt::Display for ShowMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowMode::List => write!(f, "list"),
            ShowMode::Search => write!(f, "search"),
        }
    }
}

impl std::str::FromStr for ShowMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(ShowMode::List),
            "search" => Ok(ShowMode::Search),
            _ => Err(format!("Invalid mode: {}", s)),
        }
    }
}

/// [@st-code-bin-show-dto-show-target] layer: abstract, type: Structure, name: ShowTarget
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowTarget {
    All,
    Document,
    Code,
    Group,
}

impl fmt::Display for ShowTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowTarget::All => write!(f, "all"),
            ShowTarget::Document => write!(f, "document"),
            ShowTarget::Code => write!(f, "code"),
            ShowTarget::Group => write!(f, "group"),
        }
    }
}

impl std::str::FromStr for ShowTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(ShowTarget::All),
            "document" => Ok(ShowTarget::Document),
            "code" => Ok(ShowTarget::Code),
            "group" => Ok(ShowTarget::Group),
            _ => Err(format!("Invalid target: {}", s)),
        }
    }
}

/// [@st-code-bin-show-dto-show-request-dto] layer: abstract, type: Structure, name: ShowRequestDto
#[derive(Debug, Clone)]
pub struct ShowRequestDto {
    pub mode: ShowMode,
    pub target: ShowTarget,
    pub scope: Option<String>,
}

impl ShowRequestDto {
    pub fn new(mode: ShowMode, target: ShowTarget) -> Self {
        ShowRequestDto {
            mode,
            target,
            scope: None,
        }
    }

    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut mode: Option<ShowMode> = None;
        let mut target: Option<ShowTarget> = None;
        let mut scope: Option<String> = None;

        let mut args = args.into_iter().skip(1); // Skip program name

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--mode" => {
                    mode = Some(ShowMode::from_str(
                        &args.next().ok_or("--mode requires a value")?
                    )?);
                }
                "--target" => {
                    target = Some(ShowTarget::from_str(
                        &args.next().ok_or("--target requires a value")?
                    )?);
                }
                "--scope" => {
                    scope = Some(args.next().ok_or("--scope requires a value")?.to_string());
                }
                _ => return Err(format!("Unknown argument: {}", arg).into()),
            }
        }

        let mode = mode.ok_or("--mode is required")?;
        let target = target.ok_or("--target is required")?;

        if target == ShowTarget::Group {
            return Err("--target group is not implemented yet".into());
        }

        if mode == ShowMode::Search && scope.is_none() {
            return Err("--scope is required with --mode search".into());
        }

        if mode != ShowMode::Search && scope.is_some() {
            return Err("--scope is only supported with --mode search".into());
        }

        let mut request = ShowRequestDto::new(mode, target);
        if let Some(s) = scope {
            request = request.with_scope(s);
        }

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn rejects_group_target_until_it_is_implemented() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target", "group"]));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "--target group is not implemented yet"
        );
    }

    #[test]
    fn rejects_scope_without_search_mode() {
        let result = ShowRequestDto::from_args(&args(&[
            "show",
            "--mode",
            "list",
            "--target",
            "all",
            "--scope",
            "@st-foo",
        ]));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "--scope is only supported with --mode search"
        );
    }

    #[test]
    fn requires_scope_for_search_mode() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "search", "--target", "all"]));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "--scope is required with --mode search"
        );
    }

    #[test]
    fn from_args_succeeds_with_list_mode_and_all_target() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target", "all"]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.mode, ShowMode::List);
        assert_eq!(req.target, ShowTarget::All);
        assert!(req.scope.is_none());
    }

    #[test]
    fn from_args_succeeds_with_search_mode_and_scope() {
        let result = ShowRequestDto::from_args(&args(&[
            "show", "--mode", "search", "--target", "all", "--scope", "@st-foo",
        ]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.mode, ShowMode::Search);
        assert_eq!(req.target, ShowTarget::All);
        assert_eq!(req.scope, Some("@st-foo".to_string()));
    }

    #[test]
    fn from_args_succeeds_with_document_target() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target", "document"]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.target, ShowTarget::Document);
    }

    #[test]
    fn from_args_succeeds_with_code_target() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target", "code"]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.target, ShowTarget::Code);
    }

    #[test]
    fn from_args_fails_when_mode_is_missing() {
        let result = ShowRequestDto::from_args(&args(&["show", "--target", "all"]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "--mode is required");
    }

    #[test]
    fn from_args_fails_when_target_is_missing() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list"]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "--target is required");
    }

    #[test]
    fn from_args_fails_when_mode_has_no_value() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode"]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "--mode requires a value");
    }

    #[test]
    fn from_args_fails_when_target_has_no_value() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target"]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "--target requires a value");
    }

    #[test]
    fn from_args_fails_when_scope_has_no_value() {
        let result = ShowRequestDto::from_args(&args(&[
            "show", "--mode", "search", "--target", "all", "--scope",
        ]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "--scope requires a value");
    }

    #[test]
    fn from_args_fails_with_unknown_argument() {
        let result = ShowRequestDto::from_args(&args(&[
            "show", "--mode", "list", "--target", "all", "--unknown",
        ]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unknown argument: --unknown");
    }

    #[test]
    fn from_args_fails_with_invalid_mode_value() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "invalid", "--target", "all"]));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid mode: invalid"));
    }

    #[test]
    fn from_args_fails_with_invalid_target_value() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target", "invalid"]));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid target: invalid"));
    }

    #[test]
    fn show_mode_displays_list() {
        assert_eq!(ShowMode::List.to_string(), "list");
    }

    #[test]
    fn show_mode_displays_search() {
        assert_eq!(ShowMode::Search.to_string(), "search");
    }

    #[test]
    fn show_target_displays_all() {
        assert_eq!(ShowTarget::All.to_string(), "all");
    }

    #[test]
    fn show_target_displays_document() {
        assert_eq!(ShowTarget::Document.to_string(), "document");
    }

    #[test]
    fn show_target_displays_code() {
        assert_eq!(ShowTarget::Code.to_string(), "code");
    }

    #[test]
    fn show_target_displays_group() {
        assert_eq!(ShowTarget::Group.to_string(), "group");
    }

    #[test]
    fn show_request_dto_new_has_no_scope() {
        let req = ShowRequestDto::new(ShowMode::List, ShowTarget::All);
        assert_eq!(req.mode, ShowMode::List);
        assert_eq!(req.target, ShowTarget::All);
        assert!(req.scope.is_none());
    }

    #[test]
    fn show_request_dto_with_scope_sets_scope() {
        let req = ShowRequestDto::new(ShowMode::Search, ShowTarget::All)
            .with_scope("@st-bar".to_string());
        assert_eq!(req.scope, Some("@st-bar".to_string()));
    }

    #[test]
    fn show_mode_from_str_parses_list() {
        assert_eq!("list".parse::<ShowMode>().unwrap(), ShowMode::List);
    }

    #[test]
    fn show_mode_from_str_parses_search() {
        assert_eq!("search".parse::<ShowMode>().unwrap(), ShowMode::Search);
    }

    #[test]
    fn show_mode_from_str_rejects_unknown() {
        assert!("unknown".parse::<ShowMode>().is_err());
    }

    #[test]
    fn show_target_from_str_parses_all_variants() {
        assert_eq!("all".parse::<ShowTarget>().unwrap(), ShowTarget::All);
        assert_eq!("document".parse::<ShowTarget>().unwrap(), ShowTarget::Document);
        assert_eq!("code".parse::<ShowTarget>().unwrap(), ShowTarget::Code);
        assert_eq!("group".parse::<ShowTarget>().unwrap(), ShowTarget::Group);
    }

    #[test]
    fn show_target_from_str_rejects_unknown() {
        assert!("unknown".parse::<ShowTarget>().is_err());
    }
}