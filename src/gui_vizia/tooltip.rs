use crate::common::Component;
use crate::gui_vizia::GuiData;
use vizia::prelude::*;

// impl<'a> Handle<'a, Component> {
//     pub fn new_tooltip(&self, cx: &mut Context) -> Self {
//         unimplemented!()
//     }
// }

pub fn new_tooltip<'a, T>(
    handle: Handle<'a, T>,
    cx: &mut Context,
    component: &dyn Component,
) -> Handle<'a, T>
where
    T: View,
{
    handle.tooltip(|cx| {
        VStack::new(cx, |cx| {
            let (id, ports) = component.get_id_ports();
            Label::new(cx, &id);

            for input in ports.inputs {
                HStack::new(cx, |cx| {
                    Label::new(cx, &input.id);
                    Binding::new(cx, GuiData::clock, move |cx, _| {
                        Label::new(
                            cx,
                            &format!(
                                "{:?}",
                                GuiData::simulator
                                    .get(cx)
                                    .get_input_val(&GuiData::sim_state.get(cx), &input)
                            ),
                        )
                        .class("tt_shortcut");
                    })
                })
                .size(Auto);
            }
            for output in 0..ports.outputs.len() {
                let id_clone = id.clone();
                HStack::new(cx, move |cx| {
                    Label::new(cx, &format!("out {}", output));
                    Binding::new(cx, GuiData::clock, move |cx, _| {
                        Label::new(
                            cx,
                            &format!(
                                "{:?}",
                                GuiData::simulator.get(cx).get(
                                    &GuiData::sim_state.get(cx),
                                    GuiData::simulator.get(cx).get_id_start_index(&id_clone)
                                        + output
                                )
                            ),
                        )
                        .class("tt_shortcut");
                    });
                    // Label::new(cx, v).class("tt_shortcut");
                })
                .size(Auto);
            }
        })
        .size(Auto);
    })
}
