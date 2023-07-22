use log::error;
use vizia::prelude::*;
use crate::common::Simulator;
use crate::gui_vizia::gui_components::grid::Grid;
use crate::gui_vizia::{GuiData, GuiEvent};

pub struct SimPane {

}

impl View for SimPane {}

impl SimPane {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                // Grid area
                Grid::new(cx, |cx| {
                    // (re-)bind all components when simulator changed
                    Binding::new(
                        cx,
                        GuiData::simulator.then(Simulator::ordered_components),
                        |cx, wrapper_oc| {
                            VStack::new(cx, |cx| {
                                let oc = wrapper_oc.get(cx);
                                for (i, c) in oc.iter().enumerate() {
                                    error!("comp id {}", i);
                                    VStack::new(cx, |cx| {
                                        c.view(cx);
                                    })
                                        .position_type(PositionType::SelfDirected)
                                        .size(Auto)
                                        .on_mouse_down(
                                            move |ex, button| {
                                                if button == MouseButton::Right {
                                                    error!("on_mouse_down {:?}", i);
                                                    ex.emit(GuiEvent::ShowLeftPanel(i))
                                                }
                                            },
                                        );
                                }
                            })
                                .border_color(Color::black())
                                .border_width(Pixels(1.0))
                                .overflow(Overflow::Hidden);
                        },
                    )
                })
                    .height(Pixels(1080.0))
                    .width(Pixels(1920.0));
            })
                // .size(Units::Pixels(300.0))
                .class("bg-default");
        })
    }
}