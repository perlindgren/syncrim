mod leftpane;
mod simpane;
mod rightpane;

use serde_derive::{Deserialize, Serialize};
use vizia::prelude::*;

use leftpane::LeftPane;
use rightpane::RightPane;
use simpane::SimPane;

#[derive(Serialize, Deserialize)]
pub struct SimView {
    ratio: (f32, f32, f32)
}

impl View for SimView {}

impl SimView {
    pub fn new(cx: &mut Context, ratio: (f32, f32, f32)) -> Handle<Self> {
        Self{
            ratio
        }.build(cx, |cx|{
            HStack::new(cx, |cx| {
                LeftPane::new(cx).width(Percentage(ratio.0));
                SimPane::new(cx).width(Percentage(ratio.1));
                RightPane::new(cx).width(Percentage(ratio.2));
            });
        })
    }
}
