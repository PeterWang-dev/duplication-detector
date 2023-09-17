use duplicate_detector::{preprocessor, processor, Config};
use std::{fs, io};

#[test]
fn test_full_functionalities() {
    let config = Config::from("./original.txt", "./input.txt", "./output.txt");

    // Create test files
    fs::write(
        config.original_path(),
        "这是一个测试文档，用于测试软件的功能。",
    )
    .unwrap();
    fs::write(config.input_path(), "它是测试文档，来测试软件功能").unwrap();

    let original_string = preprocessor::trim_and_convert(config.original_path()).unwrap();
    let input_string = preprocessor::trim_and_convert(config.input_path()).unwrap();

    let mut dector = processor::Detector::new(original_string, input_string);
    let ratio: f64 = dector.compute_ratio();

    let mut output_file = fs::File::create(config.output_path()).unwrap();
    io::Write::write_fmt(&mut output_file, format_args!("{ratio:.2}")).unwrap();

    let result_string = fs::read_to_string(config.output_path()).unwrap();

    assert!(result_string.parse::<f64>().unwrap() > 0.50);

    // Clean up
    fs::remove_file(config.original_path()).unwrap();
    fs::remove_file(config.input_path()).unwrap();
    fs::remove_file(config.output_path()).unwrap();
}

#[test]
fn test_with_example_inputs() {
    // Use the example inputs from example/inputs.
    let config = Config::from(
        "examples/inputs/orig.txt",
        "examples/inputs/orig_0.8_add.txt",
        "examples/output.txt",
    );

    let original_string = preprocessor::trim_and_convert(config.original_path()).unwrap();
    let input_string = preprocessor::trim_and_convert(config.input_path()).unwrap();

    let mut dector = processor::Detector::new(original_string, input_string);
    let ratio: f64 = dector.compute_ratio();

    if fs::read(config.output_path()).is_ok() {
        fs::remove_file(config.output_path()).unwrap();
    }

    let mut output_file = fs::File::create(config.output_path()).unwrap();
    io::Write::write_fmt(&mut output_file, format_args!("{ratio:.2}")).unwrap();

    let result_string = fs::read_to_string(config.output_path()).unwrap();
    let result = result_string.parse::<f64>().unwrap();

    assert!( result > 0.80 && result < 0.90);

    // Clean up
    fs::remove_file(config.output_path()).unwrap();
}
