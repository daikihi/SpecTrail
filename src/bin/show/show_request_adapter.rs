use crate::dto::ShowRequestDto;
use SpecTrail::use_case::show::show_use_case::ShowUseCaseRequestDto;

pub fn adapt_request(request: &ShowRequestDto) -> ShowUseCaseRequestDto {
    ShowUseCaseRequestDto {
        mode: request.mode.to_string(),
        target: request.target.to_string(),
        scope: request.scope.clone(),
        config_path: request.config_path.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{ShowMode, ShowRequestDto, ShowTarget};

    #[test]
    fn adapts_list_request_without_optional_fields() {
        let request = ShowRequestDto::new(ShowMode::List, ShowTarget::All);

        let adapted = adapt_request(&request);

        assert_eq!(adapted.mode, "list");
        assert_eq!(adapted.target, "all");
        assert_eq!(adapted.scope, None);
        assert_eq!(adapted.config_path, None);
    }

    #[test]
    fn preserves_scope_and_config_path() {
        let request = ShowRequestDto::new(ShowMode::Search, ShowTarget::Document)
            .with_scope("@st-foo".to_string())
            .with_config_path("src/config/config.toml".to_string());

        let adapted = adapt_request(&request);

        assert_eq!(adapted.mode, "search");
        assert_eq!(adapted.target, "document");
        assert_eq!(adapted.scope, Some("@st-foo".to_string()));
        assert_eq!(
            adapted.config_path,
            Some("src/config/config.toml".to_string())
        );
    }
}
