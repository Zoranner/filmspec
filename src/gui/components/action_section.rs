//! 操作区域组件

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::Button, button::ButtonVariants, h_flex, label::Label, progress::Progress, v_flex,
    Disableable, Icon, IconName,
};

/// 进度信息
#[derive(Clone, Default)]
pub struct ProgressInfo {
    /// 当前进度 (0.0 - 1.0)
    pub progress: f32,
    /// 当前阶段描述
    pub stage: SharedString,
    /// 已处理帧数
    pub current_frame: u32,
    /// 总帧数
    pub total_frames: u32,
}

/// 生成事件
#[derive(Clone)]
pub struct GenerateEvent;

impl EventEmitter<GenerateEvent> for ActionSection {}

/// 操作区域组件
pub struct ActionSection {
    /// 是否正在处理
    processing: bool,
    /// 进度信息
    progress: ProgressInfo,
    /// 是否可以生成（由外部设置）
    can_generate: bool,
}

impl ActionSection {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            processing: false,
            progress: ProgressInfo::default(),
            can_generate: false,
        }
    }

    /// 设置是否可以生成
    pub fn set_can_generate(&mut self, can: bool) {
        self.can_generate = can;
    }

    /// 开始处理
    pub fn start_processing(&mut self, cx: &mut Context<Self>) {
        self.processing = true;
        self.progress = ProgressInfo {
            progress: 0.0,
            stage: "准备中".into(),
            current_frame: 0,
            total_frames: 0,
        };
        cx.notify();
    }

    /// 更新进度
    pub fn update_progress(
        &mut self,
        stage: impl Into<SharedString>,
        current: u32,
        total: u32,
        progress: f32,
        cx: &mut Context<Self>,
    ) {
        self.progress.stage = stage.into();
        self.progress.current_frame = current;
        self.progress.total_frames = total;
        self.progress.progress = progress;
        cx.notify();
    }

    /// 完成处理
    pub fn finish_processing(&mut self, cx: &mut Context<Self>) {
        self.processing = false;
        self.progress.progress = 1.0;
        self.progress.stage = "完成".into();
        cx.notify();
    }

    /// 是否正在处理
    pub fn is_processing(&self) -> bool {
        self.processing
    }
}

impl Render for ActionSection {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let processing = self.processing;
        let can_generate = self.can_generate && !processing;
        let progress_stage = self.progress.stage.clone();
        let progress_value = self.progress.progress;

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
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(GenerateEvent);
                    })),
            )
    }
}
