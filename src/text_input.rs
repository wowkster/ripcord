use gpui::*;
use gpui_component::{
    button::*,
    input::{Input, InputEvent, InputState},
    *,
};

pub struct TextInput {
    input_state: Entity<InputState>,
    display_text: SharedString,

    /// We need to keep the subscriptions alive with the Example entity.
    ///
    /// So if the Example entity is dropped, the subscriptions are also dropped.
    /// This is important to avoid memory leaks.
    _subscriptions: Vec<Subscription>,
}

impl TextInput {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name"));

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value = input_state.read(cx).value();
                    this.display_text = format!("Hello, {}!", value).into();
                    cx.notify()
                }
                _ => {}
            }
        })];

        Self {
            input_state,
            display_text: SharedString::default(),
            _subscriptions,
        }
    }
}

impl Render for TextInput {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_5()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(Input::new(&self.input_state))
            .child(self.display_text.clone())
    }
}
