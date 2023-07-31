use std::ops::Range;

use crate::{
    components::{Mem, Memory},
    gui_vizia::{ViziaComponent, V},
};
use log::*;
use vizia::prelude::*;

fn range_view(cx: &mut Context) {
    let range = Range {
        start: 0x8000_0000u32,
        end: 0x8000_0020u32,
    };
    for i in range {
        if i % 4 == 0 {
            let item = (
                DataMemView::data.map(move |mem: &Memory| {
                    format!(
                        "0x{:02x}",
                        mem.0.borrow().get(&(i as usize)).copied().unwrap()
                    )
                }),
                DataMemView::data.map(move |mem: &Memory| {
                    format!(
                        "{:02x}",
                        mem.0.borrow().get(&((i + 1) as usize)).copied().unwrap()
                    )
                }),
                DataMemView::data.map(move |mem: &Memory| {
                    format!(
                        "{:02x}",
                        mem.0.borrow().get(&((i + 2) as usize)).copied().unwrap()
                    )
                }),
                DataMemView::data.map(move |mem: &Memory| {
                    format!(
                        "{:02x}",
                        mem.0.borrow().get(&((i + 3) as usize)).copied().unwrap()
                    )
                }),
            );

            HStack::new(cx, |cx| {
                Label::new(cx, &format!("0x{:08x}", i))
                    .width(Pixels(100.0))
                    .left(Pixels(10.0));
                Label::new(cx, item.0);
                Label::new(cx, item.1);
                Label::new(cx, item.2);
                Label::new(cx, item.3);
            })
            .font_size(12.0)
            .size(Auto);
        }
    }
}

#[typetag::serde]
impl ViziaComponent for Mem {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create Mem View ");
            Label::new(cx, "DataMemory")
                .hoverable(false)
                .left(Pixels(10.0))
                .top(Pixels(10.0))
                .hoverable(false)
        })
        .left(Pixels(self.pos.0 - self.width / 2.0))
        .top(Pixels(self.pos.1 - self.height / 2.0))
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .background_color(Color::lightgrey())
    }

    fn left_view(&self, cx: &mut Context) {
        trace!("---- Create Left Mem View");

        View::build(
            DataMemView {
                data: self.memory.clone(),
            },
            cx,
            |cx| {
                Label::new(cx, "Register File")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    range_view(cx);
                })
                // .size(Units::Pixels(300.0))
                .class("bg-default");
            },
        );
    }
}

#[derive(Lens, Clone)]
pub struct DataMemView {
    data: Memory,
}

impl View for DataMemView {
    fn element(&self) -> Option<&'static str> {
        Some("MemView")
    }
}
