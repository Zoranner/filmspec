use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;

use crate::mode::slice::SampleMode;
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    format: FFprobeFormat,
}

#[derive(Debug, Deserialize)]
struct FFprobeFormat {
    duration: String,
}

pub struct VideoInfo {
    pub duration: f64,
    pub path: PathBuf,
}

pub struct FFmpeg;

impl FFmpeg {
    pub fn check_availability() -> Result<()> {
        let ffmpeg_status = Command::new("ffmpeg").arg("-version").output();
        if ffmpeg_status.is_err() {
            return Err(Error::FFmpegNotFound);
        }

        let ffprobe_status = Command::new("ffprobe").arg("-version").output();
        if ffprobe_status.is_err() {
            return Err(Error::FFprobeNotFound);
        }

        Ok(())
    }

    pub fn get_video_info(video_path: &Path) -> Result<VideoInfo> {
        if !video_path.exists() {
            return Err(Error::VideoNotFound(video_path.to_path_buf()));
        }

        let output = Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                video_path.to_str().unwrap_or_default(),
            ])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::FFmpegExecution(stderr.to_string()));
        }

        let probe_output: FFprobeOutput = serde_json::from_slice(&output.stdout)
            .map_err(|e| Error::MetadataParse(e.to_string()))?;

        let duration: f64 = probe_output
            .format
            .duration
            .parse()
            .map_err(|_| Error::InvalidDuration(probe_output.format.duration.clone()))?;

        Ok(VideoInfo {
            duration,
            path: video_path.to_path_buf(),
        })
    }

    pub fn extract_strips_to_memory(
        video_path: &Path,
        frame_count: u32,
        band_length: u32,
        duration: f64,
        sample_mode: SampleMode,
    ) -> Result<Vec<u8>> {
        let fps = frame_count as f64 / duration;

        let vf = match sample_mode {
            SampleMode::Row => format!(
                "fps={},scale={}:-1:flags=lanczos,crop={}:1:0:ih/2",
                fps, band_length, band_length
            ),
            SampleMode::Column => format!(
                "fps={},scale=-1:{}:flags=lanczos,crop=1:{}:iw/2:0",
                fps, band_length, band_length
            ),
        };

        let mut child = Command::new("ffmpeg")
            .args([
                "-threads",
                "0",
                "-i",
                video_path.to_str().unwrap_or_default(),
                "-vf",
                &vf,
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgb24",
                "-v",
                "quiet",
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let stderr_thread = thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = stderr;
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).to_string()
        });

        let bytes_per_frame = band_length as usize * 3;
        let expected_total_bytes = frame_count as usize * bytes_per_frame;

        let progress_bar = ProgressBar::new(frame_count as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} frames ({eta})")
                .expect("Invalid progress bar template")
                .progress_chars("█▓░"),
        );

        let mut raw_data = Vec::with_capacity(expected_total_bytes);
        let mut buffer = [0u8; 65536];

        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    raw_data.extend_from_slice(&buffer[..n]);
                    let current_frames = raw_data.len() / bytes_per_frame;
                    progress_bar.set_position(current_frames as u64);
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }

        progress_bar.finish_and_clear();

        let stderr_output = stderr_thread.join().expect("stderr thread panicked");
        let status = child.wait()?;

        if !status.success() && raw_data.is_empty() {
            return Err(Error::FFmpegExecution(stderr_output));
        }

        Ok(raw_data)
    }

    /// 提取完整帧用于色调分析
    /// 返回 (帧数据列表, 实际帧数)
    pub fn extract_frames_to_memory(
        video_path: &Path,
        frame_count: u32,
        width: u32,
        height: u32,
        duration: f64,
    ) -> Result<(Vec<Vec<u8>>, usize)> {
        let fps = frame_count as f64 / duration;

        let vf = format!("fps={},scale={}:{}:flags=fast_bilinear", fps, width, height);

        let mut child = Command::new("ffmpeg")
            .args([
                "-threads",
                "0",
                "-i",
                video_path.to_str().unwrap_or_default(),
                "-vf",
                &vf,
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgb24",
                "-v",
                "quiet",
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        let stderr_thread = thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = stderr;
            reader.read_to_end(&mut buffer).ok();
            String::from_utf8_lossy(&buffer).to_string()
        });

        let bytes_per_frame = (width * height * 3) as usize;
        let expected_total_bytes = frame_count as usize * bytes_per_frame;

        let progress_bar = ProgressBar::new(frame_count as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} frames ({eta})")
                .expect("Invalid progress bar template")
                .progress_chars("█▓░"),
        );

        let mut raw_data = Vec::with_capacity(expected_total_bytes);
        let mut buffer = [0u8; 65536];

        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    raw_data.extend_from_slice(&buffer[..n]);
                    let current_frames = raw_data.len() / bytes_per_frame;
                    progress_bar.set_position(current_frames as u64);
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }

        progress_bar.finish_and_clear();

        let stderr_output = stderr_thread.join().expect("stderr thread panicked");
        let status = child.wait()?;

        if !status.success() && raw_data.is_empty() {
            return Err(Error::FFmpegExecution(stderr_output));
        }

        // 将原始数据分割成单独的帧
        let actual_frame_count = raw_data.len() / bytes_per_frame;
        let frames: Vec<Vec<u8>> = raw_data
            .chunks(bytes_per_frame)
            .map(|chunk| chunk.to_vec())
            .collect();

        Ok((frames, actual_frame_count))
    }
}
