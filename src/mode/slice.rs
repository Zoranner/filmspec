//! 像素条光谱模式
//!
//! 从每帧提取一条像素线（行或列），堆叠成光谱图像。

use std::path::Path;
use std::str::FromStr;

use image::{DynamicImage, Rgb};

use super::layout::render_spectrum;
use super::{LayoutMode, SpectrumMode};
use crate::ffmpeg::{ProgressCallback, VideoInfo};
use crate::{Error, FFmpeg, Result};

/// 采样方向
#[derive(Debug, Clone, Copy, Default)]
pub enum SampleMode {
    #[default]
    Row, // 横向采样（提取中间行）
    Column, // 纵向采样（提取中间列）
}

impl FromStr for SampleMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "column" | "col" | "c" => SampleMode::Column,
            _ => SampleMode::Row,
        })
    }
}

/// 像素条光谱模式
pub struct SliceMode {
    sample_mode: SampleMode,
}

impl SliceMode {
    pub fn new(sample_mode: SampleMode) -> Self {
        Self { sample_mode }
    }
}

impl SpectrumMode for SliceMode {
    fn name(&self) -> &'static str {
        "Slice"
    }

    fn id(&self) -> &'static str {
        "slice"
    }

    fn output_suffix(&self) -> &'static str {
        "_spectrum"
    }

    fn generate_with_progress(
        &self,
        video_path: &Path,
        video_info: &VideoInfo,
        frame_count: u32,
        band_length: u32,
        inner_radius: u32,
        layout_mode: LayoutMode,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<DynamicImage> {
        let raw_data = FFmpeg::extract_strips_to_memory_with_progress(
            video_path,
            frame_count,
            band_length,
            video_info.duration,
            self.sample_mode,
            progress_callback.clone(),
        )?;

        let bytes_per_strip = band_length as usize * 3;
        let actual_frame_count = raw_data.len() / bytes_per_strip;

        if actual_frame_count == 0 {
            return Err(Error::NoFramesExtracted);
        }

        // 通知进入渲染阶段
        if let Some(ref callback) = progress_callback {
            callback("生成光谱图像", 0, 1, 0.9);
        } else {
            println!("→ Building spectrum...");
        }

        // 使用统一的布局渲染函数
        let image = render_spectrum(
            actual_frame_count,
            band_length,
            inner_radius,
            layout_mode,
            |frame_idx, band_idx| {
                let strip_offset = frame_idx * bytes_per_strip;
                let pixel_offset = strip_offset + band_idx * 3;

                if pixel_offset + 2 < raw_data.len() {
                    Rgb([
                        raw_data[pixel_offset],
                        raw_data[pixel_offset + 1],
                        raw_data[pixel_offset + 2],
                    ])
                } else {
                    Rgb([0, 0, 0])
                }
            },
        );

        Ok(image)
    }
}
