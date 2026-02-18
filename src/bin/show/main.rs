/// [@st-code-bin-show-main-file] layer: abstract, type: File, name: main.rs
mod dto;

use SpecTrail::config::SpecTrailConfig;
use SpecTrail::use_case::show::show_use_case::{ShowUseCase, ShowUseCaseRequestDto};
use dto::ShowRequestDto;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let request_dto = ShowRequestDto::from_args(&args)?;

    let toml_file_name: &str = "src/config/config.toml";
    let _config: SpecTrailConfig = SpecTrailConfig::from_file(toml_file_name)?;

    // UseCase Request DTO への変換
    let use_case_request = ShowUseCaseRequestDto {
        mode: request_dto.mode.to_string(),
        target: request_dto.target.to_string(),
        scope: request_dto.scope.clone(),
    };

    let use_case = ShowUseCase::new();
    let response = use_case.execute(use_case_request)?;

    println!("Response: {:#?}", response);
    Ok(())
}
