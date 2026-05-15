use SpecTrail::domains::services::annotation::scanner::ScanWarning;

/// Writes each scan warning to standard error as a formatted warning message.
///
/// Each element of `warnings` is rendered into a human-readable warning string and printed on its own line to stderr.
///
/// # Examples
///
/// ```
/// let warnings: &[ScanWarning] = &[];
/// render_warnings(warnings);
/// ```
pub fn render_warnings(warnings: &[ScanWarning]) {
    for warning in warnings {
        eprintln!("{}", format_warning(warning));
    }
}

/// Formats a `ScanWarning` into a user-facing warning message.
///
/// The returned string begins with "warning: " and contains variant-specific
/// details: a parse warning includes the source file and line, while a resolve
/// warning includes the resolve message.
///
/// # Examples
///
/// ```
/// let pw = ParseWarning { source_file: "foo.rs".into(), line: 10 };
/// let w = ScanWarning::Parse(pw);
/// assert_eq!(format_warning(&w), "warning: skipped unknown annotation at foo.rs:10");
///
/// let rw = ResolveWarning { message: "unused symbol".into() };
/// let w2 = ScanWarning::Resolve(rw);
/// assert_eq!(format_warning(&w2), "warning: unused symbol");
/// ```
fn format_warning(warning: &ScanWarning) -> String {
    match warning {
        ScanWarning::Parse(pw) => format!(
            "warning: skipped unknown annotation at {}:{}",
            pw.source_file, pw.line
        ),
        ScanWarning::Resolve(rw) => format!("warning: {}", rw.message),
    }
}
