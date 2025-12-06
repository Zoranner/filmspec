//! 色调光谱模式
//!
//! 分析每帧的主色调，生成色调变化光谱图像。

use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};

use super::{LayoutMode, SpectrumMode};
use crate::color::HueHistogram;
use crate::ffmpeg::VideoInfo;
use crate::{Error, FFmpeg, Result};

/// 色调分析采样尺寸
const SAMPLE_WIDTH: u32 = 160;
const SAMPLE_HEIGHT: u32 = 90;

/// 色相直方图区间数量（360度 / 36 = 每个区间10度）
const HUE_BINS: usize = 36;

/// 色调光谱模式
pub struct HueMode;

impl HueMode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HueMode {
    fn default() -> Self {
        Self::new()
    }
}

impl SpectrumMode for HueMode {
    fn name(&self) -> &'static str {
        "Hue Spectrum"
    }

    fn id(&self) -> &'static str {
        "hue"
    }

    fn output_suffix(&self) -> &'static str {
        "_hue"
    }

    fn generate(
        &self,
        video_path: &Path,
        video_info: &VideoInfo,
        frame_count: u32,
        band_length: u32,
        layout_mode: LayoutMode,
    ) -> Result<RgbImage> {
        println!("Extracting {} frames for hue analysis...", frame_count);

        let (frames_data, actual_frame_count) = FFmpeg::extract_frames_to_memory(
            video_path,
            frame_count,
            SAMPLE_WIDTH,
            SAMPLE_HEIGHT,
            video_info.duration,
        )?;

        println!(
            "Extracted {} frames, analyzing dominant colors...",
            actual_frame_count
        );

        if actual_frame_count == 0 {
            return Err(Error::NoFramesExtracted);
        }

        // 计算每帧的主色调
        let dominant_colors: Vec<Rgb<u8>> = frames_data
            .iter()
            .map(|frame_data| calculate_dominant_hue(frame_data))
            .collect();

        let image = build_hue_spectrum(&dominant_colors, band_length, layout_mode);
        Ok(image)
    }
}

/// 计算帧的主色调
fn calculate_dominant_hue(image_data: &[u8]) -> Rgb<u8> {
    let mut histogram = HueHistogram::new(HUE_BINS);

    for pixel in image_data.chunks(3) {
        if pixel.len() >= 3 {
            histogram.add_pixel(pixel[0], pixel[1], pixel[2]);
        }
    }

    histogram.get_dominant_color()
}

/// 构建色调光谱图像
fn build_hue_spectrum(
    dominant_colors: &[Rgb<u8>],
    band_length: u32,
    layout_mode: LayoutMode,
) -> RgbImage {
    let frame_count = dominant_colors.len();

    match layout_mode {
        LayoutMode::Horizontal => {
            let mut image: RgbImage = ImageBuffer::new(frame_count as u32, band_length);
            for (x, color) in dominant_colors.iter().enumerate() {
                for y in 0..band_length {
                    image.put_pixel(x as u32, y, *color);
                }
            }
            image
        }
        LayoutMode::Vertical => {
            let mut image: RgbImage = ImageBuffer::new(band_length, frame_count as u32);
            for (y, color) in dominant_colors.iter().enumerate() {
                for x in 0..band_length {
                    image.put_pixel(x, y as u32, *color);
                }
            }
            image
        }
    }
}

