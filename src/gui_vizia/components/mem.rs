use crate::{
    components::{Mem, Memory},
    gui_vizia::{ViziaComponent, V},
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
                Label::new(cx, "Data Memory")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));

                VirtualList::new(
                    cx,
                    DataMemView::data.map(|mem| {
                        let mut vec = vec![];
                        for i in mem.0.borrow().iter() {
                            if i.0 % 4 == 0 {
                                vec.push(format!(
                                    "0x{:08x}:    0x{:02x}{:02x}{:02x}{:02x}",
                                    i.0,
                                    i.1,
                                    mem.0
                                        .borrow()
                                        .get(&((i.0 + 1) as usize))
                                        .copied()
                                        .unwrap_or_else(|| 0u8),
                                    mem.0
                                        .borrow()
                                        .get(&((i.0 + 2) as usize))
                                        .copied()
                                        .unwrap_or_else(|| 0u8),
                                    mem.0
                                        .borrow()
                                        .get(&((i.0 + 3) as usize))
                                        .copied()
                                        .unwrap_or_else(|| 0u8)
                                ));
                            }
                        }
                        vec
                    }),
                    20.0,
                    |cx, _, item| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, item);
                        })
                        .child_left(Pixels(10.0))
                    },
                );
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
