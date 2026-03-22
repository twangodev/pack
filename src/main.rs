use clap::Parser;
use std::path::PathBuf;

use pack::parse::{self, ParseConfig};

#[derive(Parser, Debug)]
#[command(name = "pack")]
struct Args {
    /// Input markdown file
    input: PathBuf,

    /// Markdown heading level to split sections on
    #[arg(long, default_value = "##")]
    split_on: String,

    /// Max width in characters for wrapping
    #[arg(short, long, default_value_t = 40)]
    width: usize,

    /// Output PDF file path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let markdown = std::fs::read_to_string(&args.input).expect("Failed to read input file");

    let split_level = match args.split_on.as_str() {
        "#" => 1u8,
        "##" => 2,
        "###" => 3,
        other => panic!("Unsupported split level: {}", other),
    };

    let config = ParseConfig {
        split_level,
        max_width: args.width,
    };

    let pieces = parse::extract_pieces(&markdown, &config);
    let board = pack::tui::run(pieces.clone()).expect("TUI error");

    pack::export::to_pdf(&board, &pieces, args.output.to_str().unwrap())
        .expect("Failed to render PDF");
    eprintln!("Written to {}", args.output.display());
}
