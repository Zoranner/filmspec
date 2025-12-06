//! 像素条光谱模式
//!
//! 从每帧提取一条像素线（行或列），堆叠成光谱图像。

use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};

use super::{LayoutMode, SpectrumMode};
use crate::ffmpeg::VideoInfo;
use crate::{Error, FFmpeg, Result};

/// 采样方向
#[derive(Debug, Clone, Copy, Default)]
pub enum SampleMode {
    #[default]
    Row,    // 横向采样（提取中间行）
    Column, // 纵向采样（提取中间列）
}

impl SampleMode {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "column" | "col" | "c" => SampleMode::Column,
            _ => SampleMode::Row,
        }
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

    fn generate(
        &self,
        video_path: &Path,
        video_info: &VideoInfo,
        frame_count: u32,
        band_length: u32,
        layout_mode: LayoutMode,
    ) -> Result<RgbImage> {
        println!("Extracting {} frames...", frame_count);

        let raw_data = FFmpeg::extract_strips_to_memory(
            video_path,
            frame_count,
            band_length,
            video_info.duration,
            self.sample_mode,
        )?;

        let bytes_per_strip = band_length as usize * 3;
        let actual_frame_count = raw_data.len() / bytes_per_strip;

        println!(
            "Extracted {} strips ({} bytes)",
            actual_frame_count,
            raw_data.len()
        );

        if actual_frame_count == 0 {
            return Err(Error::NoFramesExtracted);
        }

        let image = build_spectrum(&raw_data, actual_frame_count, band_length, layout_mode);
        Ok(image)
    }
}

/// 构建像素条光谱图像
fn build_spectrum(
    raw_data: &[u8],
    frame_count: usize,
    band_length: u32,
    layout_mode: LayoutMode,
) -> RgbImage {
    let bytes_per_strip = band_length as usize * 3;

    match layout_mode {
        LayoutMode::Vertical => {
            let mut image: RgbImage = ImageBuffer::new(band_length, frame_count as u32);
            for (y, chunk) in raw_data.chunks(bytes_per_strip).enumerate() {
                for (x, pixel) in chunk.chunks(3).enumerate() {
                    if pixel.len() == 3 && (x as u32) < band_length {
                        image.put_pixel(x as u32, y as u32, Rgb([pixel[0], pixel[1], pixel[2]]));
                    }
                }
            }
            image
        }
        LayoutMode::Horizontal => {
            let mut image: RgbImage = ImageBuffer::new(frame_count as u32, band_length);
            for (x, chunk) in raw_data.chunks(bytes_per_strip).enumerate() {
                for (y, pixel) in chunk.chunks(3).enumerate() {
                    if pixel.len() == 3 && (y as u32) < band_length {
                        image.put_pixel(x as u32, y as u32, Rgb([pixel[0], pixel[1], pixel[2]]));
                    }
                }
            }
            image
        }
    }
}

