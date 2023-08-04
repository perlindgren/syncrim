use crate::{
    common::{Input, SignalSigned, SignalUnsigned, SignalValue, Simulator},
    components::{ProbeHalt, TextSignal},
    gui_vizia::{GuiData, ViziaComponent, V},
    signal::SignalExpr,
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

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Halt on: ");
                    // this won't update I guess
                    Label::new(cx, &format!("{}", self.signal_expr.borrow()));
                })
                .size(Auto);
                self.build_expression(cx, &self.signal_expr.borrow())
            })
            .size(Auto);
        })
        // // textbox for entering constants
        // Textbox::new(cx, ProbeHaltView::editable_text)
        //     .on_submit(move |ex, text, enter| {
        //         trace!("submit: text {} enter {}", text, enter);
        //         ex.emit(ProbeHaltViewSetter::EditableText(text));
        //     })
        //     .on_edit(move |_ex, text| {
        //         trace!("edit: text {}", text);
        //         // let value = parse_signal(&text);
        //         // *history_submit.write().unwrap().last_mut().unwrap() = TextSignal {
        //         //     text: text.clone(),
        //         //     signal: value.into(),
        //         // };
        //         // trace!("signal {:?}", value);
        //     })
        //     .width(Pixels(80.0))
        //     .height(Pixels(20.0));
        // for c in self.inputs {
        //     text
        // }
        // trace!("component {:?}", self.inputs);
        // })
        // });
        .size(Auto)
        .left(Pixels(self.pos.0 - 40.0))
        .top(Pixels(self.pos.1 - 10.0))
        .background_color(Color::lightgoldenrodyellow())
    }
}

#[derive(Lens, Model, Setter)]
pub struct InputIdData {
    list: Vec<String>,
    choice: String,
}

#[derive(Lens, Model, Setter)]
pub struct InputFieldData {
    list: Vec<String>,
    choice: String,
}

impl ProbeHalt {
    fn build_expression(&self, cx: &mut Context, signal_expr: &SignalExpr) {
        match signal_expr {
            SignalExpr::BinOp(bin_op, lhs, rhs) => {
                HStack::new(cx, |cx| {
                    self.build_expression(cx, lhs);
                    Button::new(cx, |_| {}, |cx| Label::new(cx, &format!("{}", bin_op))).size(Auto);
                    self.build_expression(cx, rhs);
                })
                .size(Auto);
            }

            //     SignalExpr::Not(e) => unimplemented!(),
            SignalExpr::Constant(c) => {
                Button::new(cx, |_| {}, |cx| Label::new(cx, &format!("{}", c))).size(Auto);
            }
            SignalExpr::Input(Input { id, field }) => {
                trace!("-- Input -- {:?}", self.inputs);
                InputIdData {
                    list: self
                        .inputs
                        .iter()
                        .map(|input| input.id.to_owned())
                        .collect(),
                    choice: id.into(),
                }
                .build(cx);

                let simulator = GuiData::simulator.get(cx);
                let outputs = simulator.id_outputs.get(id);

                trace!("outputs {:?}", outputs);

                HStack::new(cx, |cx| {
                    // Input Id dropdown
                    Dropdown::new(
                        cx,
                        move |cx| Label::new(cx, InputIdData::choice),
                        move |cx| {
                            List::new(cx, InputIdData::list, |cx, _, item| {
                                Label::new(cx, item)
                                    .width(Stretch(1.0))
                                    .cursor(CursorIcon::Hand)
                                    .bind(InputIdData::choice, move |handle, selected| {
                                        if item.get(&handle) == selected.get(&handle) {
                                            handle.checked(true);
                                        }
                                    })
                                    .on_press(move |cx| {
                                        cx.emit(InputIdDataSetter::Choice(item.get(cx)));
                                        cx.emit(PopupEvent::Close);
                                    });
                            })
                            .width(Stretch(1.0));
                        },
                    )
                    .top(Pixels(40.0))
                    .width(Pixels(100.0));

                    Label::new(cx, ".").size(Auto);
                    Button::new(cx, |_| {}, |cx| Label::new(cx, &format!("{}", field)));
                })
                .size(Auto);
            }
            _ => unimplemented!(),
        };
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
