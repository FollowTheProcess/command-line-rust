use clap::Parser;
use fileinfo::count;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

mod fileinfo;

const LONG_ABOUT: &str = "A head clone written in Rust.";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = LONG_ABOUT)]
struct CLI {
    /// Input file(s)
    #[clap(value_parser, default_value = "-")]
    files: Vec<String>,

    /// Count lines
    #[clap(short, long, action, default_value = "false")]
    lines: bool,

    /// Count words
    #[clap(short, long, action, default_value = "false")]
    words: bool,

    /// Count bytes
    #[clap(short, long, action, default_value = "false", conflicts_with = "chars")]
    bytes: bool,

    /// Count characters
    #[clap(short, long, action, default_value = "false", conflicts_with = "bytes")]
    chars: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut cli = CLI::parse();

    if [cli.lines, cli.words, cli.bytes, cli.chars]
        .iter()
        .all(|v| v == &false)
    {
        cli.lines = true;
        cli.words = true;
        cli.bytes = true;
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &cli.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.lines, cli.lines),
                        format_field(info.words, cli.words),
                        format_field(info.bytes, cli.bytes),
                        format_field(info.chars, cli.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += info.lines;
                    total_words += info.words;
                    total_bytes += info.bytes;
                    total_chars += info.chars;
                }
            }
        }
    }

    if cli.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, cli.lines),
            format_field(total_words, cli.words),
            format_field(total_bytes, cli.bytes),
            format_field(total_chars, cli.chars),
        )
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format! {"{:>8}", value}
    } else {
        "".to_string()
    }
}
