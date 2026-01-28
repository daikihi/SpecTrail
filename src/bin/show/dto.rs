use std::fmt;

/// Show command modes
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

/// Show command targets
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

/// Data Transfer Object for show command requests
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
}
