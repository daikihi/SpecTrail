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

        let mut request = ShowRequestDto::new(mode, target);
        if let Some(s) = scope {
            request = request.with_scope(s);
        }

        Ok(request)
    }
}
