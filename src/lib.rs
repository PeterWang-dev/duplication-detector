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
}
