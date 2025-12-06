//! 色调光谱模式
//!
//! 分析每帧的主色调，生成色调变化光谱图像。

use std::path::Path;

use image::{DynamicImage, Rgb};

use super::layout::render_spectrum;
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
        inner_radius: u32,
        layout_mode: LayoutMode,
    ) -> Result<DynamicImage> {
        let (frames_data, actual_frame_count) = FFmpeg::extract_frames_to_memory(
            video_path,
            frame_count,
            SAMPLE_WIDTH,
            SAMPLE_HEIGHT,
            video_info.duration,
        )?;

        if actual_frame_count == 0 {
            return Err(Error::NoFramesExtracted);
        }

        println!("→ Analyzing colors...");

        // 计算每帧的主色调
        let dominant_colors: Vec<Rgb<u8>> = frames_data
            .iter()
            .map(|frame_data| calculate_dominant_hue(frame_data))
            .collect();

        println!("→ Building spectrum...");

        // 使用统一的布局渲染函数
        // 色调模式下，整个径向使用同一颜色
        let image = render_spectrum(
            dominant_colors.len(),
            band_length,
            inner_radius,
            layout_mode,
            |frame_idx, _band_idx| dominant_colors[frame_idx],
        );

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
