use vizia::prelude::*;

#[derive(Lens, Data, Clone)]
pub struct Hover {
    pinned: bool,
}

impl Hover {
    pub fn new(cx: &mut Context, pinned: impl Lens<Target = bool>) -> Handle<Self> {
        println!("---- Create Hover View");
        View::build(Hover { pinned: false }, cx, |cx| {
            Element::new(cx)
                .size(Pixels(20.0))
                .background_color(Color::green());
            Checkbox::new(cx, pinned)
                .on_toggle(|cx| cx.emit(HoverEvent::PinnedToggle))
                .id("checkbox_1");
            Label::new(cx, "Checkbox 1").describing("checkbox_1");
        })
        .position_type(PositionType::SelfDirected)
        .top(Percentage(100.0))
        .translate((Pixels(0.0), Pixels(10.0)))
    }
}

pub enum HoverEvent {
    OnHover,
    OnHoverOut,
    PinnedToggle,
}

impl View for Hover {}
