//! 文件选择区域组件

use std::path::PathBuf;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::Button,
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex, Disableable, Icon, IconName,
};

/// 文件选择区域组件
pub struct FileSection {
    /// 输入文件路径状态
    input_state: Entity<InputState>,
    /// 输出文件路径状态
    output_state: Entity<InputState>,
    /// 输入路径
    input_path: Option<PathBuf>,
    /// 输出目录（用户手动选择的）
    output_dir: Option<PathBuf>,
    /// 输出路径（完整路径）
    output_path: Option<PathBuf>,
    /// 是否禁用
    disabled: bool,
}

impl FileSection {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state =
            cx.new(|cx| InputState::new(window, cx).default_value("请选择视频文件..."));
        let output_state =
            cx.new(|cx| InputState::new(window, cx).default_value("请选择输出路径..."));

        Self {
            input_state,
            output_state,
            input_path: None,
            output_dir: None,
            output_path: None,
            disabled: false,
        }
    }

    /// 设置禁用状态
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// 获取输入路径
    pub fn input_path(&self) -> Option<&PathBuf> {
        self.input_path.as_ref()
    }

    /// 获取输出路径
    pub fn output_path(&self) -> Option<&PathBuf> {
        self.output_path.as_ref()
    }

    /// 根据处理模式后缀更新输出路径
    pub fn update_output_path(
        &mut self,
        suffix: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ref input) = self.input_path {
            let stem = input
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("spectrum");
            let filename = format!("{}{}.png", stem, suffix);

            // 优先使用用户选择的输出目录，否则使用输入文件所在目录
            let output_dir = self
                .output_dir
                .clone()
                .or_else(|| input.parent().map(|p| p.to_path_buf()));

            let new_path = if let Some(dir) = output_dir {
                dir.join(&filename)
            } else {
                PathBuf::from(&filename)
            };

            // 只有路径变化时才更新
            if self.output_path.as_ref() != Some(&new_path) {
                self.output_path = Some(new_path.clone());
                let display = new_path.to_string_lossy().to_string();
                self.output_state.update(cx, |state, cx| {
                    state.set_value(&display, window, cx);
                });
            }
        }
    }

    /// 选择输入文件
    fn select_input_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let path_receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("选择视频文件".into()),
        });

        let input_state = self.input_state.clone();
        let output_state = self.output_state.clone();

        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            if let Ok(Ok(Some(paths))) = path_receiver.await {
                if let Some(path) = paths.into_iter().next() {
                    let display = path.to_string_lossy().to_string();

                    // 更新状态
                    let output_display = this
                        .update_in(cx, |this, _window, _cx| {
                            this.input_path = Some(path.clone());

                            // 自动生成输出路径（使用默认后缀 _spectrum_slice）
                            let stem = path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("spectrum");
                            let filename = format!("{}_spectrum_slice.png", stem);

                            let output_dir = this
                                .output_dir
                                .clone()
                                .or_else(|| path.parent().map(|p| p.to_path_buf()));

                            if let Some(dir) = output_dir {
                                this.output_path = Some(dir.join(&filename));
                            } else {
                                this.output_path = Some(PathBuf::from(&filename));
                            }

                            this.output_path
                                .as_ref()
                                .map(|p| p.to_string_lossy().to_string())
                        })
                        .ok()
                        .flatten();

                    // 更新输入路径 UI
                    let _ = input_state.update_in(cx, |state, window, cx| {
                        state.set_value(&display, window, cx);
                    });

                    // 更新输出路径 UI
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

    /// 选择输出目录
    fn select_output_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let path_receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("选择输出目录".into()),
        });

        let output_state = self.output_state.clone();

        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            if let Ok(Ok(Some(paths))) = path_receiver.await {
                if let Some(dir_path) = paths.into_iter().next() {
                    let output_path = this
                        .update_in(cx, |this, _window, _cx| {
                            this.output_dir = Some(dir_path.clone());

                            // 如果有输入文件，更新输出路径（使用默认后缀 _spectrum_slice）
                            if let Some(ref input) = this.input_path {
                                let stem = input
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("spectrum");
                                let filename = format!("{}_spectrum_slice.png", stem);
                                this.output_path = Some(dir_path.join(filename));
                            }

                            this.output_path.clone()
                        })
                        .ok()
                        .flatten();

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
}

impl Render for FileSection {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let disabled = self.disabled;

        GroupBox::new().outline().title("文件").child(
            v_flex()
                .gap_3()
                // 输入文件
                .child(
                    v_flex().gap_1().child(Label::new("输入视频")).child(
                        h_flex()
                            .gap_2()
                            .child(Input::new(&self.input_state).disabled(true).flex_1())
                            .child(
                                Button::new("browse-input")
                                    .icon(Icon::new(IconName::FolderOpen))
                                    .disabled(disabled)
                                    .when(!disabled, |btn| {
                                        btn.on_click(cx.listener(|this, _, window, cx| {
                                            this.select_input_file(window, cx);
                                        }))
                                    }),
                            ),
                    ),
                )
                // 输出目录
                .child(
                    v_flex().gap_1().child(Label::new("输出目录")).child(
                        h_flex()
                            .gap_2()
                            .child(Input::new(&self.output_state).disabled(true).flex_1())
                            .child(
                                Button::new("browse-output")
                                    .icon(Icon::new(IconName::Folder))
                                    .disabled(disabled)
                                    .when(!disabled, |btn| {
                                        btn.on_click(cx.listener(|this, _, window, cx| {
                                            this.select_output_file(window, cx);
                                        }))
                                    }),
                            ),
                    ),
                ),
        )
    }
}
