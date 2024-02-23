use crate::{
    common::Simulator,
    components::Cross,
    gui_vizia::{GuiData, ViziaComponent, V},
};
use log::*;
use vizia::prelude::*;

#[typetag::serde]
impl ViziaComponent for Cross {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create Probe View");
            let input = self.input.clone();
            VStack::new(cx, |cx| {
                Binding::new(
                    cx,
                    crate::gui_vizia::GuiData::simulator.then(Simulator::cycle),
                    move |cx, _| {
                        Label::new(cx, {
                            let simulator = GuiData::simulator.get(cx);
                            &format!("{}", simulator.get_input_signal(&input))
                        })
                        .hoverable(false);
                    },
                )
            })
            .size(Auto)
        })
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 10.0))
        // .width(Pixels(20.0)) // TODO, max width?
        .width(Auto)
        .height(Pixels(20.0))
        .background_color(Color::aqua())
    }
}
