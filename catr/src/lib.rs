use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

const LONG_ABOUT: &str = "A cat clone written in Rust.";

/// CLIResult encodes a result of executing the catr CLI
type CLIResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Input file(s)
    #[clap(value_parser)]
    files: Vec<String>,
    /// Show line numbers
    #[clap(short = 'n', long = "number", conflicts_with = "number-non-blank")]
    number_lines: bool,
    /// Number non-blank lines
    #[clap(short = 'b', long = "number-nonblank", conflicts_with = "number-lines")]
    number_non_blank: bool,
}

pub fn run() -> CLIResult<()> {
    let cli = Cli::parse();
    for filename in cli.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if cli.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if cli.number_non_blank {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> CLIResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
