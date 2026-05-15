use SpecTrail::domains::services::annotation::scanner::ScanWarning;

pub fn render_warnings(warnings: &[ScanWarning]) {
    for warning in warnings {
        eprintln!("{}", format_warning(warning));
    }
}

fn format_warning(warning: &ScanWarning) -> String {
    match warning {
        ScanWarning::Parse(pw) => format!(
            "warning: skipped unknown annotation at {}:{}",
            pw.source_file, pw.line
        ),
        ScanWarning::Resolve(rw) => format!("warning: {}", rw.message),
    }
}
