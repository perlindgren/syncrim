use crate::{
    common::{Component, SignalValue, Simulator, ViziaComponent, V},
    components::ProbeStim,
    gui_vizia::{popup::build_popup, tooltip::new_component_tooltip},
};

use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeStim {
    // create view
    fn view<'a>(&'a self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, |cx| {
            trace!("---- Create ProbeStim View");
            let values = self.values.clone();
            View::build(ProbeStimView {}, cx, |cx| {
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
                // NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
                build_popup(cx, self.get_id_ports());
            })
            .position_type(PositionType::SelfDirected)
            .background_color(Color::lightblue())
            .left(Pixels(self.pos.0 - 10.0))
            .top(Pixels(self.pos.1 - 10.0))
            .width(Auto)
            // .width() // TODO, maybe some max width
            .height(Pixels(20.0))
            // TODO: do we want/need tooltip/popup for constants
            .on_press(|ex| ex.emit(PopupEvent::Switch))
            .tooltip(|cx| new_component_tooltip(cx, self));
        })
    }
}
pub struct ProbeStimView {}

impl View for ProbeStimView {
    fn element(&self) -> Option<&'static str> {
        Some("ProbeStim")
    }
}
