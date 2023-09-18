use duplication_detector::{preprocessor::trim_and_convert, processor::Detector, Config};
use std::{error::Error, fs, io};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Print the inputs from CLI.
    // No errors should occur here as the inputs are definately exists.
    println!(
        "Original file: {}",
        config.original_path().to_str().unwrap()
    );
    println!("Input file: {}", config.input_path().to_str().unwrap());

    // Pre-process: Change lines into a single string. Errors are propagated.
    let original_string = trim_and_convert(config.original_path())?;
    let input_string = trim_and_convert(config.input_path())?;

    // Process, calculate and print the ratio. No errors should occur here.
    let mut dector = Detector::new(original_string, input_string);
    let ratio: f64 = dector.compute_ratio();
    println!("Ratio: {:.2}", ratio);

    // Write the ratio to the output file. Errors are propagated.
    let mut output_file = fs::File::create(config.output_path())?;
    let status = io::Write::write_fmt(&mut output_file, format_args!("{ratio:.2}"));

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
