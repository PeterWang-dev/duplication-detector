use clap::Command;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    original_path: PathBuf,
    input_path: PathBuf,
    output_path: PathBuf,
}

impl Config {
    pub fn from_command(command: Command) -> Self {
        let matches = command.get_matches();

        Config {
            original_path: PathBuf::from(matches.get_one::<String>("original_file").unwrap()),
            input_path: PathBuf::from(matches.get_one::<String>("input_file").unwrap()),
            output_path: PathBuf::from(matches.get_one::<String>("output_file").unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    #[test]
    fn test_config_from_command() {
        use super::Config;
        use clap::Arg;

        // Create test files
        fs::write("./original.txt", "original").unwrap();
        fs::write("./input.txt", "input").unwrap();
        fs::write("./output.txt", "output").unwrap();

        // Manually create a clap::Command
        let command = clap::Command::new("duplicate-detector")
            .arg(Arg::new("original_file").default_value("./original.txt"))
            .arg(Arg::new("input_file").default_value("./input.txt"))
            .arg(Arg::new("output_file").default_value("./output.txt"));

        let config = Config::from_command(command);

        assert_eq!(config.original_path, PathBuf::from("./original.txt"));
        assert_eq!(config.input_path, PathBuf::from("./input.txt"));
        assert_eq!(config.output_path, PathBuf::from("./output.txt"));

        // Clean up
        fs::remove_file("./original.txt").unwrap();
        fs::remove_file("./input.txt").unwrap();
        fs::remove_file("./output.txt").unwrap();
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    dbg!(config);
    Ok(())
}
