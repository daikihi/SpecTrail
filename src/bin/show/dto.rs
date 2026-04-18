/// [@st-code-bin-show-dto-file] layer: abstract, type: File, name: dto.rs
use std::fmt;
use std::str::FromStr;

/// [@st-code-bin-show-dto-show-mode] layer: abstract, type: Structure, name: ShowMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowMode {
    List,
}

impl fmt::Display for ShowMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowMode::List => write!(f, "list"),
        }
    }
}

impl std::str::FromStr for ShowMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(ShowMode::List),
            _ => Err(format!("Invalid mode: {}", s)),
        }
    }
}

/// [@st-code-bin-show-dto-show-view] layer: abstract, type: Structure, name: ShowView
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowView {
    Summary,
    List,
    Group,
    Detail,
}

impl fmt::Display for ShowView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowView::Summary => write!(f, "summary"),
            ShowView::List => write!(f, "list"),
            ShowView::Group => write!(f, "group"),
            ShowView::Detail => write!(f, "detail"),
        }
    }
}

impl std::str::FromStr for ShowView {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "summary" => Ok(ShowView::Summary),
            "list" => Ok(ShowView::List),
            "group" => Ok(ShowView::Group),
            "detail" => Ok(ShowView::Detail),
            _ => Err(format!("Invalid view: {}", s)),
        }
    }
}

/// [@st-code-bin-show-dto-show-format] layer: abstract, type: Structure, name: ShowFormat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowFormat {
    Text,
    Json,
}

impl fmt::Display for ShowFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowFormat::Text => write!(f, "text"),
            ShowFormat::Json => write!(f, "json"),
        }
    }
}

impl std::str::FromStr for ShowFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(ShowFormat::Text),
            "json" => Ok(ShowFormat::Json),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

/// [@st-code-bin-show-dto-show-target] layer: abstract, type: Structure, name: ShowTarget
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowTarget {
    All,
    Document,
    Code,
}

impl fmt::Display for ShowTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowTarget::All => write!(f, "all"),
            ShowTarget::Document => write!(f, "document"),
            ShowTarget::Code => write!(f, "code"),
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
            _ => Err(format!("Invalid target: {}", s)),
        }
    }
}

/// [@st-code-bin-show-dto-show-request-dto] layer: abstract, type: Structure, name: ShowRequestDto
#[derive(Debug, Clone)]
pub struct ShowRequestDto {
    pub mode: ShowMode,
    pub target: ShowTarget,
    pub view: ShowView,
    pub format: ShowFormat,
    pub scope: Option<String>,
    /// [@st-code-bin-show-dto-show-request-dto-config-path] layer: abstract, type: Structure, name: config_path
    pub config_path: Option<String>,
}

impl ShowRequestDto {
    pub fn new(mode: ShowMode, target: ShowTarget) -> Self {
        ShowRequestDto {
            mode,
            target,
            view: ShowView::List,
            format: ShowFormat::Text,
            scope: None,
            config_path: None,
        }
    }

    pub fn with_view(mut self, view: ShowView) -> Self {
        self.view = view;
        self
    }

    pub fn with_format(mut self, format: ShowFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_config_path(mut self, config_path: String) -> Self {
        self.config_path = Some(config_path);
        self
    }

    /// Parses command-line style arguments into a `ShowRequestDto`.
    ///
    /// Expects an arguments slice where the first element is the program name (it is skipped),
    /// and recognizes `--mode <value>`, `--target <value>`, and `--scope <value>`. Validates that
    /// `--mode` and `--target` are provided, rejects `--target group`, requires `--scope` when
    /// `--mode search` is used, and rejects `--scope` for modes other than `search`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` with a descriptive message for any of:
    /// - missing value for `--mode`, `--target`, or `--scope` (messages: `"--mode requires a value"`, `"--target requires a value"`, `"--scope requires a value"`),
    /// - unknown argument (`"Unknown argument: <arg>"`),
    /// - missing required flags (`"--mode is required"`, `"--target is required"`),
    /// - unsupported target (`"--target group is not implemented yet"`),
    /// - missing scope with search mode (`"--scope is required with --mode search"`),
    /// - scope provided with a non-search mode (`"--scope is only supported with --mode search"`).
    ///
    /// # Examples
    ///
    /// ```
    /// let argv = vec!["prog".to_string(), "--mode".to_string(), "search".to_string(), "--target".to_string(), "all".to_string(), "--scope".to_string(), "@st-foo".to_string()];
    /// let req = ShowRequestDto::from_args(&argv).unwrap();
    /// assert_eq!(req.mode.to_string(), "search");
    /// assert_eq!(req.target.to_string(), "all");
    /// assert_eq!(req.scope.unwrap(), "@st-foo".to_string());
    /// ```
    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut mode: Option<ShowMode> = None;
        let mut target: Option<ShowTarget> = None;
        let mut view: ShowView = ShowView::List;
        let mut format: ShowFormat = ShowFormat::Text;
        let mut scope: Option<String> = None;
        let mut config_path: Option<String> = None;

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
                "--view" => {
                    view = ShowView::from_str(
                        &args.next().ok_or("--view requires a value")?
                    )?;
                }
                "--format" => {
                    format = ShowFormat::from_str(
                        &args.next().ok_or("--format requires a value")?
                    )?;
                }
                "--scope" => {
                    scope = Some(args.next().ok_or("--scope requires a value")?.to_string());
                }
                "--config" => {
                    config_path = Some(args.next().ok_or("--config requires a value")?.to_string());
                }
                _ => return Err(format!("Unknown argument: {}", arg).into()),
            }
        }

        let mode = mode.ok_or("--mode is required")?;
        let target = target.ok_or("--target is required")?;

        let mut request = ShowRequestDto::new(mode, target)
            .with_view(view)
            .with_format(format);
        if let Some(s) = scope {
            request = request.with_scope(s);
        }
        if let Some(config_path) = config_path {
            request = request.with_config_path(config_path);
        }

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Converts a slice of `&str` into an owned `Vec<String>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let input = ["a", "b", "c"];
    /// let v = args(&input);
    /// assert_eq!(v, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    /// ```
    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn from_args_defaults_view_to_list_and_format_to_text() {
        let result = ShowRequestDto::from_args(&args(&["show", "--mode", "list", "--target", "all"]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.mode, ShowMode::List);
        assert_eq!(req.target, ShowTarget::All);
        assert_eq!(req.view, ShowView::List);
        assert_eq!(req.format, ShowFormat::Text);
        assert!(req.scope.is_none());
    }

    #[test]
    fn from_args_succeeds_with_view_and_format() {
        let result = ShowRequestDto::from_args(&args(&[
            "show",
            "--mode",
            "list",
            "--target",
            "all",
            "--view",
            "detail",
            "--format",
            "json",
        ]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.view, ShowView::Detail);
        assert_eq!(req.format, ShowFormat::Json);
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
    fn show_request_dto_new_has_no_scope() {
        let req = ShowRequestDto::new(ShowMode::List, ShowTarget::All);
        assert_eq!(req.mode, ShowMode::List);
        assert_eq!(req.target, ShowTarget::All);
        assert!(req.scope.is_none());
    }

    #[test]
    fn show_request_dto_with_scope_sets_scope() {
        let req = ShowRequestDto::new(ShowMode::List, ShowTarget::All)
            .with_scope("@st-bar".to_string());
        assert_eq!(req.scope, Some("@st-bar".to_string()));
    }

    #[test]
    fn show_mode_from_str_parses_list() {
        assert_eq!("list".parse::<ShowMode>().unwrap(), ShowMode::List);
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
    }

    #[test]
    fn show_target_from_str_rejects_unknown() {
        assert!("unknown".parse::<ShowTarget>().is_err());
    }

    #[test]
    fn accepts_config_path() {
        let result = ShowRequestDto::from_args(&args(&[
            "show",
            "--mode",
            "list",
            "--target",
            "all",
            "--config",
            "src/config/config.toml",
        ]));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().config_path,
            Some(String::from("src/config/config.toml"))
        );
    }

    #[test]
    fn from_args_fails_when_config_has_no_value() {
        let result = ShowRequestDto::from_args(&args(&[
            "show",
            "--mode",
            "list",
            "--target",
            "all",
            "--config",
        ]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "--config requires a value");
    }

    #[test]
    fn accepts_config_with_other_options() {
        let result = ShowRequestDto::from_args(&args(&[
            "show",
            "--mode",
            "list",
            "--target",
            "all",
            "--scope",
            "@st-test",
            "--config",
            "config/custom.toml",
        ]));
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.mode, ShowMode::List);
        assert_eq!(req.target, ShowTarget::All);
        assert_eq!(req.scope, Some("@st-test".to_string()));
        assert_eq!(req.config_path, Some("config/custom.toml".to_string()));
    }
}