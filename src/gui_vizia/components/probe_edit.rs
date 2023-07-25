use crate::gui_vizia::GuiData;
use crate::{
    common::{Signal, SignalSigned, SignalUnsigned, Simulator, ViziaComponent},
    components::{ProbeEdit, TextSignal},
};
use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeEdit {
    fn view(&self, cx: &mut Context) {
        trace!("---- Create ProbeEdit View");
        ProbeEditView {
            editable_text: "".to_string(),
        }
        .build(cx);

        let history_bind = self.edit_history.clone();
        let history_submit = self.edit_history.clone();

        Textbox::new(cx, ProbeEditView::editable_text)
            .bind(
                GuiData::simulator.then(Simulator::cycle),
                move |mut handle, cycle| {
                    let cx = handle.context();
                    trace!("bind: clock --- {}", cycle.get(cx));
                    let text = history_bind.read().unwrap().last().unwrap().text.clone();
                    trace!("last text: {:?}", text);
                    cx.emit(ProbeEditViewSetter::EditableText(text));
                },
            )
            .on_submit(move |ex, text, enter| {
                trace!("submit: text {} enter {}", text, enter);
                ex.emit(ProbeEditViewSetter::EditableText(text));
            })
            .on_edit(move |_ex, text| {
                trace!("edit: text {}", text);

                let signal = parse_signal(&text);
                *history_submit.write().unwrap().last_mut().unwrap() = TextSignal {
                    text: text.clone(),
                    signal,
                };
                trace!("signal {:?}", signal);
            })
            .position_type(PositionType::SelfDirected)
            .left(Pixels(self.pos.0 - 40.0))
            .top(Pixels(self.pos.1 - 10.0))
            .width(Pixels(80.0))
            .height(Pixels(20.0));
    }
}

#[derive(Lens, Setter, Model)]
pub struct ProbeEditView {
    editable_text: String,
}

fn parse_signal(text: &str) -> Signal {
    let text = text.trim();

    if let Ok(signal) = text.parse::<SignalSigned>() {
        Signal::Data(signal as SignalUnsigned)
    } else if let Some(hex) = text.strip_prefix("0x") {
        if let Ok(signal) = SignalUnsigned::from_str_radix(hex, 16) {
            Signal::Data(signal)
        } else {
            Signal::Unknown
        }
    } else {
        Signal::Unknown
    }
}
