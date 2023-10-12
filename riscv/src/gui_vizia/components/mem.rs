use std::ops::Range;

use crate::components::{Mem, Memory};
use log::*;
use syncrim::vizia::prelude::*;
use syncrim::{
    common::Simulator,
    //components::{Mem, Memory},
    gui_vizia::{GuiData, ViziaComponent, V},
};

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
                    mem.0.borrow().get(&idx).copied().unwrap_or(0u8),
                    mem.0.borrow().get(&(idx + 1)).copied().unwrap_or(0u8),
                    mem.0.borrow().get(&(idx + 2)).copied().unwrap_or(0u8),
                    mem.0.borrow().get(&(idx + 3)).copied().unwrap_or(0u8),
                ));
            }
            data_slice
        };
        let view = View::build(
            DataMemView {
                data: self.memory.clone(),
                start: self.range.start as usize,
                data_slice,
                //we may init to 0 range, once view opens this will be updated.
                slice_range: Range { start: 0, end: 0 },
            },
            cx,
            |cx| {
                Label::new(cx, "Data Memory")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                VirtualList::new(cx, DataMemView::data_slice, 20.0, |cx, _, item| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, item);
                    })
                    .child_left(Pixels(10.0))
                })
                .on_change(|cx, range| {
                    cx.emit(DataEvent::UpdateScroll(range));
                });
            },
        )
        .entity();
        Binding::new(
            cx,
            GuiData::simulator.then(Simulator::cycle),
            move |cx, _| cx.emit_to(view, DataEvent::UpdateClock),
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
    UpdateClock,
    UpdateScroll(Range<usize>),
    UpdateView(Range<usize>),
}

impl View for DataMemView {
    fn element(&self) -> Option<&'static str> {
        Some("MemView")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            DataEvent::UpdateView(range) => {
                for idx in range.clone() {
                    if let Some(data_fmt) = self.data_slice.get_mut(idx) {
                        *data_fmt = format!(
                            "0x{:08x}:    0x{:02x}{:02x}{:02x}{:02x}",
                            idx * 4 + self.start,
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4))
                                .copied()
                                .unwrap_or(0u8),
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4 + 1))
                                .copied()
                                .unwrap_or(0u8),
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4 + 2))
                                .copied()
                                .unwrap_or(0u8),
                            self.data
                                .0
                                .borrow()
                                .get(&(self.start + idx * 4 + 3))
                                .copied()
                                .unwrap_or(0u8),
                        );
                    } else {
                        // Why do we end up here, seems wrong
                        panic!("Internal error, lookup should always succeed.")
                    }
                }
            }
            DataEvent::UpdateClock => cx.emit(DataEvent::UpdateView(self.slice_range.clone())), //update the entire view on clock.
            DataEvent::UpdateScroll(new_range) => {
                //calculate the "delta" between the view before and after scroll, update that.
                let old_range = self.slice_range.clone();
                self.slice_range = new_range.clone();
                let dirty_range_start = if new_range.start < old_range.start {
                    new_range.start
                } else if new_range.start < old_range.end {
                    old_range.end
                } else {
                    new_range.start
                };
                let dirty_range_end = if new_range.end < old_range.start {
                    new_range.end
                } else if new_range.end < old_range.end {
                    old_range.start
                } else {
                    new_range.end
                };
                let dirty_range = Range {
                    start: dirty_range_start,
                    end: dirty_range_end,
                };

                cx.emit(DataEvent::UpdateView(dirty_range))
            }
        })
    }
}
