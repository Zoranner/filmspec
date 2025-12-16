//! 配置区域视图

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    input::InputState,
    input::NumberInput,
    label::Label,
    select::{Select, SelectState},
    slider::{Slider, SliderState},
    v_flex, ActiveTheme,
};

use crate::gui::app::FilmspecApp;
use crate::gui::state::AppState;

/// 渲染配置区域
#[allow(clippy::too_many_arguments)]
pub fn render_config_section(
    state: &AppState,
    width_state: &Entity<InputState>,
    height_state: &Entity<InputState>,
    radius_slider: &Entity<SliderState>,
    layout_select: &Entity<SelectState<Vec<&'static str>>>,
    process_select: &Entity<SelectState<Vec<&'static str>>>,
    sample_select: &Entity<SelectState<Vec<&'static str>>>,
    cx: &mut Context<FilmspecApp>,
) -> impl IntoElement {
    let theme = cx.theme();

    let is_slice = state.is_slice_mode();
    let is_radial = state.is_radial_mode();
    let inner_radius = radius_slider.read(cx).value().end() as u32;

    let (width_label, height_label) = ("帧数", "带宽");

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
                            .child(Select::new(layout_select)),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_1()
                            .child(Label::new("模式"))
                            .child(Select::new(process_select)),
                    ),
            )
            // 采样模式（仅切片模式）
            .when(is_slice, |this| {
                this.child(
                    v_flex()
                        .gap_1()
                        .child(Label::new("采样"))
                        .child(Select::new(sample_select)),
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
                        .child(Slider::new(radius_slider)),
                )
            })
            // 尺寸 - 使用 NumberInput 组件（放在最后）
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_1()
                            .child(Label::new(width_label))
                            .child(NumberInput::new(width_state)),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_1()
                            .child(Label::new(height_label))
                            .child(NumberInput::new(height_state)),
                    ),
            ),
    )
}
