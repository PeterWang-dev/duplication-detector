use clap::ArgMatches;
use duplicate_detector::Config;

pub fn to_config(matches: ArgMatches) -> Config {
    let original_path = matches.get_one::<String>("original_file").unwrap();
    let input_path = matches.get_one::<String>("input_file").unwrap();
    let output_path = matches.get_one::<String>("output_file").unwrap();

    Config::from(original_path, input_path, output_path)
}

#[cfg(test)]
mod tests {
    use super::to_config;

    #[test]
    fn test_to_config() {
        use clap::Arg;

        let args = vec![
            "duplicate-detector",
            "./original.txt",
            "./input.txt",
            "./output.txt",
        ];

        // Manually create a clap::ArgMatches object
        let matches = clap::Command::new("duplicate-detector")
            .arg(Arg::new("original_file").required(true))
            .arg(Arg::new("input_file").required(true))
            .arg(Arg::new("output_file").required(true)).get_matches_from(&args);

        let config = to_config(matches);

        assert_eq!(config.original_path().to_str().unwrap(), "./original.txt");
        assert_eq!(config.input_path().to_str().unwrap(), "./input.txt");
        assert_eq!(config.output_path().to_str().unwrap(), "./output.txt");
    }
}
