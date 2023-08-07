use std::ops::Range;

use crate::{
    common::Simulator,
    components::{Mem, Memory},
    gui_vizia::{GuiData, ViziaComponent, V},
};
use log::*;
use vizia::prelude::*;

#[typetag::serde]
impl ViziaComponent for Mem {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create Mem View ");
            Label::new(cx, "DataMemory")
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
        //We initialize data_slice with the initial state of the memory.
        //from now on, data_slice only gets updated over
        //the relevant (visible) data interval, and when needed (so only on clock)
        //so as to not trigger unnecessary redraws.
        let data_slice = {
            let mut data_slice = vec![];
            let mem = self.memory.clone();
            trace!("range {:x?}", self.range);
            for idx in (self.range.start as usize..self.range.end as usize).step_by(4) {
                trace!("idx {:x?}", idx);

                data_slice.push(format!(
                    "0x{:08x}:    {:02x}{:02x}{:02x}{:02x}",
                    self.range.start as usize + idx * 4,
                    mem.0.borrow().get(&idx).copied().unwrap_or_else(|| 0u8),
                    mem.0
                        .borrow()
                        .get(&(idx + 1))
                        .copied()
                        .unwrap_or_else(|| 0u8),
                    mem.0
                        .borrow()
                        .get(&(idx + 2))
                        .copied()
                        .unwrap_or_else(|| 0u8),
                    mem.0
                        .borrow()
                        .get(&(idx + 3))
                        .copied()
                        .unwrap_or_else(|| 0u8)
                ));
            }
            data_slice
        };
        let view = View::build(
            DataMemView {
                data: self.memory.clone(),
                start: self.range.start as usize,
                data_slice: data_slice,
                //we may init to 0 range, once view opens this will be updated.
                slice_range: Range { start: 0, end: 0 },
            },
            cx,
            |cx| {
                Label::new(cx, "Data Memory")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                VirtualList::new(cx, DataMemView::data_slice, 20.0, |cx, idx, item| {
                    HStack::new(cx, |cx| {
                        //if a value comes into view, update it with fresh data from memory
                        // cx.emit(DataEvent::UpdateVal(idx));
                        Label::new(cx, item);
                    })
                    .child_left(Pixels(10.0))
                })
                .on_change(|cx, range| {
                    cx.emit(DataEvent::UpdateSlice(range));
                });
            },
        )
        .entity();
        Binding::new(
            cx,
            GuiData::simulator.then(Simulator::cycle),
            move |cx, _| cx.emit_to(view, DataEvent::UpdateData),
        );
    }
}

#[derive(Lens, Clone)]
pub struct DataMemView {
    data: Memory,
    start: usize,
    data_slice: Vec<String>,
    slice_range: Range<usize>,
}

pub enum DataEvent {
    UpdateData,
    UpdateSlice(Range<usize>),
}

impl View for DataMemView {
    fn element(&self) -> Option<&'static str> {
        Some("MemView")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            DataEvent::UpdateData => {
                for idx in self.slice_range.clone().into_iter() {
                    if let Some(data_fmt) = self.data_slice.get_mut(idx) {
                        *data_fmt = format!(
                            "0x{:08x}:    {:02x}{:02x}{:02x}{:02x}",
                            idx * 4 + self.start,
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4))
                                .copied()
                                .unwrap_or_else(|| 0u8),
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4 + 1))
                                .copied()
                                .unwrap_or_else(|| 0u8),
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4 + 2))
                                .copied()
                                .unwrap_or_else(|| 0u8),
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4 + 3))
                                .copied()
                                .unwrap_or_else(|| 0u8)
                        );
                    } else {
                        // Why do we end up here, seems wrong
                        println!("{:x}", idx);
                        panic!("Internal error, lookup should always succeed.")
                    }
                }
            }
            DataEvent::UpdateSlice(range) => {
                println!("{:?}", range);
                self.slice_range = range.clone();
                cx.emit(DataEvent::UpdateData);
            }
        })
    }
}
