mod dto;

use SpecTrail::config::SpecTrailConfig;
use dto::{ShowRequestDto, ShowMode, ShowTarget};
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let request = parse_args(&args)?;
    
    let toml_file_name: &str = "src/config/config.toml";
    let config: SpecTrailConfig = SpecTrailConfig::from_file(toml_file_name)?;

    println!("Request: {:#?}", request);
    println!("Config: {:#?}", config);
    Ok(())
}

fn parse_args(args: &[String]) -> Result<ShowRequestDto, Box<dyn std::error::Error>> {
    let mut mode: Option<ShowMode> = None;
    let mut target: Option<ShowTarget> = None;
    let mut scope: Option<String> = None;

    let mut i = 1; // Skip program name
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => {
                i += 1;
                if i < args.len() {
                    mode = Some(ShowMode::from_str(&args[i])?);
                } else {
                    return Err("--mode requires a value".into());
                }
            }
            "--target" => {
                i += 1;
                if i < args.len() {
                    target = Some(ShowTarget::from_str(&args[i])?);
                } else {
                    return Err("--target requires a value".into());
                }
            }
            "--scope" => {
                i += 1;
                if i < args.len() {
                    scope = Some(args[i].clone());
                } else {
                    return Err("--scope requires a value".into());
                }
            }
            _ => return Err(format!("Unknown argument: {}", args[i]).into()),
        }
        i += 1;
    }

    let mode = mode.ok_or("--mode is required")?;
    let target = target.ok_or("--target is required")?;

    let mut request = ShowRequestDto::new(mode, target);
    if let Some(s) = scope {
        request = request.with_scope(s);
    }

    Ok(request)
}
