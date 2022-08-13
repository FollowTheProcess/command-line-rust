use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

const LONG_ABOUT: &str = "A uniq clone written in Rust.";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Read input from file, defaults to stdin
    #[clap(value_parser, default_value = "-")]
    input_file: String,

    /// Write output to file, defaults to stdout
    #[clap(value_parser)]
    output_file: Option<String>,

    /// Count unique lines
    #[clap(short, long)]
    count: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut file = open(&cli.input_file).map_err(|e| format!("{}: {}", cli.input_file, e))?;
    let mut output_file: Box<dyn Write> = match &cli.output_file {
        Some(name) => Box::new(File::create(name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> Result<(), io::Error> {
        if count > 0 {
            if cli.count {
                write!(output_file, "{:>4} {}", count, text)?;
            } else {
                write!(output_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut current = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = file.read_line(&mut current)?;
        if bytes == 0 {
            break;
        }

        if current.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = current.clone();
            count = 0;
        }
        count += 1;
        current.clear();
    }

    print(count, &previous)?;
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
