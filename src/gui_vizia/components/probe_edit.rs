use crate::{
    common::{Signal, ViziaComponent},
    components::ProbeEdit,
};
use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeEdit {
    fn view(&self, cx: &mut Context) {
        trace!("---- Create ProbeEdit View");
        ProbeEditView {
            editable_text: "0".to_string(),
        }
        .build(cx);

        self.clone().build(cx);

        Textbox::new(cx, ProbeEditView::editable_text)
            .width(Pixels(300.0))
            .on_submit(|cx, text, b| {
                error!("text {} {}", text, b);

                if b {
                    if let Ok(signal) = text.parse::<Signal>() {
                        cx.emit(ProbeEditViewSetter::EditableText(text));
                        cx.emit(ProbeEditEvent::Value(signal));
                        error!("signal {}", signal);
                    }
                };
            })
            .left(Pixels(self.pos.0 - 40.0))
            .top(Pixels(self.pos.1 - 20.0))
            .width(Pixels(80.0))
            .height(Pixels(40.0));
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProbeEditEvent {
    Value(Signal),
}

impl Model for ProbeEdit {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            ProbeEditEvent::Value(signal) => {
                error!("view: {}", signal);
                *self.data.borrow_mut() = *signal;
            }
        });
    }
}

#[derive(Lens, Setter, Model)]
pub struct ProbeEditView {
    editable_text: String,
}
