use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    ops::Range,
    panic,
    rc::Rc,
};

use crate::components::InstrMem;
use log::trace;
use syncrim::{
    common::Simulator,
    gui_vizia::{tooltip::new_component_tooltip, GuiData, ViziaComponent, V},
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for InstrMem {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, move |cx| Label::new(cx, "Instruction Memory"))
            .position_type(PositionType::SelfDirected)
            .left(Pixels(self.pos.0 - 50.0))
            .top(Pixels(self.pos.1 - 100.0))
            .width(Pixels(200.0))
            .height(Pixels(100.0))
            .tooltip(|cx| new_component_tooltip(cx, self))
    }
    fn left_view(&self, cx: &mut Context) {
        trace!("---- Create Left Mem View");
        //We initialize data_slice with the initial state of the memory.
        //from now on, data_slice only gets updated over
        //the relevant (visible) data interval, and when needed (so only on clock)
        //so as to not trigger unnecessary redraws.
        let data_slice = {
            let mut data_slice = vec![];
            let mem = self.bytes.clone();
            trace!("range {:x?}", self.range);
            for idx in (self.range.start as usize..self.range.end as usize).step_by(4) {
                trace!("idx {:x?}", idx);
                let instr = (*self.bytes.get(&((idx) as usize)).unwrap() as u32) << 24
                    | (*self.bytes.get(&((idx + 1) as usize)).unwrap() as u32) << 16
                    | (*self.bytes.get(&((idx + 2) as usize)).unwrap() as u32) << 8
                    | (*self.bytes.get(&((idx + 3) as usize)).unwrap() as u32);
                data_slice.push(
                    format!(
                        "0x{:08x}:    {:08x}        ",
                        self.range.start as usize + idx * 4,
                        instr,
                    ) + &panic::catch_unwind(|| {
                        format!("{:?}", asm_riscv::I::try_from(instr).unwrap())
                    })
                    .unwrap_or_else(|_| format!("Unknown instruction")),
                );
            }
            data_slice
        };
        let view = View::build(
            InstrMemView {
                data: self.bytes.clone(),
                start: self.range.start as usize,
                data_slice,
                //we may init to 0 range, once view opens this will be updated.
                slice_range: Range { start: 0, end: 0 },
                breakpoints: self.breakpoints.clone(),
            },
            cx,
            |cx| {
                Label::new(cx, "Data Memory")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                VirtualList::new(cx, InstrMemView::data_slice, 20.0, |cx, idx, item| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, item).on_mouse_up(move |cx, btn| {
                            if btn == MouseButton::Right {
                                //println!("Pressed {}", idx)
                                cx.emit(DataEvent::Breakpoint(idx))
                            }
                        });
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
pub struct InstrMemView {
    data: BTreeMap<usize, u8>,
    start: usize,
    data_slice: Vec<String>,
    slice_range: Range<usize>,
    breakpoints: Rc<RefCell<HashSet<usize>>>,
}

pub enum DataEvent {
    UpdateClock,
    UpdateScroll(Range<usize>),
    UpdateView(Range<usize>),
    Breakpoint(usize),
}

impl View for InstrMemView {
    fn element(&self) -> Option<&'static str> {
        Some("InstrMemView")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            DataEvent::UpdateView(range) => {
                trace!("UpdateView {:?}", range);
                for idx in range.clone().step_by(4) {
                    if let Some(data_fmt) = self.data_slice.get_mut(idx) {
                        trace!("idx {:x?}", idx);
                        let instr = (*self.data.get(&((idx * 4) as usize)).unwrap() as u32) << 24
                            | (*self.data.get(&((idx * 4 + 1) as usize)).unwrap() as u32) << 16
                            | (*self.data.get(&((idx * 4 + 2) as usize)).unwrap() as u32) << 8
                            | (*self.data.get(&((idx * 4 + 3) as usize)).unwrap() as u32);
                        *data_fmt = (format!(
                            "0x{:08x}:    {:08x}        ",
                            self.start as usize + idx * 4,
                            instr,
                        ) + &panic::catch_unwind(|| {
                            format!("{:?}", asm_riscv::I::try_from(instr).unwrap())
                        })
                        .unwrap_or_else(|_| format!("Unknown instruction")));
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
            DataEvent::Breakpoint(idx) => {
                if self.breakpoints.borrow().contains(&(idx * 4)) {
                    trace!("Breakpoint exists already");
                    self.breakpoints.borrow_mut().remove(&(idx * 4));
                } else {
                    trace!("New breakpoint!");
                    self.breakpoints.borrow_mut().insert(idx * 4);
                }
            }
        })
    }
}
