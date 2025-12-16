//! 图形用户界面模块
//!
//! 使用 GPUI + gpui-component 框架构建的桌面应用界面

pub mod app;
pub mod state;
mod views;

pub use app::run_gui;
