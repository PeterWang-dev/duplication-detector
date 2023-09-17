use clap::{Arg, Command};

mod cli_parser;
mod controller;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("duplicate-detector")
        .version("0.1.0")
        .author("PeterWang-dev <PeterWang030908@hotmail.com>")
        .about(
            "Test the duplication rate of the input file based on the original file. \
            Output will be stored in a file which path is specified by the user.",
        )
        .arg(
            Arg::new("original_file")
                .help("The path of reference file for duplication detector")
                .required(true),
        )
        .arg(
            Arg::new("input_file")
                .help("The path of input file")
                .required(true),
        )
        .arg(
            Arg::new("output_file")
                .help("The path where it stores the ratio of duplication")
                .required(true),
        ).get_matches();

    let config = cli_parser::to_config(matches);

    if let Err(e) = controller::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

