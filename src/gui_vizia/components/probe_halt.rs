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
pub struct InputData {
    list: Vec<String>,
    choice: String,
}

#[derive(Lens, Model, Setter)]
pub struct BinOpData {
    list: Vec<String>,
    choice: String,
}

impl ProbeHalt {
    fn build_expression(&self, cx: &mut Context, signal_expr: &SignalExpr) {
        match signal_expr {
            SignalExpr::BinOp(bin_op, lhs, rhs) => {
                HStack::new(cx, |cx| {
                    self.build_expression(cx, lhs);
                    trace!("bin op");
                    BinOpData {
                        list: [
                            "&&".to_string(),
                            "||".to_string(),
                            "==".to_string(),
                            ">".to_string(),
                            ">s".to_string(),
                        ]
                        .to_vec(),
                        choice: format!("{}", bin_op),
                    }
                    .build(cx);
                    //
                    // Button::new(cx, |_| {}, |cx| Label::new(cx, &format!("{}", bin_op))).size(Auto);
                    Dropdown::new(
                        cx,
                        move |cx| Label::new(cx, BinOpData::choice).size(Auto),
                        move |cx| {
                            List::new(cx, BinOpData::list, |cx, _, item| {
                                Label::new(cx, item)
                                    .cursor(CursorIcon::Hand)
                                    .bind(InputData::choice, move |handle, selected| {
                                        if item.get(&handle) == selected.get(&handle) {
                                            handle.checked(true);
                                        }
                                    })
                                    .on_press(move |cx| {
                                        cx.emit(BinOpDataSetter::Choice(item.get(cx)));
                                        cx.emit(PopupEvent::Close);
                                    })
                                    .size(Auto);
                            })
                            .size(Auto);
                        },
                    )
                    .width(Pixels(40.0));
                    self.build_expression(cx, rhs);
                })
                .size(Auto);
            }

            SignalExpr::Not(e) => {
                HStack::new(cx, |cx| {
                    Label::new(cx, "!");
                    self.build_expression(cx, e);
                })
                .size(Auto);
            }
            SignalExpr::Constant(c) => {
                Button::new(cx, |_| {}, |cx| Label::new(cx, &format!("{}", c))).size(Auto);
            }
            SignalExpr::Input(Input { id, field }) => {
                trace!("-- Input -- {:?}", self.inputs);
                InputData {
                    list: self
                        .inputs
                        .iter()
                        .map(|input| format!("{}.{}", input.id, input.field))
                        .collect(),
                    choice: format!("{}.{}", id, field),
                }
                .build(cx);

                // Input Id.Field dropdown
                Dropdown::new(
                    cx,
                    move |cx| Label::new(cx, InputData::choice).size(Auto),
                    move |cx| {
                        List::new(cx, InputData::list, |cx, _, item| {
                            Label::new(cx, item)
                                // .width(Stretch(1.0))
                                .cursor(CursorIcon::Hand)
                                .bind(InputData::choice, move |handle, selected| {
                                    if item.get(&handle) == selected.get(&handle) {
                                        handle.checked(true);
                                    }
                                })
                                .on_press(move |cx| {
                                    cx.emit(InputDataSetter::Choice(item.get(cx)));
                                    cx.emit(PopupEvent::Close);
                                })
                                .size(Auto);
                        })
                        .size(Auto);
                    },
                )
                // .size(Auto);
                .width(Pixels(140.0));
            }
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
