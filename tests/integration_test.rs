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

    assert_eq!(result_string.parse::<f64>().unwrap() - 0.90, 0.10);

    // Clean up
    fs::remove_file("./original.txt").unwrap();
    fs::remove_file("./input.txt").unwrap();
    fs::remove_file("./output.txt").unwrap();
}
