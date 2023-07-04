// use std::fmt::Alignment;
use crate::{
    common::{Component, Input, Output, OutputType, Ports, Signal, Simulator},
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};
use serde::{Deserialize, Serialize};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[derive(Serialize, Deserialize)]
pub struct Sext {
    pub id: String,
    pub pos: (f32, f32),
    pub sext_in: Input,
    pub in_size: u8,
    pub out_size: u8,
}

#[typetag::serde]
impl Component for Sext {
    fn to_(&self) {
        println!("Sign Extension");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.sext_in.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn evaluate(&self, simulator: &mut Simulator) {
        // get input values
        let mut value = simulator.get_input_val(&self.sext_in);
        let max_size: Signal = 1 << self.in_size as Signal;
        assert!(
            value < max_size,
            "SXT input ({}) greater than allowed input size ({})",
            value,
            max_size
        );

        if (value & 1 << (self.in_size - 1)) != 0 {
            value |= (1 << self.out_size as Signal) - (1 << self.in_size as Signal)
        }

        println!(
            "{}, {}, {}",
            value,
            1 << (self.out_size as Signal),
            1 << (self.in_size as Signal)
        );

        // set output
        simulator.set_id_index(&self.id, 0, value);
    }

    // create viewI
    fn view(&self, cx: &mut Context) {
        println!("---- Create Sext View");
        assert!(self.in_size < self.out_size);

        View::build(SextView {}, cx, move |cx| {
            Label::new(cx, "SXT")
                .width(Pixels(80.0))
                .top(Pixels(20.0))
                .text_align(TextAlign::Center)
                .hoverable(false);
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .left(Pixels(self.pos.0 - 40.0))
        .top(Pixels(self.pos.1 - 20.0))
        .width(Pixels(80.0))
        .height(Pixels(40.0))
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct SextView {}

impl View for SextView {
    fn element(&self) -> Option<&'static str> {
        Some("Sext")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        //println!("Sext draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let height = bounds.height();
        let _width = bounds.width();
        let top = bounds.top();
        let left = bounds.left();
        let right = bounds.right();
        let bottom = bounds.bottom();

        path.move_to(left + 0.5, top + height / 2.0 + 0.5);
        path.line_to(right + 0.5, top + 0.5);
        path.line_to(right + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, top + height / 2.0 + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
