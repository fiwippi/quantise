use clap::Parser;
use image::ImageReader;

use quantise::{palette, quantise};

/// Program to quantise an image
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input filepath
    #[arg(short, long, required = true)]
    input: String,

    /// Output filepath
    #[arg(short, long, required = true)]
    output: String,

    /// How many discrete greyscale hues to quantise into
    #[arg(short, long, required = true)]
    colours: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let input = ImageReader::open(args.input)?.decode()?;
    quantise::<palette::V2>(&input, args.colours).save(args.output)?;

    Ok(())
}
