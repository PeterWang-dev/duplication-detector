use std::error::Error;

use duplicate_detector::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    dbg!(config);
    Ok(())
}