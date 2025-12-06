use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FFmpeg not found in PATH. Please install FFmpeg and ensure it's accessible.")]
    FFmpegNotFound,

    #[error("FFprobe not found in PATH. Please install FFmpeg and ensure ffprobe is accessible.")]
    FFprobeNotFound,

    #[error("Failed to execute FFmpeg: {0}")]
    FFmpegExecution(String),

    #[error("Failed to parse video metadata: {0}")]
    MetadataParse(String),

    #[error("Video file not found: {}", .0.display())]
    VideoNotFound(PathBuf),

    #[error("Invalid video duration: {0}")]
    InvalidDuration(String),

    #[error("Failed to load image: {0}")]
    ImageLoad(#[from] image::ImageError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No frames extracted from video")]
    NoFramesExtracted,
}
