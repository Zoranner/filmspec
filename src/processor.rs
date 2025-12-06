use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};

use crate::{Error, FFmpeg, Result};

#[derive(Debug, Clone, Copy, Default)]
pub enum SampleMode {
    #[default]
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum StackMode {
    Vertical,
    #[default]
    Horizontal,
}

pub struct ProcessorConfig {
    pub frame_count: u32,
    pub strip_length: u32,
    pub sample_mode: SampleMode,
    pub stack_mode: StackMode,
}

pub struct Processor {
    config: ProcessorConfig,
}

impl Processor {
    pub fn new(config: ProcessorConfig) -> Self {
        Self { config }
    }

    pub fn generate_spectrum(&self, video_path: &Path, output_path: &Path) -> Result<()> {
        FFmpeg::check_availability()?;

        let video_info = FFmpeg::get_video_info(video_path)?;

        println!("Extracting {} frames...", self.config.frame_count);
        let raw_data = FFmpeg::extract_strips_to_memory(
            video_path,
            self.config.frame_count,
            self.config.strip_length,
            video_info.duration,
            self.config.sample_mode,
        )?;

        let bytes_per_strip = self.config.strip_length as usize * 3;
        let actual_frame_count = raw_data.len() / bytes_per_strip;
        println!(
            "Extracted {} strips ({} bytes)",
            actual_frame_count,
            raw_data.len()
        );

        if actual_frame_count == 0 {
            return Err(Error::NoFramesExtracted);
        }

        let spectrum = self.build_spectrum(&raw_data, actual_frame_count);
        spectrum.save(output_path)?;

        Ok(())
    }

    fn build_spectrum(&self, raw_data: &[u8], frame_count: usize) -> RgbImage {
        let strip_length = self.config.strip_length;
        let bytes_per_strip = strip_length as usize * 3;

        match self.config.stack_mode {
            StackMode::Vertical => {
                let mut image: RgbImage = ImageBuffer::new(strip_length, frame_count as u32);
                for (y, chunk) in raw_data.chunks(bytes_per_strip).enumerate() {
                    for (x, pixel) in chunk.chunks(3).enumerate() {
                        if pixel.len() == 3 && (x as u32) < strip_length {
                            image.put_pixel(
                                x as u32,
                                y as u32,
                                Rgb([pixel[0], pixel[1], pixel[2]]),
                            );
                        }
                    }
                }
                image
            }
            StackMode::Horizontal => {
                let mut image: RgbImage = ImageBuffer::new(frame_count as u32, strip_length);
                for (x, chunk) in raw_data.chunks(bytes_per_strip).enumerate() {
                    for (y, pixel) in chunk.chunks(3).enumerate() {
                        if pixel.len() == 3 && (y as u32) < strip_length {
                            image.put_pixel(
                                x as u32,
                                y as u32,
                                Rgb([pixel[0], pixel[1], pixel[2]]),
                            );
                        }
                    }
                }
                image
            }
        }
    }
}
