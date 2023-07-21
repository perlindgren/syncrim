use vizia::icons;
use vizia::prelude::*;
use crate::common::Simulator;
use crate::gui_vizia::{GuiData, GuiEvent};

pub struct LeftPane {}

impl View for LeftPane {}

impl LeftPane {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx|{
            Binding::new(
                cx,
                GuiData::simulator.then(Simulator::ordered_components),
                |cx, wrapper_oc| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Left").top(Pixels(0.0));
                        let oc = wrapper_oc.get(cx);
                        for (i, c) in oc.iter().enumerate() {
                            VStack::new(cx, |cx| {
                                // left pane bar
                                HStack::new(cx, move |cx| {
                                    Button::new(
                                        cx,
                                        move |cx| {
                                            cx.emit(GuiEvent::ToggleExpandLeftPanel(i))
                                        },
                                        |cx| {
                                            Label::new(
                                                cx,
                                                GuiData::expanded.map(move |expanded| {
                                                    if expanded.contains(&i) {
                                                        // expanded
                                                        icons::ICON_CHEVRON_DOWN
                                                    } else {
                                                        // folded
                                                        icons::ICON_CHEVRON_RIGHT
                                                    }
                                                }),
                                            )
                                                .class("icon")
                                        },
                                    )
                                        .left(Pixels(5.0))
                                        .top(Stretch(1.0))
                                        .bottom(Stretch(1.0))
                                        .right(Stretch(1.0))
                                        .size(Auto);
                                    let (id, _) = c.get_id_ports();

                                    Label::new(cx, &format!("Instance: {}", &id))
                                        .left(Pixels(5.0))
                                        .top(Stretch(1.0))
                                        .bottom(Stretch(1.0))
                                        .right(Stretch(1.0))
                                        .size(Auto);

                                    Button::new(
                                        cx,
                                        move |cx| cx.emit(GuiEvent::HideLeftPanel(i)),
                                        |cx| Label::new(cx, icons::ICON_X).class("icon"),
                                    )
                                        .right(Pixels(1.0))
                                        .top(Pixels(1.0))
                                        .bottom(Pixels(1.0));
                                })
                                    .background_color(Color::lightgrey())
                                    .height(Auto)
                                    .border_color(Color::darkgray())
                                    .border_width(Pixels(1.0));
                                // left view expanded or folded
                                VStack::new(cx, |cx| c.left_view(cx)).display(
                                    GuiData::expanded.map(move |hs_expanded| {
                                        if hs_expanded.contains(&i) {
                                            Display::Flex
                                        } else {
                                            Display::None
                                        }
                                    }),
                                );
                            })
                                .display(
                                    GuiData::visible.map(move |hs_visible| {
                                        if hs_visible.contains(&i) {
                                            Display::Flex
                                        } else {
                                            Display::None
                                        }
                                    }),
                                );
                        }
                    })
                        .border_color(Color::black())
                        .border_width(Pixels(1.0));
                },
            );
        })
    }
}