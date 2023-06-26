use crate::common::{Component, Input, Output, OutputType, Ports, SimState, Simulator};
use crate::gui_vizia::tooltip::new_tooltip;
use crate::gui_vizia::GuiData;
use serde::{Deserialize, Serialize};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub id: String,
    pub pos: (f32, f32),
    pub r_in: Input,
}

#[typetag::serde]
impl Component for Register {
    fn to_(&self) {
        println!("register");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Vector of inputs
                inputs: vec![self.r_in.clone()],
                out_type: OutputType::Sequential,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate input value to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input value
        let value = simulator.get_input_val(sim_state, &self.r_in);
        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);
        println!("eval: register id {} in {}", self.id, value);
    }

    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create Register View ");

        let handle = View::build(RegisterView {}, cx, |_cx| {})
            .position_type(PositionType::SelfDirected)
            .left(Pixels(self.pos.0 - 10.0))
            .top(Pixels(self.pos.1 - 15.0))
            .width(Pixels(20.0))
            .height(Pixels(30.0));
        new_tooltip(handle, cx, self as &dyn Component);

        //.on_over(|ex| println!("over register"))
        // .tooltip(|cx| {
        //     let simulator = GuiData::simulator.get(cx);
        //     let sim_state = GuiData::sim_state.get(cx);
        //     let (id, ports) = self.get_id_ports();

        //     VStack::new(cx, |cx| {
        //         Label::new(cx, &id);
        //         for input in ports.inputs {
        //             HStack::new(cx, |cx| {
        //                 let v = simulator.get_input_val(&sim_state, &input);
        //                 Label::new(cx, &input.id);
        //                 Label::new(cx, v).class("tt_shortcut");
        //             })
        //             .size(Auto);
        //         }
        //         for output in 0..ports.outputs.len() {
        //             HStack::new(cx, |cx| {
        //                 let v = simulator
        //                     .get(&sim_state, simulator.get_id_start_index(&id) + output);
        //                 Label::new(cx, &format!("out {}", output));
        //                 Label::new(cx, v).class("tt_shortcut");
        //             })
        //             .size(Auto);
        //         }
        //     })
        //     .size(Auto);
        // });
    }
}

pub struct RegisterView {}

impl View for RegisterView {
    fn element(&self) -> Option<&'static str> {
        Some("Register")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let left = bounds.left();
        let right = bounds.right();
        let top = bounds.top();
        let bottom = bounds.bottom();
        let width = bounds.width();
        let height = bounds.height();

        // draw box
        path.move_to(left + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + 0.5);
        path.line_to(right + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, top + 0.5);

        // draw cut out
        path.line_to(left + width * 0.5 + 0.5, top + height * 0.25 + 0.5);
        path.line_to(right + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
