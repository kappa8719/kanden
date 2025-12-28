use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
enum SourceFormat {
    Pumpkin,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Source format of extracted data
    #[clap(long)]
    source_format: SourceFormat,
    /// Source directory of extracted data
    #[clap(long)]
    source: PathBuf,
    /// Output directory of converted data
    #[clap(long)]
    output: PathBuf,
}

fn main() {
    let args = Cli::parse();

    match args.source_format {
        SourceFormat::Pumpkin => convert_from_pumpkin(args.source, args.output),
    }
}

fn convert_from_pumpkin(source: PathBuf, output: PathBuf) {
    source.join("")
}
