//! GUI 视图组件模块

mod action_section;
mod config_section;
mod file_section;

pub use action_section::{ActionSection, GenerateEvent};
pub use config_section::ConfigSection;
pub use file_section::FileSection;
