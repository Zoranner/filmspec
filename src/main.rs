//! Filmspec CLI
//!
//! 从视频文件生成光谱图像的命令行工具。

use std::path::PathBuf;

use clap::Parser;

use filmspec::{LayoutMode, ProcessMode, Processor, ProcessorConfig, Result, SampleMode};

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

    /// Sample direction: row (middle horizontal line) or col (middle vertical line)
    /// Only used in slice mode
    #[arg(short, long, default_value = "row")]
    sample: String,

    /// Process mode: slice (pixel lines) or hue (dominant color spectrum)
    #[arg(short, long, default_value = "slice")]
    mode: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let process_mode = ProcessMode::from_str(&cli.mode);

    let output_path = cli.output.unwrap_or_else(|| {
        let stem = cli
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("spectrum");
        PathBuf::from(format!("{}{}.png", stem, process_mode.output_suffix()))
    });

    let sample_mode = SampleMode::from_str(&cli.sample);

    let layout_mode = match cli.layout.to_lowercase().as_str() {
        "vertical" | "v" => LayoutMode::Vertical,
        _ => LayoutMode::Horizontal,
    };

    let (frame_count, band_length) = match layout_mode {
        LayoutMode::Horizontal => (cli.width, cli.height),
        LayoutMode::Vertical => (cli.height, cli.width),
    };

    let config = ProcessorConfig {
        frame_count,
        band_length,
        sample_mode,
        layout_mode,
        process_mode,
    };

    let processor = Processor::new(config);

    println!("Processing: {}", cli.input.display());
    println!(
        "Output: {} ({}×{})",
        output_path.display(),
        cli.width,
        cli.height
    );
    println!("Mode: {}, Layout: {}", process_mode.name(), cli.layout);
    if matches!(process_mode, ProcessMode::Slice) {
        println!("Sample: {}", cli.sample);
    }
    println!();

    processor.generate_spectrum(&cli.input, &output_path)?;

    println!();
    println!("Spectrum generated successfully!");

    Ok(())
}
