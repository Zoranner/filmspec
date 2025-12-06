mod error;
mod ffmpeg;
mod processor;

pub use error::Error;
pub use ffmpeg::FFmpeg;
pub use processor::{Processor, ProcessorConfig, SampleMode, StackMode};

pub type Result<T> = std::result::Result<T, Error>;
