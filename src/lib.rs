use std::{error::Error, fs, io, path::PathBuf, f64::INFINITY};

pub mod preprocessor;
pub mod processor;

#[derive(Debug)]
pub struct Config {
    original_path: PathBuf,
    input_path: PathBuf,
    output_path: PathBuf,
}

impl Config {
    pub fn from(original_path: &str, input_path: &str, output_path: &str) -> Self {
        Config {
            original_path: PathBuf::from(original_path),
            input_path: PathBuf::from(input_path),
            output_path: PathBuf::from(output_path),
        }
    }

    pub fn original_path(&self) -> &PathBuf {
        &self.original_path
    }

    pub fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    pub fn output_path(&self) -> &PathBuf {
        &self.output_path
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Print the inputs from CLI.
    // No errors should occur here as the inputs are definately exists.
    println!(
        "Original file: {}",
        config.original_path().to_str().unwrap()
    );
    println!("Input file: {}", config.input_path().to_str().unwrap());

    // Pre-process: Change lines into a single string. Errors are propagated.
    let original_string = preprocessor::trim_and_convert(config.original_path())?;
    let input_string = preprocessor::trim_and_convert(config.input_path())?;

    // Process, calculate and print the ratio. No errors should occur here.
    let mut dector = processor::Detector::new(original_string, input_string);
    dector.compute_ratio();

    let ratio: f64 = match dector.duplicate_ratio() {
        Some(r) => {
            println!("Ratio: {:.2}", r);
            r
        }
        None => {
            println!("Ratio: None");
            INFINITY
        }
    };

    // Write the ratio to the output file. Errors are propagated.
    let mut output_file = fs::File::create(config.output_path())?;
    let status = io::Write::write_fmt(&mut output_file, format_args!("{ratio:.2}"));

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from() {
        let config = Config::from("./original.txt", "./input.txt", "./output.txt");

        assert_eq!(config.original_path, PathBuf::from("./original.txt"));
        assert_eq!(config.input_path, PathBuf::from("./input.txt"));
        assert_eq!(config.output_path, PathBuf::from("./output.txt"));
    }

    #[test]
    fn test_config_getter() {
        let config = Config::from("./original.txt", "./input.txt", "./output.txt");

        assert_eq!(config.original_path(), &PathBuf::from("./original.txt"));
        assert_eq!(config.input_path(), &PathBuf::from("./input.txt"));
        assert_eq!(config.output_path(), &PathBuf::from("./output.txt"));
    }
}
