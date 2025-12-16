//! 文件选择区域视图

use gpui::*;
use gpui_component::{
    button::Button,
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex, Icon, IconName,
};

use crate::gui::app::FilmspecApp;

/// 渲染文件选择区域
pub fn render_file_section(
    input_state: &Entity<InputState>,
    output_state: &Entity<InputState>,
    cx: &mut Context<FilmspecApp>,
) -> impl IntoElement {
    GroupBox::new().outline().title("文件").child(
        v_flex()
            .gap_3()
            // 输入文件
            .child(
                v_flex().gap_1().child(Label::new("输入视频")).child(
                    h_flex()
                        .gap_2()
                        .child(Input::new(input_state).disabled(true).flex_1())
                        .child(
                            Button::new("browse-input")
                                .icon(Icon::new(IconName::FolderOpen))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.select_input_file(window, cx);
                                })),
                        ),
                ),
            )
            // 输出目录
            .child(
                v_flex().gap_1().child(Label::new("输出目录")).child(
                    h_flex()
                        .gap_2()
                        .child(Input::new(output_state).disabled(true).flex_1())
                        .child(
                            Button::new("browse-output")
                                .icon(Icon::new(IconName::Folder))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.select_output_file(window, cx);
                                })),
                        ),
                ),
            ),
    )
}
