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
            history: Vec::new(),
        }
        .build(cx);

        self.clone().build(cx);
        let plepps = self.clone();

        Textbox::new(cx, ProbeEditView::editable_text)
            .width(Pixels(300.0))
            .on_submit(|cx, text, enter| {
                trace!("text {} {}", text, enter);

                // TODO, do we want to require pressing enter?
                // IMO, you probably just want to enter the value and click simulate
                //if enter {
                if let Ok(signal) = text.parse::<Signal>() {
                    cx.emit(ProbeEditViewSetter::EditableText(text));
                    cx.emit(ProbeEditEvent::Value(signal));
                    error!("signal {}", signal);
                } else {
                    error!("could not parse input");
                }
                //};
            })
            .bind(crate::gui_vizia::GuiData::clock, move |cx, clock| {
                error!("clock --- {}", clock.get(&cx));
                error!("view history: {:?}", plepps.history);
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
                trace!("view: {}", signal);
                *self.history.borrow_mut().last_mut().unwrap() = *signal;
            }
        });
    }
}

#[derive(Lens, Setter, Model)]
pub struct ProbeEditView {
    editable_text: String,
    history: Vec<String>,
}
