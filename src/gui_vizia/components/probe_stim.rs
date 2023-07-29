use crate::{
    common::{SignalValue, Simulator},
    components::ProbeStim,
    gui_vizia::{ViziaComponent, V},
};
use log::*;
use vizia::prelude::*;

#[typetag::serde]
impl ViziaComponent for ProbeStim {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create ProbeStim View");
            let values = self.values.clone();
            VStack::new(cx, |cx| {
                Binding::new(
                    cx,
                    crate::gui_vizia::GuiData::simulator.then(Simulator::cycle),
                    move |cx, cycle| {
                        let cycle = cycle.get(cx);
                        let rhs = if let Some(value) = values.get(cycle - 1) {
                            *value
                        } else {
                            (SignalValue::Unknown).into()
                        };
                        Label::new(cx, &format!("{}", rhs)).hoverable(false);
                    },
                );
            })
            .size(Auto)
        })
        .top(Pixels(self.pos.1 - 10.0))
        .left(Pixels(self.pos.0 - 10.0))
        .width(Auto)
        // .width() // TODO, maybe some max width
        .background_color(Color::lightblue())
        .height(Pixels(20.0))
    }
}
