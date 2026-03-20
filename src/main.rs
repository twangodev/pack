use clap::Parser;
use std::path::PathBuf;

use pack::extract::{self, ExtractConfig, render_outline};

#[derive(Parser, Debug)]
#[command(
    name = "pack",
)]
struct Args {
    /// Input markdown file
    input: PathBuf,

    /// Markdown heading level to split sections on
    #[arg(long, default_value = "##")]
    split_on: String,

    /// Max width in characters for wrapping
    #[arg(short, long, default_value_t = 40)]
    width: usize,
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

    let config = ExtractConfig {
        split_level,
        max_width: args.width,
    };

    let pieces = extract::extract_pieces(&markdown, &config);
    eprintln!("Parsed {} pieces\n", pieces.len());

    for p in &pieces {
        for row in render_outline(p) {
            eprintln!("{}", row);
        }
        eprintln!();
    }
}
