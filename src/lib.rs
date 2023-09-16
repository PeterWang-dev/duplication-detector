use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io};

    #[test]
    fn test_config() {
        let config = Config::from("./original.txt", "./input.txt", "./output.txt");

        assert_eq!(config.original_path, PathBuf::from("./original.txt"));
        assert_eq!(config.input_path, PathBuf::from("./input.txt"));
        assert_eq!(config.output_path, PathBuf::from("./output.txt"));

        assert_eq!(config.original_path(), &PathBuf::from("./original.txt"));
        assert_eq!(config.input_path(), &PathBuf::from("./input.txt"));
        assert_eq!(config.output_path(), &PathBuf::from("./output.txt"));
    }

    // #[test]
    // fn test_full_functionalities() {
    //     // Create test files
    //     fs::write("./original.txt", "这是一个测试文档，用于测试软件的功能。").unwrap();
    //     fs::write("./input.txt", "它是测试文档，来测试软件功能").unwrap();

    //     let original_string = preprocessor::trim_and_convert("./original.txt").unwrap();
    //     let input_string = preprocessor::trim_and_convert("./input.txt").unwrap();

    //     let dector = processor::detector::new(original_string, input_string);
    //     let ratio: f64 = dector.get_ratio();

    //     let mut output_file = fs::File::create(config.output_path).unwrap();
    //     io::Write::write_fmt(&mut output_file, format_args!("{ratio:.2}"));

    //     assert_eq!(
    //         fs::read_to_string("./output.txt").unwrap().parse::<f64>() - 0.90,
    //         0.10
    //     );

    //     // Clean up
    //     fs::remove_file("./original.txt").unwrap();
    //     fs::remove_file("./input.txt").unwrap();
    //     fs::remove_file("./output.txt").unwrap();
    // }
}
