use SpecTrail::config::SpecTrailConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_file_name: &str = "src/config/config.toml";
    let config: SpecTrailConfig = SpecTrailConfig::from_file(toml_file_name)?;

    println!("{:#?}", config);
    Ok(())
}
