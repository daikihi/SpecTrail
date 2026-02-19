/// [@st-code-bin-show-main-file] layer: abstract, type: File, name: main.rs
mod dto;

use SpecTrail::config::SpecTrailConfig;
use SpecTrail::use_case::show::show_use_case::{
    ShowUseCase, ShowUseCaseRequestDto, ShowUseCaseResponseDto,
};
use dto::ShowRequestDto;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let request_dto: ShowRequestDto = match ShowRequestDto::from_args(&args) {
        Ok(request_dto) => {
            println!("Request: {:#?}", request_dto);
            request_dto
        }
        Err(error) => {
            println!("Error: {}", error);
            return Err(error);
        }
    };

    // fixme : config file name should not be fixed on a code
    let toml_file_name: &str = "src/config/config.toml";
    // @todo config is not used in this command
    let _config: SpecTrailConfig = match SpecTrailConfig::from_file(toml_file_name) {
        Ok(config) => {
            println!("Config: {:#?}", config);
            config
        }
        Err(error) => {
            println!("Error: {}", error);
            return Err(error);
        }
    };

    // Convert to UseCase Request DTO
    let use_case_request = ShowUseCaseRequestDto {
        mode: request_dto.mode.to_string(),
        target: request_dto.target.to_string(),
        scope: request_dto.scope.clone(),
    };

    let use_case: ShowUseCase = ShowUseCase::new();
    let response: Result<ShowUseCaseResponseDto, Box<dyn std::error::Error>> =
        use_case.execute(use_case_request);

    match response {
        Ok(response) => {
            println!("Response: {:#?}", response);
            Ok(())
        }
        Err(error) => {
            println!("Error: {}", error);
            Err(error)
        }
    }
}
