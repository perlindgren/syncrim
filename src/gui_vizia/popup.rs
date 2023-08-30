use crate::{
    common::{Id, Ports, Simulator},
    gui_vizia::GuiData,
};
use vizia::prelude::*;

use log::*;

use crate::gui_vizia::popup::popup_data_derived_lenses::is_open;

pub fn build_popup(cx: &mut Context, id_ports: (Id, Ports)) -> Handle<'_, Popup<Wrapper<is_open>>> {
    trace!("build_popup");
    PopupData::default().build(cx);

    Popup::new(cx, PopupData::is_open, true, move |cx| {
        VStack::new(cx, |cx| {
            let (id, ports) = id_ports.clone();
            Label::new(cx, &id);

            for input in ports.inputs {
                let input = input.input;
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
                                    GuiData::simulator.get(cx).get_input_value(&input)
                                ),
                            );
                            //.class("tt_shortcut");
                        },
                    )
                })
                .size(Auto);
            }
            for output in 0..ports.outputs.len() {
                let id_clone = id.clone();
                HStack::new(cx, move |cx| {
                    Label::new(cx, &format!("out {} - ", output));
                    Binding::new(
                        cx,
                        GuiData::simulator.then(Simulator::cycle),
                        move |cx, _| {
                            Label::new(
                                cx,
                                &format!(
                                    "{}",
                                    GuiData::simulator.get(cx).get(
                                        GuiData::simulator.get(cx).get_id_start_index(&id_clone)
                                            + output
                                    )
                                ),
                            );
                            // .class("tt_shortcut");
                        },
                    );
                })
                .size(Auto);
            }
        })
        .size(Auto);
    })
    //.on_blur(|cx| cx.emit(PopupEvent::Close))
    .size(Auto)
    // .position_type(PositionType::SelfDirected)
    .top(Percentage(100.0)) // place popup below
    .left(Percentage(0.0))
}
