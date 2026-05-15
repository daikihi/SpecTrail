mod text;
mod warning;

use crate::show_response_adapter::ShowResponseView;
use crate::dto::ShowFormat;

/// Render a `ShowResponseView` according to its selected output format and emit any warnings.
///
/// If the view model contains warnings, they are written to stderr via the `warning` module.
/// The function dispatches on `view_model.format`: it calls the text renderer for `Text`
/// and prints an error message to stderr for `Json` (JSON rendering is not implemented).
///
/// # Arguments
///
/// * `view_model` - The view model containing the data to render, any collected warnings, and the chosen output format.
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
