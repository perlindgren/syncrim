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

        Textbox::new(cx, ProbeEditView::editable_text)
            .width(Pixels(300.0))
            .on_submit(|cx, text, b| {
                error!("text {} {}", text, b);

                if b {
                    if let Ok(signal) = text.parse::<Signal>() {
                        cx.emit(ProbeEditViewSetter::EditableText(text));
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

#[derive(Lens, Setter, Model)]
pub struct ProbeEditView {
    editable_text: String,
}
