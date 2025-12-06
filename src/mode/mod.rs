//! 光谱生成模式模块
//!
//! 定义了 `SpectrumMode` trait 和各种处理模式的实现。
//! 新增模式只需实现 `SpectrumMode` trait 并在此模块注册即可。

pub mod hue;
mod layout;
pub mod slice;

pub use hue::HueMode;
pub use slice::SliceMode;

use std::path::Path;
use std::str::FromStr;

use image::DynamicImage;

use crate::ffmpeg::VideoInfo;
use crate::Result;

/// 布局方向
#[derive(Debug, Clone, Copy, Default)]
pub enum LayoutMode {
    Vertical,
    #[default]
    Horizontal,
    /// 环形布局（从中心向外扩展，类似CD光碟）
    Radial,
}

/// 光谱生成模式 trait
///
/// 所有处理模式都需要实现这个 trait。
/// 这提供了一个统一的接口，便于扩展新模式。
pub trait SpectrumMode {
    /// 模式名称（用于显示）
    fn name(&self) -> &'static str;

    /// 模式标识符（用于 CLI 参数）
    fn id(&self) -> &'static str;

    /// 输出文件后缀
    fn output_suffix(&self) -> &'static str;

    /// 生成光谱图像
    fn generate(
        &self,
        video_path: &Path,
        video_info: &VideoInfo,
        frame_count: u32,
        band_length: u32,
        inner_radius: u32,
        layout_mode: LayoutMode,
    ) -> Result<DynamicImage>;
}

/// 处理模式枚举
///
/// 包含所有可用的处理模式，便于 CLI 解析和调度。
#[derive(Debug, Clone, Copy, Default)]
pub enum ProcessMode {
    #[default]
    Slice,
    Hue,
}

impl FromStr for ProcessMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "hue" | "dominant" | "color" => ProcessMode::Hue,
            _ => ProcessMode::Slice,
        })
    }
}

impl ProcessMode {
    /// 获取模式的名称
    pub fn name(&self) -> &'static str {
        match self {
            ProcessMode::Slice => "Slice",
            ProcessMode::Hue => "Hue Spectrum",
        }
    }

    /// 获取输出文件后缀
    pub fn output_suffix(&self) -> &'static str {
        match self {
            ProcessMode::Slice => "_spectrum",
            ProcessMode::Hue => "_hue",
        }
    }
}
