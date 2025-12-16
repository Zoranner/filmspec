//! 视频光谱处理器
//!
//! 协调视频分析和光谱生成的主要处理器。

use std::path::Path;

use crate::ffmpeg::ProgressCallback;
use crate::mode::hue::HueMode;
use crate::mode::slice::{SampleMode, SliceMode};
use crate::mode::{LayoutMode, ProcessMode, SpectrumMode};
use crate::{FFmpeg, Result};

/// 处理器配置
pub struct ProcessorConfig {
    /// 帧数（取决于输出尺寸和布局方向）
    pub frame_count: u32,
    /// 色带长度（像素数，取决于输出尺寸和布局方向）
    pub band_length: u32,
    /// 内圆半径（仅环形布局使用）
    pub inner_radius: u32,
    /// 采样模式（行/列，仅 Slice 模式使用）
    pub sample_mode: SampleMode,
    /// 布局方向（水平/垂直/环形）
    pub layout_mode: LayoutMode,
    /// 处理模式（像素条/色调）
    pub process_mode: ProcessMode,
}

/// 视频光谱处理器
pub struct Processor {
    config: ProcessorConfig,
}

impl Processor {
    /// 创建新的处理器实例
    pub fn new(config: ProcessorConfig) -> Self {
        Self { config }
    }

    /// 生成光谱图像（CLI 版本，使用控制台进度条）
    pub fn generate_spectrum(&self, video_path: &Path, output_path: &Path) -> Result<()> {
        self.generate_spectrum_with_progress(video_path, output_path, None)
    }

    /// 生成光谱图像（支持进度回调）
    pub fn generate_spectrum_with_progress(
        &self,
        video_path: &Path,
        output_path: &Path,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<()> {
        FFmpeg::check_availability()?;

        let video_info = FFmpeg::get_video_info(video_path)?;

        // 根据处理模式选择对应的实现
        let image = match self.config.process_mode {
            ProcessMode::Slice => {
                let mode = SliceMode::new(self.config.sample_mode);
                mode.generate_with_progress(
                    video_path,
                    &video_info,
                    self.config.frame_count,
                    self.config.band_length,
                    self.config.inner_radius,
                    self.config.layout_mode,
                    progress_callback.clone(),
                )?
            }
            ProcessMode::Hue => {
                let mode = HueMode::new();
                mode.generate_with_progress(
                    video_path,
                    &video_info,
                    self.config.frame_count,
                    self.config.band_length,
                    self.config.inner_radius,
                    self.config.layout_mode,
                    progress_callback.clone(),
                )?
            }
        };

        // 通知进入保存阶段
        if let Some(ref callback) = progress_callback {
            callback("保存图像", 0, 1, 0.95);
        } else {
            println!("→ Saving image...");
        }

        image.save(output_path)?;
        Ok(())
    }
}
