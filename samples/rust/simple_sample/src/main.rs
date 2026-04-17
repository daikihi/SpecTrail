/// [@ss-simple-sample] layer: meta, type: Philosophy, name: Simple Sample
/// [@ss-simple-sample-presentation] layer: abstract, type: Philosophy, name: Presentation
/// [@ss-simple-sample-requirement-1] layer: abstract, type: Philosophy, name: Requirement 1
/// [@ss-simple-sample-structure-main] layer: abstract, type: Structure, name: Main Structure

mod application;
mod domain;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let name = env::args().nth(1).unwrap_or_default();
    match application::run(name) {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}