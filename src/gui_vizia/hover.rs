use crate::{
    common::{Component, Input, Output, OutputType, Ports, Simulator},
    gui_vizia::tooltip::new_component_tooltip,
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[derive(Lens, Data, Clone)]
pub struct Hover {
    hovered: bool,
}

impl Hover {
    pub fn new(cx: &mut Context, pos: (f32, f32)) {
        println!("---- Create Hover View");
        View::build(Hover { hovered: false }, cx, |cx| {
            Element::new(cx)
                .size(Pixels(10.0))
                .background_color(Color::green());
        })
        //  .position_type(PositionType::SelfDirected)
        .bind(Hover::hovered, |cx, hovered| println!("data changed"))
        .left(Pixels(pos.0))
        .top(Pixels(pos.1));

        // .display({
        //     let x = Hover::hovered.map(|hovered| {
        //         if *hovered {
        //             Display::Flex
        //         } else {
        //             Display::None
        //         }
        //     });
        //     Display::Flex
        // });
    }
}

pub enum HoverEvent {
    OnHover,
    OnHoverOut,
}

impl View for Hover {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|hover_event, _meta| match hover_event {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            HoverEvent::OnHover => {
                println!("on_hover_received");
                self.hovered = true;
            }
            HoverEvent::OnHoverOut => {
                println!("on_hover_out_received");
                self.hovered = false;
            }
        });
    }
}
