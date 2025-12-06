use std::path::PathBuf;

use clap::Parser;

use filmspec::{Processor, ProcessorConfig, Result, SampleMode, StackMode};

#[derive(Parser)]
#[command(name = "filmspec")]
#[command(about = "Generate film spectrum from video files")]
#[command(version)]
struct Cli {
    /// Input video file path
    input: PathBuf,

    /// Output image path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output image width in pixels
    #[arg(short, long, default_value = "1920")]
    width: u32,

    /// Output image height in pixels
    #[arg(short = 'H', long, default_value = "300")]
    height: u32,

    /// Layout direction: h (horizontal, left to right) or v (vertical, top to bottom)
    #[arg(short, long, default_value = "h")]
    layout: String,

    /// Slice direction: row (middle horizontal line) or col (middle vertical line)
    #[arg(short, long, default_value = "row")]
    slice: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_path = cli.output.unwrap_or_else(|| {
        let stem = cli
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("spectrum");
        PathBuf::from(format!("{}_spectrum.png", stem))
    });

    let sample_mode = match cli.slice.to_lowercase().as_str() {
        "column" | "col" | "c" => SampleMode::Column,
        _ => SampleMode::Row,
    };

    let stack_mode = match cli.layout.to_lowercase().as_str() {
        "vertical" | "v" => StackMode::Vertical,
        _ => StackMode::Horizontal,
    };

    let (frame_count, strip_length) = match stack_mode {
        StackMode::Horizontal => (cli.width, cli.height),
        StackMode::Vertical => (cli.height, cli.width),
    };

    let config = ProcessorConfig {
        frame_count,
        strip_length,
        sample_mode,
        stack_mode,
    };

    let processor = Processor::new(config);

    println!("Processing: {}", cli.input.display());
    println!(
        "Output: {} ({}×{})",
        output_path.display(),
        cli.width,
        cli.height
    );
    println!("Layout: {}, Slice: {}", cli.layout, cli.slice);
    println!();

    processor.generate_spectrum(&cli.input, &output_path)?;

    println!();
    println!("Spectrum generated successfully!");

    Ok(())
}
