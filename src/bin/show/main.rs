mod dto;

use SpecTrail::config::SpecTrailConfig;
use dto::ShowRequestDto;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let request = ShowRequestDto::from_args(&args)?;
    
    let toml_file_name: &str = "src/config/config.toml";
    let config: SpecTrailConfig = SpecTrailConfig::from_file(toml_file_name)?;

    println!("Request: {:#?}", request);
    println!("Config: {:#?}", config);
    Ok(())
}
