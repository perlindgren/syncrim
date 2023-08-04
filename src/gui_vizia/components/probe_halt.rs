use crate::{
    common::{SignalSigned, SignalUnsigned, SignalValue, Simulator},
    components::{ProbeHalt, TextSignal},
    gui_vizia::{GuiData, ViziaComponent, V},
};
use log::*;
use vizia::prelude::*;

#[typetag::serde]
impl ViziaComponent for ProbeHalt {
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, |cx| {
            trace!("---- Create ProbeHalt View");
            ProbeHaltView {
                editable_text: "".to_string(),
            }
            .build(cx);

            Textbox::new(cx, ProbeHaltView::editable_text)
                .on_submit(move |ex, text, enter| {
                    trace!("submit: text {} enter {}", text, enter);
                    ex.emit(ProbeHaltViewSetter::EditableText(text));
                })
                .on_edit(move |_ex, text| {
                    trace!("edit: text {}", text);

                    // let value = parse_signal(&text);
                    // *history_submit.write().unwrap().last_mut().unwrap() = TextSignal {
                    //     text: text.clone(),
                    //     signal: value.into(),
                    // };
                    // trace!("signal {:?}", value);
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
pub struct ProbeHaltView {
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
