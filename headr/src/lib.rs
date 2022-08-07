use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

const LONG_ABOUT: &str = "A cat clon&e written in Rust.";

/// CLIResult encodes a result of executing the catr CLI
type CLIResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct Cli {
    /// Input file(s)
    #[clap(value_parser, default_value = "-")]
    files: Vec<String>,
    /// Number of lines to print
    #[clap(
        short = 'n',
        long = "lines",
        default_value_t = 10,
        conflicts_with = "bytes",
        value_parser
    )]
    lines: usize,
    /// Number of bytes to print
    #[clap(short = 'c', long = "bytes", conflicts_with = "lines", value_parser)]
    bytes: Option<usize>,
}

pub fn run() -> CLIResult<()> {
    let cli = Cli::parse();
    let num_files = cli.files.len();
    for (file_num, filename) in cli.files.iter().enumerate() {
        match open(filename) {
            Err(err) => {
                eprintln!("Failed to open {}: {}", filename, err)
            }
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = cli.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]))
                } else {
                    let mut line = String::new();
                    for _ in 0..cli.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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
