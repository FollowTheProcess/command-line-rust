use clap::Parser;

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Echo command line arguments to stdout"
)] // Read from `Cargo.toml`
struct Cli {
    /// Input text
    #[clap(value_parser)]
    text: Vec<String>,
    /// Do not print new line
    #[clap(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    print!(
        "{}{}",
        cli.text.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    );
}
