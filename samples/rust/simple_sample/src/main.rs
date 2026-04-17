/// [@ss-simple-sample] layer: implementation, type: Structure, name: Simple Sample
/// [@ss-simple-sample-presentation] layer: implementation, type: Structure, name: Presentation
/// [@ss-simple-sample-requirement-1] layer: implementation, type: Structure, name: Requirement 1
/// [@ss-simple-sample-structure-main] layer: implementation, type: Structure, name: Main Structure

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
