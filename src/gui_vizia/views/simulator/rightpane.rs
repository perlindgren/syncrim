use vizia::prelude::*;

pub struct RightPane {}

impl View for RightPane {}

impl RightPane {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx|{
            Label::new(cx, "Right").top(Pixels(0.0));
        })
    }
}
