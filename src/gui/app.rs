//! 主应用程序

use std::sync::Arc;

use gpui::*;
use gpui_component::{
    notification::Notification, scroll::ScrollableElement, v_flex, ActiveTheme, Root, WindowExt,
};

use super::components::{ActionSection, ConfigSection, FileSection, GenerateEvent};
use crate::mode::LayoutMode;
use crate::processor::{Processor, ProcessorConfig};

/// 窗口尺寸（固定）
pub const WINDOW_WIDTH: f32 = 480.0;
pub const WINDOW_HEIGHT: f32 = 540.0;

/// Filmspec 应用
pub struct FilmspecApp {
    /// 文件选择区域
    file_section: Entity<FileSection>,
    /// 配置区域
    config_section: Entity<ConfigSection>,
    /// 操作区域
    action_section: Entity<ActionSection>,
}

impl FilmspecApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let file_section = cx.new(|cx| FileSection::new(window, cx));
        let config_section = cx.new(|cx| ConfigSection::new(window, cx));
        let action_section = cx.new(|cx| ActionSection::new(window, cx));

        // 订阅生成事件
        cx.subscribe_in(
            &action_section,
            window,
            |this, _, _event: &GenerateEvent, window, cx| {
                this.generate_spectrum(window, cx);
            },
        )
        .detach();

        Self {
            file_section,
            config_section,
            action_section,
        }
    }

    /// 更新操作区域的可用状态和处理中的禁用状态
    fn update_ui_state(&self, window: &mut Window, cx: &mut Context<Self>) {
        let can_generate = self.file_section.read(cx).input_path().is_some();
        let is_processing = self.action_section.read(cx).is_processing();

        // 更新操作区域
        self.action_section.update(cx, |action, cx| {
            action.set_can_generate(can_generate);
            cx.notify();
        });

        // 更新文件选择区域的禁用状态
        self.file_section.update(cx, |file, cx| {
            file.set_disabled(is_processing);
            cx.notify();
        });

        // 更新配置区域的禁用状态
        self.config_section.update(cx, |config, cx| {
            config.set_disabled(is_processing);
            cx.notify();
        });

        // 更新输出路径（根据处理模式）
        let suffix = self.config_section.read(cx).get_output_suffix(cx);
        self.file_section.update(cx, |file, cx| {
            file.update_output_path(suffix, window, cx);
        });
    }

    pub fn generate_spectrum(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // 获取配置
        let config_data = self.config_section.read(cx).get_config(cx);

        // 更新文件路径（根据处理模式）
        let suffix = self.config_section.read(cx).get_output_suffix(cx);
        self.file_section.update(cx, |file, cx| {
            file.update_output_path(suffix, window, cx);
        });

        let input_path = match self.file_section.read(cx).input_path() {
            Some(p) => p.clone(),
            None => {
                window.push_notification(Notification::warning("请选择输入视频"), cx);
                return;
            }
        };

        let output_path = match self.file_section.read(cx).output_path() {
            Some(p) => p.clone(),
            None => {
                window.push_notification(Notification::warning("请选择输出路径"), cx);
                return;
            }
        };

        // 开始处理
        self.action_section.update(cx, |action, cx| {
            action.start_processing(cx);
        });
        cx.notify();

        let (frame_count, band_length, inner_radius) = match config_data.layout_mode {
            LayoutMode::Horizontal => (config_data.width, config_data.height, 0),
            LayoutMode::Vertical => (config_data.height, config_data.width, 0),
            LayoutMode::Radial => (
                config_data.width,
                config_data.height,
                config_data.inner_radius,
            ),
        };

        let processor_config = ProcessorConfig {
            frame_count,
            band_length,
            inner_radius,
            sample_mode: config_data.sample_mode,
            layout_mode: config_data.layout_mode,
            process_mode: config_data.process_mode,
        };

        // 创建进度更新通道
        let (progress_tx, progress_rx) = std::sync::mpsc::channel::<(String, u32, u32, f32)>();

        // 创建进度回调
        let progress_callback: crate::ProgressCallback =
            Arc::new(move |stage, current, total, progress| {
                let _ = progress_tx.send((stage.to_string(), current, total, progress));
            });

        // 启动处理线程
        let handle = std::thread::spawn(move || {
            let processor = Processor::new(processor_config);
            processor.generate_spectrum_with_progress(
                &input_path,
                &output_path,
                Some(progress_callback),
            )
        });

        // 启动进度更新任务
        let action_section = self.action_section.clone();
        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            // 轮询进度更新
            loop {
                match progress_rx.try_recv() {
                    Ok((stage, current, total, progress)) => {
                        let _ = action_section.update_in(cx, |action, _window, cx| {
                            action.update_progress(&stage, current, total, progress, cx);
                        });
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        if handle.is_finished() {
                            break;
                        }
                        gpui::Timer::after(std::time::Duration::from_millis(50)).await;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                }
            }

            // 获取最终结果
            let result = handle.join();

            let _ = action_section.update_in(cx, |action, _window, cx| {
                action.finish_processing(cx);
            });

            let _ = this.update_in(cx, |_this, window, cx| {
                match result {
                    Ok(Ok(())) => {
                        window.push_notification(Notification::success("光谱图像生成完成！"), cx);
                    }
                    Ok(Err(e)) => {
                        window
                            .push_notification(Notification::error(format!("生成失败: {}", e)), cx);
                    }
                    Err(_) => {
                        window.push_notification(Notification::error("处理过程发生错误"), cx);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }
}

impl Render for FilmspecApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 更新 UI 状态（操作按钮可用性、处理中禁用状态、输出路径）
        self.update_ui_state(window, cx);

        let theme = cx.theme();
        let bg_color = theme.background;
        let border_color = theme.border;

        v_flex()
            .size_full()
            .bg(bg_color)
            // 中间可滚动区域
            .child(
                div()
                    .id("scroll-container")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .child(
                        v_flex()
                            .p_4()
                            .gap_4()
                            .child(self.file_section.clone())
                            .child(self.config_section.clone()),
                    ),
            )
            // 操作区（固定底部）
            .child(
                div()
                    .flex_shrink_0()
                    .p_4()
                    .border_t_1()
                    .border_color(border_color)
                    .child(self.action_section.clone()),
            )
            // Overlay layers
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}

/// 运行 GUI 应用
pub fn run_gui() {
    use gpui_component_assets::Assets;

    Application::new().with_assets(Assets).run(|cx: &mut App| {
        // 初始化 gpui-component
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            is_resizable: false,
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            window.set_window_title("Filmspec - 视频光谱生成工具");
            let app = cx.new(|cx| FilmspecApp::new(window, cx));
            // Root must be the first level child of window
            cx.new(|cx| Root::new(app, window, cx))
        })
        .unwrap();
    });
}
