/// [@st-code-bin-show-main-file] layer: abstract, type: File, name: main.rs
mod dto;

use SpecTrail::domains::services::annotation::scanner::ScanWarning;
use SpecTrail::use_case::show::show_use_case::{
    ShowUseCase, ShowUseCaseRequestDto, ShowUseCaseResponseDto,
};
use dto::ShowRequestDto;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let request_dto: ShowRequestDto = match ShowRequestDto::from_args(&args) {
        Ok(request_dto) => request_dto,
        Err(error) => {
            eprintln!("Error: {}", error);
            return Err(error);
        }
    };

    /* Convert to UseCase Request DTO */
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
            /* Print scan warnings to stderr to notify the user of any malformed annotations or missing links. */
            for warning in &response.warnings {
                match warning {
                    ScanWarning::Parse(pw) => {
                        eprintln!(
                            "WARNING: [{}] Parse error: {} (raw: '{}')",
                            pw.source_file, pw.message, pw.raw_text
                        );
                    }
                    ScanWarning::Resolve(rw) => {
                        eprintln!("WARNING: {}", rw.message);
                    }
                }
            }

            println!("Response: {:#?}", response);
            Ok(())
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            Err(error)
        }
    }
}
