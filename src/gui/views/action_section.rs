//! 操作区域视图

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::Button, button::ButtonVariants, h_flex, label::Label, progress::Progress, v_flex,
    Disableable, Icon, IconName,
};

use crate::gui::app::FilmspecApp;
use crate::gui::state::AppState;

/// 渲染操作区域
pub fn render_action_section(state: &AppState, cx: &mut Context<FilmspecApp>) -> impl IntoElement {
    let processing = state.processing;
    let can_generate = state.can_generate();
    let progress_stage = state.progress.stage.clone();
    let progress_value = state.progress.progress;

    v_flex()
        .gap_3()
        // 进度条
        .when(processing, |this| {
            this.child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Label::new(progress_stage).text_size(px(11.0)))
                            .child(
                                Label::new(format!("{:.0}%", progress_value * 100.0))
                                    .text_size(px(11.0)),
                            ),
                    )
                    .child(Progress::new().value(progress_value * 100.0)),
            )
        })
        // 生成按钮
        .child(
            Button::new("generate")
                .primary()
                .w_full()
                .icon(Icon::new(IconName::ArrowRight))
                .label(if processing {
                    "处理中..."
                } else {
                    "生成光谱"
                })
                .disabled(!can_generate)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.generate_spectrum(window, cx);
                })),
        )
}
