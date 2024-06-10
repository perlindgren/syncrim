use crate::{
    common::Simulator,
    gui_vizia::{GuiData, ViziaComponent},
};
use vizia::prelude::*;

pub fn new_component_tooltip<'a>(
    cx: &'a mut Context,
    component: &dyn ViziaComponent,
) -> Handle<'a, Tooltip> {
    Tooltip::new(cx, |cx| {
        VStack::new(cx, |cx| {
            let (id, ports) = component.get_id_ports();
            Label::new(cx, &id);

            for input_port in ports.inputs {
                let input = input_port.input;
                HStack::new(cx, |cx| {
                    Label::new(cx, &input.id);
                    Binding::new(
                        cx,
                        GuiData::simulator.then(Simulator::cycle),
                        move |cx, _| {
                            Label::new(
                                cx,
                                &format!(
                                    "{:?}",
                                    GuiData::simulator
                                        .view(cx.data().unwrap())
                                        .unwrap()
                                        .get_input_value(&input)
                                ),
                            )
                            .class("tt_shortcut");
                        },
                    )
                })
                .size(Auto);
            }
            for output in 0..ports.outputs.len() {
                let id_clone = id.clone();
                HStack::new(cx, move |cx| {
                    Label::new(cx, &format!("out {}", output));
                    Binding::new(
                        cx,
                        GuiData::simulator.then(Simulator::cycle),
                        move |cx, _| {
                            Label::new(
                                cx,
                                &format!(
                                    "{:?}",
                                    GuiData::simulator.view(cx.data().unwrap()).unwrap().get(
                                        GuiData::simulator
                                            .view(cx.data().unwrap())
                                            .unwrap()
                                            .get_id_start_index(&id_clone)
                                            + output
                                    )
                                ),
                            )
                            .class("tt_shortcut");
                        },
                    );
                })
                .size(Auto);
            }
        })
        .size(Auto);
    })
}
