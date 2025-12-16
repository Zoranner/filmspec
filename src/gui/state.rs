//! 应用状态管理

use std::path::PathBuf;

use gpui::SharedString;

use crate::mode::{LayoutMode, ProcessMode};
use crate::SampleMode;

/// 处理进度信息
#[derive(Clone, Default)]
pub struct ProcessingProgress {
    /// 当前进度 (0.0 - 1.0)
    pub progress: f32,
    /// 当前阶段描述
    pub stage: SharedString,
    /// 已处理帧数
    pub current_frame: u32,
    /// 总帧数
    pub total_frames: u32,
}

/// 应用程序状态
pub struct AppState {
    /// 输入视频路径
    pub input_path: Option<PathBuf>,
    /// 输出目录（用户手动选择的）
    pub output_dir: Option<PathBuf>,
    /// 输出图片路径（完整路径）
    pub output_path: Option<PathBuf>,
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 布局模式
    pub layout_mode: LayoutMode,
    /// 采样模式
    pub sample_mode: SampleMode,
    /// 处理模式
    pub process_mode: ProcessMode,
    /// 内圆半径 (环形模式)
    pub inner_radius: u32,
    /// 处理状态
    pub processing: bool,
    /// 处理进度
    pub progress: ProcessingProgress,
    /// 状态消息
    pub status_message: Option<SharedString>,
    /// 是否成功
    pub is_success: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input_path: None,
            output_dir: None,
            output_path: None,
            width: 1920,
            height: 300,
            layout_mode: LayoutMode::Horizontal,
            sample_mode: SampleMode::Row,
            process_mode: ProcessMode::Slice,
            inner_radius: 250,
            processing: false,
            progress: ProcessingProgress::default(),
            status_message: None,
            is_success: false,
        }
    }
}

impl AppState {
    /// 切换布局模式
    pub fn cycle_layout_mode(&mut self) {
        self.layout_mode = match self.layout_mode {
            LayoutMode::Horizontal => LayoutMode::Vertical,
            LayoutMode::Vertical => LayoutMode::Radial,
            LayoutMode::Radial => LayoutMode::Horizontal,
        };

        // 根据布局模式调整默认尺寸
        match self.layout_mode {
            LayoutMode::Radial => {
                self.width = 1920;
                self.height = 500;
            }
            _ => {
                self.width = 1920;
                self.height = 300;
            }
        }

        self.update_output_path();
    }

    /// 切换处理模式
    pub fn cycle_process_mode(&mut self) {
        self.process_mode = match self.process_mode {
            ProcessMode::Slice => ProcessMode::Hue,
            ProcessMode::Hue => ProcessMode::Slice,
        };
        self.update_output_path();
    }

    /// 切换采样模式
    pub fn cycle_sample_mode(&mut self) {
        self.sample_mode = match self.sample_mode {
            SampleMode::Row => SampleMode::Column,
            SampleMode::Column => SampleMode::Row,
        };
    }

    /// 增加宽度
    pub fn increase_width(&mut self) {
        self.width = (self.width + 100).min(4096);
    }

    /// 减少宽度
    pub fn decrease_width(&mut self) {
        self.width = (self.width.saturating_sub(100)).max(100);
    }

    /// 增加高度
    pub fn increase_height(&mut self) {
        self.height = (self.height + 50).min(2000);
    }

    /// 减少高度
    pub fn decrease_height(&mut self) {
        self.height = (self.height.saturating_sub(50)).max(50);
    }

    /// 更新输出路径（根据输入路径、输出目录和处理模式）
    pub fn update_output_path(&mut self) {
        if let Some(ref input) = self.input_path {
            let stem = input
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("spectrum");
            let suffix = self.process_mode.output_suffix();
            let filename = format!("{}{}.png", stem, suffix);

            // 优先使用用户选择的输出目录，否则使用输入文件所在目录
            let output_dir = self
                .output_dir
                .clone()
                .or_else(|| input.parent().map(|p| p.to_path_buf()));

            if let Some(dir) = output_dir {
                self.output_path = Some(dir.join(filename));
            } else {
                self.output_path = Some(PathBuf::from(filename));
            }
        }
    }

    /// 设置输入路径
    pub fn set_input_path(&mut self, path: PathBuf) {
        self.input_path = Some(path);
        self.update_output_path();
        self.status_message = None;
    }

    /// 设置输出目录
    pub fn set_output_dir(&mut self, dir: PathBuf) {
        self.output_dir = Some(dir);
        self.update_output_path();
    }

    /// 设置输出路径（完整路径）
    pub fn set_output_path(&mut self, path: PathBuf) {
        self.output_path = Some(path);
    }

    /// 可以开始生成
    pub fn can_generate(&self) -> bool {
        self.input_path.is_some() && !self.processing
    }

    /// 获取布局模式标签
    pub fn layout_label(&self) -> &'static str {
        match self.layout_mode {
            LayoutMode::Horizontal => "水平布局",
            LayoutMode::Vertical => "垂直布局",
            LayoutMode::Radial => "环形布局",
        }
    }

    /// 获取采样模式标签
    pub fn sample_label(&self) -> &'static str {
        match self.sample_mode {
            SampleMode::Row => "行采样",
            SampleMode::Column => "列采样",
        }
    }

    /// 获取处理模式标签
    pub fn process_label(&self) -> &'static str {
        match self.process_mode {
            ProcessMode::Slice => "切片模式",
            ProcessMode::Hue => "色调模式",
        }
    }

    /// 是否为切片模式
    pub fn is_slice_mode(&self) -> bool {
        matches!(self.process_mode, ProcessMode::Slice)
    }

    /// 是否为环形布局
    pub fn is_radial_mode(&self) -> bool {
        matches!(self.layout_mode, LayoutMode::Radial)
    }
}
