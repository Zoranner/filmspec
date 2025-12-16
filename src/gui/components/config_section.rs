//! 配置区域组件

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    label::Label,
    select::{Select, SelectState},
    slider::{Slider, SliderEvent, SliderState},
    v_flex, ActiveTheme, IndexPath,
};

use crate::mode::{LayoutMode, ProcessMode};
use crate::SampleMode;

/// 配置数据（用于传递给外部）
#[derive(Clone, Debug)]
pub struct ConfigData {
    pub width: u32,
    pub height: u32,
    pub inner_radius: u32,
    pub layout_mode: LayoutMode,
    pub process_mode: ProcessMode,
    pub sample_mode: SampleMode,
}

/// 配置区域组件
pub struct ConfigSection {
    // Slider 状态
    width_slider: Entity<SliderState>,
    height_slider: Entity<SliderState>,
    radius_slider: Entity<SliderState>,
    // Select 状态
    layout_select: Entity<SelectState<Vec<&'static str>>>,
    process_select: Entity<SelectState<Vec<&'static str>>>,
    sample_select: Entity<SelectState<Vec<&'static str>>>,
    // 缓存的值
    width: u32,
    height: u32,
    inner_radius: u32,
    // 是否禁用
    disabled: bool,
}

impl ConfigSection {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // 帧数 Slider 状态
        let width_slider = cx.new(|_cx| {
            SliderState::new()
                .min(100.0)
                .max(7680.0)
                .step(10.0)
                .default_value(1920.0)
        });

        // 监听帧数 Slider 变化事件
        cx.subscribe(&width_slider, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.width = value.end() as u32;
            cx.notify();
        })
        .detach();

        // 带宽 Slider 状态
        let height_slider = cx.new(|_cx| {
            SliderState::new()
                .min(50.0)
                .max(1000.0)
                .step(10.0)
                .default_value(200.0)
        });

        // 监听带宽 Slider 变化事件
        cx.subscribe(&height_slider, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.height = value.end() as u32;
            cx.notify();
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

        // 监听内圆半径 Slider 变化事件
        cx.subscribe(&radius_slider, |this, _, event: &SliderEvent, cx| {
            let SliderEvent::Change(value) = event;
            this.inner_radius = value.end() as u32;
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
            width_slider,
            height_slider,
            radius_slider,
            layout_select,
            process_select,
            sample_select,
            width: 1920,
            height: 200,
            inner_radius: 250,
            disabled: false,
        }
    }

    /// 获取当前处理模式
    fn get_process_mode(&self, cx: &App) -> ProcessMode {
        if let Some(idx) = self.process_select.read(cx).selected_index(cx) {
            match idx.row {
                0 => ProcessMode::Slice,
                1 => ProcessMode::Hue,
                _ => ProcessMode::Slice,
            }
        } else {
            ProcessMode::Slice
        }
    }

    /// 设置禁用状态
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// 获取当前配置数据
    pub fn get_config(&self, cx: &App) -> ConfigData {
        let width = self.width_slider.read(cx).value().end() as u32;
        let height = self.height_slider.read(cx).value().end() as u32;

        let layout_mode = if let Some(idx) = self.layout_select.read(cx).selected_index(cx) {
            match idx.row {
                0 => LayoutMode::Horizontal,
                1 => LayoutMode::Vertical,
                2 => LayoutMode::Radial,
                _ => LayoutMode::Horizontal,
            }
        } else {
            LayoutMode::Horizontal
        };

        let process_mode = if let Some(idx) = self.process_select.read(cx).selected_index(cx) {
            match idx.row {
                0 => ProcessMode::Slice,
                1 => ProcessMode::Hue,
                _ => ProcessMode::Slice,
            }
        } else {
            ProcessMode::Slice
        };

        let sample_mode = if let Some(idx) = self.sample_select.read(cx).selected_index(cx) {
            match idx.row {
                0 => SampleMode::Row,
                1 => SampleMode::Column,
                _ => SampleMode::Row,
            }
        } else {
            SampleMode::Row
        };

        ConfigData {
            width,
            height,
            inner_radius: self.inner_radius,
            layout_mode,
            process_mode,
            sample_mode,
        }
    }

    /// 获取处理模式的输出后缀
    pub fn get_output_suffix(&self, cx: &App) -> &'static str {
        self.get_process_mode(cx).output_suffix()
    }

    /// 是否为切片模式
    fn is_slice_mode(&self, cx: &App) -> bool {
        if let Some(idx) = self.process_select.read(cx).selected_index(cx) {
            idx.row == 0
        } else {
            true
        }
    }

    /// 是否为环形布局
    fn is_radial_mode(&self, cx: &App) -> bool {
        if let Some(idx) = self.layout_select.read(cx).selected_index(cx) {
            idx.row == 2
        } else {
            false
        }
    }
}

impl Render for ConfigSection {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let is_slice = self.is_slice_mode(cx);
        let is_radial = self.is_radial_mode(cx);
        let width = self.width_slider.read(cx).value().end() as u32;
        let height = self.height_slider.read(cx).value().end() as u32;
        let inner_radius = self.radius_slider.read(cx).value().end() as u32;
        let disabled = self.disabled;

        GroupBox::new().outline().title("配置").child(
            v_flex()
                .gap_3()
                // 布局和模式 - 使用 Select 组件
                .child(
                    h_flex()
                        .gap_3()
                        .child(
                            v_flex()
                                .flex_1()
                                .gap_1()
                                .child(Label::new("布局"))
                                .child(Select::new(&self.layout_select).disabled(disabled)),
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .gap_1()
                                .child(Label::new("模式"))
                                .child(Select::new(&self.process_select).disabled(disabled)),
                        ),
                )
                // 采样模式（仅切片模式）
                .when(is_slice, |this| {
                    this.child(
                        v_flex()
                            .gap_1()
                            .child(Label::new("采样"))
                            .child(Select::new(&self.sample_select).disabled(disabled)),
                    )
                })
                // 内圆半径（仅环形模式）- 使用 Slider 组件
                .when(is_radial, |this| {
                    this.child(
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("内圆半径"))
                                    .child(
                                        Label::new(format!("{} px", inner_radius))
                                            .text_color(theme.muted_foreground),
                                    ),
                            )
                            .child(Slider::new(&self.radius_slider).disabled(disabled)),
                    )
                })
                // 帧数 - 使用 Slider 组件
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .justify_between()
                                .child(Label::new("采样帧数"))
                                .child(
                                    Label::new(format!("{}", width))
                                        .text_color(theme.muted_foreground),
                                ),
                        )
                        .child(Slider::new(&self.width_slider).disabled(disabled)),
                )
                // 带宽 - 使用 Slider 组件
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .justify_between()
                                .child(Label::new("条带宽度"))
                                .child(
                                    Label::new(format!("{} px", height))
                                        .text_color(theme.muted_foreground),
                                ),
                        )
                        .child(Slider::new(&self.height_slider).disabled(disabled)),
                ),
        )
    }
}
