use crate::{
    common::{SignalSigned, SignalUnsigned, SignalValue, Simulator},
    components::{ProbeEdit, TextSignal},
    gui_vizia::{GuiData, ViziaComponent, V},
};
use log::*;
use vizia::prelude::*;

#[typetag::serde]
impl ViziaComponent for ProbeEdit {
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, |cx| {
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

                    let value = parse_signal(&text);
                    *history_submit.write().unwrap().last_mut().unwrap() = TextSignal {
                        text: text.clone(),
                        signal: value.into(),
                    };
                    trace!("signal {:?}", value);
                })
                .width(Pixels(80.0))
                .height(Pixels(20.0));
        })
        .size(Auto)
        .left(Pixels(self.pos.0 - 40.0))
        .top(Pixels(self.pos.1 - 10.0))
    }
}

#[derive(Lens, Setter, Model)]
pub struct ProbeEditView {
    editable_text: String,
}

fn parse_signal(text: &str) -> SignalValue {
    let text = text.trim();

    if let Ok(signal) = text.parse::<SignalSigned>() {
        (signal as SignalUnsigned).into()
    } else if let Some(hex) = text.strip_prefix("0x") {
        if let Ok(signal) = SignalUnsigned::from_str_radix(hex, 16) {
            signal.into()
        } else {
            SignalValue::Unknown
        }
    } else {
        SignalValue::Unknown
    }
}
