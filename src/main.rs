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

    /// Output image width in pixels (default: 1920 for h/v, 800 for radial)
    #[arg(short, long)]
    width: Option<u32>,

    /// Output image height in pixels (default: 300 for h/v, 200 for radial)
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Layout direction: h (horizontal), v (vertical), or r (radial/disc)
    #[arg(short, long, default_value = "h")]
    layout: String,

    /// Inner radius for radial layout (pixels)
    #[arg(long, default_value = "250")]
    inner_radius: u32,

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

    let process_mode: ProcessMode = cli.mode.parse().unwrap();

    let output_path = cli.output.unwrap_or_else(|| {
        let stem = cli
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("spectrum");
        PathBuf::from(format!("{}{}.png", stem, process_mode.output_suffix()))
    });

    let sample_mode: SampleMode = cli.sample.parse().unwrap();

    let layout_mode = match cli.layout.to_lowercase().as_str() {
        "vertical" | "v" => LayoutMode::Vertical,
        "radial" | "r" | "disc" | "ring" => LayoutMode::Radial,
        _ => LayoutMode::Horizontal,
    };

    // 根据布局模式设置默认值
    let (default_width, default_height) = match layout_mode {
        LayoutMode::Radial => (1920, 800), // 环形模式：帧数1920，色带长度800
        _ => (1920, 300),                  // 水平/垂直模式默认值
    };

    let width = cli.width.unwrap_or(default_width);
    let height = cli.height.unwrap_or(default_height);

    let (frame_count, band_length, inner_radius) = match layout_mode {
        LayoutMode::Horizontal => (width, height, 0),
        LayoutMode::Vertical => (height, width, 0),
        LayoutMode::Radial => {
            // 环形布局：
            // - frame_count = 帧数（沿圆周方向）
            // - band_length = 圆环宽度（径向方向，即每个像素条的长度）
            // - 输出图像尺寸 = (inner_radius + band_length) * 2
            (width, height, cli.inner_radius)
        }
    };

    let config = ProcessorConfig {
        frame_count,
        band_length,
        inner_radius,
        sample_mode,
        layout_mode,
        process_mode,
    };

    let processor = Processor::new(config);

    // 打印配置信息
    println!("Input:  {}", cli.input.display());
    println!("Output: {}", output_path.display());

    match layout_mode {
        LayoutMode::Radial => {
            let output_size = (inner_radius + band_length) * 2;
            println!(
                "Config: {} | Radial {}×{} | {} frames | ring {} (inner {})",
                process_mode.name(),
                output_size,
                output_size,
                frame_count,
                band_length,
                inner_radius
            );
        }
        LayoutMode::Horizontal => {
            print!("Config: {} | Horizontal {}×{}", process_mode.name(), width, height);
            if matches!(process_mode, ProcessMode::Slice) {
                print!(" | sample: {}", cli.sample);
            }
            println!();
        }
        LayoutMode::Vertical => {
            print!("Config: {} | Vertical {}×{}", process_mode.name(), height, width);
            if matches!(process_mode, ProcessMode::Slice) {
                print!(" | sample: {}", cli.sample);
            }
            println!();
        }
    }
    println!();

    processor.generate_spectrum(&cli.input, &output_path)?;

    println!("Done!");

    Ok(())
}
