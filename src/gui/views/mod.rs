//! GUI 视图组件模块

mod action_section;
mod config_section;
mod file_section;

pub use action_section::render_action_section;
pub use config_section::render_config_section;
pub use file_section::render_file_section;
