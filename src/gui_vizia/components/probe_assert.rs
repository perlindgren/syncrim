use crate::{
    common::{SignalValue, Simulator, ViziaComponent, V},
    components::ProbeAssert,
    gui_vizia::GuiData,
};
use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeAssert {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create ProbeAssert View");

            let values = self.values.clone();

            let input = self.input.clone();
            VStack::new(cx, |cx| {
                Binding::new(
                    cx,
                    crate::gui_vizia::GuiData::simulator.then(Simulator::cycle),
                    move |cx, cycle| {
                        let cycle = cycle.get(cx);
                        let assert = if let Some(value) = values.get(cycle - 1) {
                            *value
                        } else {
                            (SignalValue::Unknown).into()
                        };
                        let simulator = GuiData::simulator.get(cx);
                        let signal = simulator.get_input_signal(&input);
                        if signal == assert {
                            Label::new(cx, &format!("{} == {}", signal, assert))
                                .background_color(Color::lightgreen())
                        } else {
                            Label::new(cx, &format!("{} != {}", signal, assert))
                                .background_color(Color::lightcoral())
                        }
                        .hoverable(false);
                    },
                );
            })
            .size(Auto)
        })
        .top(Pixels(self.pos.1 - 10.0))
        .left(Pixels(self.pos.0 - 10.0))
        .width(Auto)
        // .width(Pixels(20.0)) // TODO, maybe some max width
        .height(Pixels(20.0))
    }
}
