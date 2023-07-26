use crate::{
    common::{Component, Signal, SignalData, Simulator, ViziaComponent},
    components::ProbeAssert,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip, GuiData},
};
use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeAssert {
    // create view
    fn view(&self, cx: &mut Context) {
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
                        (SignalData::Unknown).into()
                    };
                    let simulator = GuiData::simulator.get(cx);
                    let signal = simulator.get_input_signal(&input);
                    Label::new(cx, &format!("{} == {}", signal, assert))
                        .hoverable(false)
                        .background_color(if signal == assert {
                            Color::lightgreen()
                        } else {
                            Color::lightcoral()
                        });
                },
            );
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
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
    }
}
pub struct ProbeAssertView {}

impl View for ProbeAssertView {
    fn element(&self) -> Option<&'static str> {
        Some("ProbeAssert")
    }
}
