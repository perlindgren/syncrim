use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    ops::Range,
    panic,
    rc::Rc,
};

use crate::components::InstrMem;
use log::trace;
use syncrim::{
    common::{Input, Simulator},
    gui_vizia::{tooltip::new_component_tooltip, GuiData, ViziaComponent, V},
    vizia::{prelude::*, style::Color},
};

#[typetag::serde]
impl ViziaComponent for InstrMem {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create InstMem View ");
            Label::new(cx, "Instruction Memory")
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
        trace!("---- Create Left Instr View View");
        let data_slice = {
            let mut data_slice = vec![];
            trace!("range {:x?}", self.range);
            for idx in (self.range.start as usize..self.range.end as usize).step_by(4) {
                trace!("idx {:x?}", idx);
                let instr = (*self.bytes.get(&((idx) as usize)).unwrap() as u32) << 24
                    | (*self.bytes.get(&((idx + 1) as usize)).unwrap() as u32) << 16
                    | (*self.bytes.get(&((idx + 2) as usize)).unwrap() as u32) << 8
                    | (*self.bytes.get(&((idx + 3) as usize)).unwrap() as u32);
                data_slice.push(
                    (format!(
                        "0x{:08x}:    {:08x}         ",
                        self.range.start as usize + idx * 4,
                        instr,
                    ) + &panic::catch_unwind(|| {
                        format!("{:?}", asm_riscv::I::try_from(instr).unwrap())
                    })
                    .unwrap_or_else(|_| format!("Unknown instruction"))),
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
                pc_input: self.pc.clone(),
                pc: 0,
            },
            cx,
            |cx| {
                Label::new(cx, "Instruction Memory")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                VirtualList::new(cx, InstrMemView::data_slice, 20.0, |cx, idx, item| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "◉")
                            .on_mouse_up(move |cx, btn| {
                                if btn == MouseButton::Right {
                                    cx.emit(DataEvent::Breakpoint(idx))
                                }
                            })
                            .color(InstrMemView::breakpoints.map(move |breakpoints| {
                                if breakpoints.borrow().contains(&(idx * 4)) {
                                    Color::rgba(255, 0, 0, 255)
                                //red
                                } else {
                                    Color::rgba(255, 255, 255, 0)
                                    //transluscent
                                }
                            }));
                        Label::new(cx, item).on_mouse_up(move |cx, btn| {
                            if btn == MouseButton::Right {
                                cx.emit(DataEvent::Breakpoint(idx))
                            }
                        });
                    })
                    .background_color(InstrMemView::pc.map(move |pc| {
                        if *pc as usize == idx * 4 {
                            Color::yellow()
                        } else {
                            Color::white()
                        }
                    }))
                    .child_left(Pixels(10.0))
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
    pc_input: Input,
    pc: u32,
}

pub enum DataEvent {
    UpdateClock,
    Breakpoint(usize),
}

impl View for InstrMemView {
    fn element(&self) -> Option<&'static str> {
        Some("InstrMemView")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            DataEvent::UpdateClock => {
                self.pc = GuiData::simulator
                    .get(cx)
                    .get_input_value(&self.pc_input)
                    .try_into()
                    .unwrap();
            }
            DataEvent::Breakpoint(idx) => {
                if self.breakpoints.borrow().contains(&(idx * 4)) {
                    trace!("Breakpoint removed");
                    self.breakpoints.borrow_mut().remove(&(idx * 4));
                } else {
                    trace!("New breakpoint!");
                    self.breakpoints.borrow_mut().insert(idx * 4);
                }
            }
        })
    }
}
