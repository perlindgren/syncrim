use vizia::prelude::*;

#[derive(Lens, Data, Clone)]
pub struct Hover {}

impl Hover {
    pub fn new(cx: &mut Context, pos: (f32, f32)) -> Handle<Self> {
        println!("---- Create Hover View");
        View::build(Hover {}, cx, |cx| {
            Element::new(cx)
                .size(Pixels(20.0))
                .background_color(Color::green());
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(pos.0 - 20.0))
        .top(Pixels(pos.1 - 20.0))
    }
}

pub enum HoverEvent {
    OnHover,
    OnHoverOut,
}

impl View for Hover {}
