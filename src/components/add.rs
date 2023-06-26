use crate::common::{Component, Input, Output, OutputType, Ports, SimState, Simulator};
use crate::gui_vizia::GuiData;
use serde::{Deserialize, Serialize};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: String,
    pub pos: (f32, f32),
    pub a_in: Input,
    pub b_in: Input,
}

#[typetag::serde]
impl Component for Add {
    fn to_(&self) {
        println!("add");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.a_in.clone(), self.b_in.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate addition to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input values
        let a_in = simulator.get_input_val(sim_state, &self.a_in);
        let b_in = simulator.get_input_val(sim_state, &self.b_in);

        // compute addition (notice will panic on overflow)
        let value = a_in + b_in;

        println!("eval Add a_in {}, b_in {}, value = {}", a_in, b_in, value);

        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);
    }

    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create Add View");
        View::build(AddView {}, cx, |cx| {
            Label::new(cx, "+")
                .left(Percentage(50.0))
                .top(Pixels(40.0 - 10.0));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 40.0))
        .width(Pixels(40.0))
        .height(Pixels(80.0))
        .tooltip(|cx| {
            let simulator = GuiData::simulator.get(cx);
            let sim_state = GuiData::sim_state.get(cx);
            let (id, ports) = self.get_id_ports();

            VStack::new(cx, |cx| {
                Label::new(cx, &id);
                for input in ports.inputs {
                    HStack::new(cx, |cx| {
                        let v = simulator.get_input_val(&sim_state, &input);
                        Label::new(cx, &input.id);
                        Label::new(cx, v).class("tt_shortcut");
                    })
                    .size(Auto);
                }
                for output in 0..ports.outputs.len() {
                    HStack::new(cx, |cx| {
                        let v =
                            simulator.get(&sim_state, simulator.get_id_start_index(&id) + output);
                        Label::new(cx, &format!("out {}", output));
                        Label::new(cx, v).class("tt_shortcut");
                    })
                    .size(Auto);
                }
            })
            .size(Auto);
        });
    }
}

pub struct AddView {}

impl View for AddView {
    fn element(&self) -> Option<&'static str> {
        Some("Add")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        //println!("Add draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let height = bounds.height();
        let width = bounds.width();
        let top = bounds.top();
        let left = bounds.left();
        let right = bounds.right();
        let bottom = bounds.bottom();

        // top left
        path.move_to(left + 0.5, top + 0.5);

        // top right corner
        path.line_to(left + width * 0.5 + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + height * 0.25 + 0.5);

        // bottom right corner
        path.line_to(right + 0.5, bottom - height * 0.25 + 0.5);
        path.line_to(left + width * 0.5 + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);

        // left outtake
        path.line_to(left + 0.5, bottom - 0.25 * height + 0.5);
        path.line_to(left + width * 0.25 + 0.5, top + 0.5 * height + 0.5);
        path.line_to(left + 0.5, top + 0.25 * height + 0.5);
        path.line_to(left + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
