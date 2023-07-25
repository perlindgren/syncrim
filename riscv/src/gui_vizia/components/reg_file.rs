use crate::components::{Reg, RegFile, RegStore};
use std::{convert::TryFrom, ops::Range};
use syncrim::{
    common::ViziaComponent, gui_vizia::tooltip::new_component_tooltip, vizia::prelude::*,
};

use log::*;

#[derive(Lens)]
pub struct RegTabs {
    list: Vec<&'static str>,
}

impl Model for RegTabs {}

fn range_view(cx: &mut Context, range: Range<u8>) {
    for i in range {
        let item =
            RegFileView::registers.map(move |reg| reg.borrow().get(i as usize).copied().unwrap());

        HStack::new(cx, |cx| {
            Label::new(cx, &format!("{:?}", Reg::try_from(i).unwrap()))
                .width(Pixels(50.0))
                .left(Pixels(10.0));
            Label::new(cx, item);
        })
        .font_size(12.0)
        .size(Auto);
    }
}

#[typetag::serde]
impl ViziaComponent for RegFile {
    // create view
    fn left_view(&self, cx: &mut Context) {
        trace!("---- Create Left Instr View");

        View::build(
            RegFileView {
                registers: self.registers.clone(),
            },
            cx,
            |cx| {
                Label::new(cx, "Register File")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    range_view(cx, RegStore::full_range());
                })
                // .size(Units::Pixels(300.0))
                .class("bg-default");
            },
        );
    }

    // create view
    fn view(&self, cx: &mut Context) {
        trace!("---- Create RegFile View");
        View::build(
            RegFileView {
                registers: self.registers.clone(),
            },
            cx,
            |cx| {
                RegTabs {
                    list: vec!["Lo", "Hi", "Recent"],
                }
                .build(cx);

                TabView::new(cx, RegTabs::list, |cx, item| match item.get(cx) {
                    "Lo" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| range_view(cx, RegStore::lo_range()),
                    ),

                    "Hi" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| range_view(cx, RegStore::hi_range()),
                    ),

                    "Recent" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx)
                                .size(Pixels(200.0))
                                .background_color(Color::blue());
                        },
                    ),

                    _ => unreachable!(),
                })
                .width(Pixels(500.0))
                .height(Pixels(300.0));
            },
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - self.width / 2.0))
        .top(Pixels(self.pos.1 - self.height / 2.0))
        .background_color(Color::lightgrey())
        .border_width(Pixels(1.0))
        .border_color(Color::black())
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

#[derive(Lens, Clone)]
pub struct RegFileView {
    registers: RegStore,
}

impl View for RegFileView {
    fn element(&self) -> Option<&'static str> {
        Some("RegView")
    }
}