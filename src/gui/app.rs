//! 主应用程序

use std::sync::Arc;

use gpui::*;
use gpui_component::{
    input::{InputState, NumberInputEvent, StepAction},
    notification::Notification,
    scroll::ScrollableElement,
    select::SelectState,
    slider::{SliderEvent, SliderState},
    v_flex, ActiveTheme, IndexPath, Root, WindowExt,
};

use super::state::AppState;
use super::views::{render_action_section, render_config_section, render_file_section};
use crate::mode::{LayoutMode, ProcessMode};
use crate::processor::{Processor, ProcessorConfig};
use crate::SampleMode;

/// 窗口尺寸（固定）
pub const WINDOW_WIDTH: f32 = 480.0;
pub const WINDOW_HEIGHT: f32 = 640.0;

/// Filmspec 应用
pub struct FilmspecApp {
    pub(crate) state: AppState,
    // 文件路径 Input 组件状态
    pub(crate) input_file_state: Entity<InputState>,
    pub(crate) output_file_state: Entity<InputState>,
    // 尺寸 Input 组件状态
    pub(crate) width_state: Entity<InputState>,
    pub(crate) height_state: Entity<InputState>,
    // Slider 组件状态
    pub(crate) radius_slider: Entity<SliderState>,
    // Select 组件状态
    pub(crate) layout_select: Entity<SelectState<Vec<&'static str>>>,
    pub(crate) process_select: Entity<SelectState<Vec<&'static str>>>,
    pub(crate) sample_select: Entity<SelectState<Vec<&'static str>>>,
}

impl FilmspecApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 文件路径状态
        let input_file_state =
            cx.new(|cx| InputState::new(window, cx).default_value("请选择视频文件..."));
        let output_file_state =
            cx.new(|cx| InputState::new(window, cx).default_value("请选择输出路径..."));

        // 尺寸状态 - 使用正则表达式限制只能输入数字
        let number_pattern = regex::Regex::new(r"^\d*$").unwrap();
        let width_state = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("1920")
                .pattern(number_pattern.clone())
        });
        let height_state = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("200")
                .pattern(number_pattern)
        });

        // 监听宽度 NumberInput 的加减事件
        cx.subscribe_in(&width_state, window, {
            move |_this, state, event: &NumberInputEvent, window, cx| {
                let NumberInputEvent::Step(action) = event;
                state.update(cx, |state, cx| {
                    let current: i32 = state.text().to_string().parse().unwrap_or(1920);
                    let step = 100;
                    let new_value = match action {
                        StepAction::Increment => (current + step).min(7680),
                        StepAction::Decrement => (current - step).max(100),
                    };
                    state.set_value(&new_value.to_string(), window, cx);
                });
            }
        })
        .detach();

        // 监听高度 NumberInput 的加减事件
        cx.subscribe_in(&height_state, window, {
            move |_this, state, event: &NumberInputEvent, window, cx| {
                let NumberInputEvent::Step(action) = event;
                state.update(cx, |state, cx| {
                    let current: i32 = state.text().to_string().parse().unwrap_or(200);
                    let step = 50;
                    let new_value = match action {
                        StepAction::Increment => (current + step).min(4320),
                        StepAction::Decrement => (current - step).max(50),
                    };
                    state.set_value(&new_value.to_string(), window, cx);
                });
            }
        })
        .detach();

        // 内圆半径 Slider 状态
        let radius_slider = cx.new(|_cx| {
            SliderState::new()
                .min(50.0)
                .max(500.0)
                .step(10.0)
                .default_value(250.0)
        });

        // 监听 Slider 变化事件
        cx.subscribe(&radius_slider, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.state.inner_radius = value.end() as u32;
            cx.notify();
        })
        .detach();

        // 布局选择状态
        let layout_items = vec!["水平", "垂直", "环形"];
        let layout_select =
            cx.new(|cx| SelectState::new(layout_items, Some(IndexPath::new(0)), window, cx));

        // 模式选择状态
        let process_items = vec!["切片", "色调"];
        let process_select =
            cx.new(|cx| SelectState::new(process_items, Some(IndexPath::new(0)), window, cx));

        // 采样选择状态
        let sample_items = vec!["行采样", "列采样"];
        let sample_select =
            cx.new(|cx| SelectState::new(sample_items, Some(IndexPath::new(0)), window, cx));

        Self {
            state: AppState::default(),
            input_file_state,
            output_file_state,
            width_state,
            height_state,
            radius_slider,
            layout_select,
            process_select,
            sample_select,
        }
    }

    pub fn select_input_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let path_receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("选择视频文件".into()),
        });

        let input_state = self.input_file_state.clone();
        let output_state = self.output_file_state.clone();
        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            if let Ok(Ok(Some(paths))) = path_receiver.await {
                if let Some(path) = paths.into_iter().next() {
                    let display = path.to_string_lossy().to_string();
                    // 更新状态并获取生成的输出路径
                    let output_display = this
                        .update_in(cx, |this, _window, _cx| {
                            this.state.set_input_path(path);
                            // 返回生成的输出路径用于更新 UI
                            this.state
                                .output_path
                                .as_ref()
                                .map(|p| p.to_string_lossy().to_string())
                        })
                        .ok()
                        .flatten();
                    // 更新输入路径 InputState 的值
                    let _ = input_state.update_in(cx, |state, window, cx| {
                        state.set_value(&display, window, cx);
                    });
                    // 同步更新输出路径 InputState 的值
                    if let Some(output_display) = output_display {
                        let _ = output_state.update_in(cx, |state, window, cx| {
                            state.set_value(&output_display, window, cx);
                        });
                    }
                }
            }
        })
        .detach();
    }

    pub fn select_output_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let path_receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("选择输出目录".into()),
        });

        let output_state = self.output_file_state.clone();
        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            if let Ok(Ok(Some(paths))) = path_receiver.await {
                if let Some(dir_path) = paths.into_iter().next() {
                    // 获取完整的输出文件路径（目录 + 自动生成的文件名）
                    let output_path = this
                        .update_in(cx, |this, _window, _cx| {
                            this.state.set_output_dir(dir_path);
                            this.state.output_path.clone()
                        })
                        .ok()
                        .flatten();

                    // 更新 InputState 的值
                    if let Some(path) = output_path {
                        let display = path.to_string_lossy().to_string();
                        let _ = output_state.update_in(cx, |state, window, cx| {
                            state.set_value(&display, window, cx);
                        });
                    }
                }
            }
        })
        .detach();
    }

    fn sync_select_to_state(&mut self, cx: &mut Context<Self>) {
        // 同步布局选择
        if let Some(idx) = self.layout_select.read(cx).selected_index(cx) {
            self.state.layout_mode = match idx.row {
                0 => LayoutMode::Horizontal,
                1 => LayoutMode::Vertical,
                2 => LayoutMode::Radial,
                _ => LayoutMode::Horizontal,
            };
        }

        // 同步处理模式选择
        if let Some(idx) = self.process_select.read(cx).selected_index(cx) {
            self.state.process_mode = match idx.row {
                0 => ProcessMode::Slice,
                1 => ProcessMode::Hue,
                _ => ProcessMode::Slice,
            };
        }

        // 同步采样模式选择
        if let Some(idx) = self.sample_select.read(cx).selected_index(cx) {
            self.state.sample_mode = match idx.row {
                0 => SampleMode::Row,
                1 => SampleMode::Column,
                _ => SampleMode::Row,
            };
        }
    }

    pub fn generate_spectrum(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // 同步选择状态
        self.sync_select_to_state(cx);

        // 从 InputState 读取值
        let width_text = self.width_state.read(cx).text().to_string();
        let height_text = self.height_state.read(cx).text().to_string();
        let width: u32 = width_text.parse().unwrap_or(1920);
        let height: u32 = height_text.parse().unwrap_or(200);
        self.state.width = width;
        self.state.height = height;

        let input_path = match &self.state.input_path {
            Some(p) => p.clone(),
            None => {
                window.push_notification(Notification::warning("请选择输入视频"), cx);
                return;
            }
        };

        let output_path = match &self.state.output_path {
            Some(p) => p.clone(),
            None => {
                window.push_notification(Notification::warning("请选择输出路径"), cx);
                return;
            }
        };

        self.state.processing = true;
        self.state.progress.progress = 0.0;
        self.state.progress.stage = "准备中".into();
        cx.notify();

        let (frame_count, band_length, inner_radius) = match self.state.layout_mode {
            LayoutMode::Horizontal => (width, height, 0),
            LayoutMode::Vertical => (height, width, 0),
            LayoutMode::Radial => (width, height, self.state.inner_radius),
        };

        let config = ProcessorConfig {
            frame_count,
            band_length,
            inner_radius,
            sample_mode: self.state.sample_mode,
            layout_mode: self.state.layout_mode,
            process_mode: self.state.process_mode,
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
            let processor = Processor::new(config);
            processor.generate_spectrum_with_progress(
                &input_path,
                &output_path,
                Some(progress_callback),
            )
        });

        // 启动进度更新任务
        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            // 轮询进度更新
            loop {
                // 尝试接收进度更新
                match progress_rx.try_recv() {
                    Ok((stage, current, total, progress)) => {
                        let _ = this.update_in(cx, |this, _window, cx| {
                            this.state.progress.stage = stage.into();
                            this.state.progress.current_frame = current;
                            this.state.progress.total_frames = total;
                            this.state.progress.progress = progress;
                            cx.notify();
                        });
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        // 检查线程是否完成
                        if handle.is_finished() {
                            break;
                        }
                        // 短暂等待
                        gpui::Timer::after(std::time::Duration::from_millis(50)).await;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                }
            }

            // 获取最终结果
            let result = handle.join();

            let _ = this.update_in(cx, |this, window, cx| {
                this.state.processing = false;
                this.state.progress.progress = 1.0;
                this.state.progress.stage = "完成".into();

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
        let theme = cx.theme();
        let bg_color = theme.background;
        let border_color = theme.border;

        // 先渲染各个部分
        let file_section = render_file_section(&self.input_file_state, &self.output_file_state, cx);
        let config_section = render_config_section(
            &self.state,
            &self.width_state,
            &self.height_state,
            &self.radius_slider,
            &self.layout_select,
            &self.process_select,
            &self.sample_select,
            cx,
        );
        let action_section = render_action_section(&self.state, cx);

        v_flex()
            .size_full()
            .bg(bg_color)
            // 中间可滚动区域
            .child(
                div()
                    .id("scroll-container")
                    .flex_1()
                    .min_h_0() // 允许收缩
                    .overflow_y_scrollbar()
                    .child(
                        v_flex()
                            .p_4()
                            .gap_4()
                            .child(file_section)
                            .child(config_section),
                    ),
            )
            // 操作区（固定底部）
            .child(
                div()
                    .flex_shrink_0()
                    .p_4()
                    .border_t_1()
                    .border_color(border_color)
                    .child(action_section),
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
