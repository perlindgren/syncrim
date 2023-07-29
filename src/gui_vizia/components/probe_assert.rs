use crate::{
    common::{Component, SignalValue, Simulator, ViziaComponent, V},
    components::ProbeAssert,
    gui_vizia::{popup::build_popup, tooltip::new_component_tooltip, GuiData},
};
use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeAssert {
    // create view
    fn view<'a>(&'a self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, |cx| {
            trace!("---- Create ProbeAssert View");

            let values = self.values.clone();

            View::build(ProbeAssertView {}, cx, |cx| {
                let input = self.input.clone();
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
                // NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
                build_popup(cx, self.get_id_ports());
            })
            .position_type(PositionType::SelfDirected)
            .left(Pixels(self.pos.0 - 10.0))
            .top(Pixels(self.pos.1 - 10.0))
            // .width(Pixels(20.0)) // TODO, maybe some max width
            .width(Auto)
            .height(Pixels(20.0))
            // TODO: do we want/need tooltip/popup for constants
            .on_press(|ex| ex.emit(PopupEvent::Switch))
            .tooltip(|cx| new_component_tooltip(cx, self));
        })
    }
}
pub struct ProbeAssertView {}

impl View for ProbeAssertView {
    fn element(&self) -> Option<&'static str> {
        Some("ProbeAssert")
    }
}
