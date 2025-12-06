//! Filmspec - 电影光谱生成库
//!
//! 从视频文件生成视觉光谱图像的工具库。
//!
//! # 功能模式
//!
//! - **Slice（像素条模式）**：从每帧提取一条像素线堆叠成光谱
//! - **Hue（色调模式）**：分析每帧主色调，生成色调变化光谱
//!
//! # 示例
//!
//! ```ignore
//! use filmspec::{Processor, ProcessorConfig, ProcessMode, LayoutMode, SampleMode};
//!
//! let config = ProcessorConfig {
//!     frame_count: 1920,
//!     band_length: 300,
//!     sample_mode: SampleMode::Row,
//!     layout_mode: LayoutMode::Horizontal,
//!     process_mode: ProcessMode::Slice,
//! };
//!
//! let processor = Processor::new(config);
//! processor.generate_spectrum("video.mp4", "output.png")?;
//! ```

pub mod color;
mod error;
mod ffmpeg;
pub mod mode;
mod processor;

pub use error::Error;
pub use ffmpeg::FFmpeg;
pub use mode::slice::SampleMode;
pub use mode::{LayoutMode, ProcessMode};
pub use processor::{Processor, ProcessorConfig};

/// 标准 Result 类型别名
pub type Result<T> = std::result::Result<T, Error>;
