/// [@st-code-bin-show-main-file] layer: abstract, type: File, name: main.rs
mod dto;
mod show_request_adapter;
mod show_response_adapter;
mod output;

use dto::ShowRequestDto;
use show_request_adapter::adapt_request;
use show_response_adapter::adapt_response;
use SpecTrail::use_case::show::show_use_case::ShowUseCase;
use std::env;
use std::process;

/// Entry point for the `show` CLI: parses command-line arguments, executes the Show use case, and prints any warnings and a compact summary of results.
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let request_dto: ShowRequestDto = match ShowRequestDto::from_args(&args) {
        Ok(request_dto) => request_dto,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    let use_case = ShowUseCase::new();
    let response = use_case.execute(adapt_request(&request_dto));

    match response {
        Ok(response) => {
            let view_model = adapt_response(response, request_dto.view, request_dto.format);
            output::render(&view_model);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}
