use gpui::*;
use gpui_component::{button::*, *};

use crate::text_input::TextInput;

mod text_input;
mod direct_messages_nav;

pub struct RipcordApp {
    text_input: Entity<TextInput>,
}

impl RipcordApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| TextInput::new(window, cx));

        Self { text_input }
    }
}

impl Render for RipcordApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .bg(rgb(0x080808))
            .text_color(rgb(0xffffff))
            .size_full()
            .child(
                h_flex()
                    .w_full()
                    .justify_center()
                    .py_1()
                    .text_xs()
                    .child("Ripcord"),
            )
            .child(
                h_flex()
                    .size_full()
                    .child("server nav")
                    .child(
                        div()
                            .h_full()
                            .w_64()
                            .border_1()
                            .border_color(rgb(0x202020))
                            .rounded_tl_lg()
                            .child("side nav"),
                    )
                    .child(
                        div()
                            .size_full()
                            .bg(rgb(0x101010))
                            .border_t_1()
                            .border_color(rgb(0x202020))
                            .child("page"),
                    ),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    ..Default::default()
                },
                // WindowOptions::default(),
                |window, cx| {
                    let view = cx.new(|cx| RipcordApp::new(window, cx));
                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
