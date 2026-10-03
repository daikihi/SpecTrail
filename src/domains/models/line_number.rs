use std::fmt;

/// [@st-code-domain-models-line-number] layer: abstract, type: Structure, name: LineNumber
/// Represents a 1-based source code or document line number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineNumber(pub usize);

impl LineNumber {
    pub fn new(line: usize) -> Self {
        Self(line)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl fmt::Display for LineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for LineNumber {
    fn from(line: usize) -> Self {
        Self(line)
    }
}
