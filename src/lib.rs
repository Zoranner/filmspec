//! Filmspec - 电影光谱生成库
//!
//! 从视频文件生成视觉光谱图像的工具库。
//!
//! # 功能模式
//!
//! - **Slice（像素条模式）**：从每帧提取一条像素线堆叠成光谱
//! - **Hue（色调模式）**：分析每帧主色调，生成色调变化光谱
//!
//! # 布局模式
//!
//! - **Horizontal（水平布局）**：从左到右的时间轴
//! - **Vertical（垂直布局）**：从上到下的时间轴
//! - **Radial（环形布局）**：类似CD光碟的圆形布局，从12点钟方向顺时针
//!
//! # 示例
//!
//! ```ignore
//! use filmspec::{Processor, ProcessorConfig, ProcessMode, LayoutMode, SampleMode};
//!
//! let config = ProcessorConfig {
//!     frame_count: 1920,
//!     band_length: 300,
//!     inner_radius: 50,
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
pub mod processor;

#[cfg(feature = "gui")]
pub mod gui;

pub use error::Error;
pub use ffmpeg::{FFmpeg, ProgressCallback};
pub use mode::slice::SampleMode;
pub use mode::{LayoutMode, ProcessMode};
pub use processor::{Processor, ProcessorConfig};

/// 标准 Result 类型别名
pub type Result<T> = std::result::Result<T, Error>;
