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
}
