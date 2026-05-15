mod text;
mod warning;

use crate::show_response_adapter::ShowResponseView;
use crate::dto::ShowFormat;

pub fn render(view_model: &ShowResponseView) {
    // Render warnings to stderr if any
    if !view_model.warnings.is_empty() {
        warning::render_warnings(&view_model.warnings);
    }

    match view_model.format {
        ShowFormat::Text => {
            text::render_text(view_model);
        }
        ShowFormat::Json => {
            // TODO: Implement JSON rendering
            eprintln!("Error: JSON format is not yet implemented.");
        }
    }
}
