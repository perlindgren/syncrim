use crate::common::{Component, Input, Output, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};

// components

#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: String,
    pub pos: (f32, f32),
    pub value: u32, // perhaps vector here ... not sure
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub id: String,
    pub pos: (f32, f32),
    pub r_in: Input,
}

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: String,
    pub pos: (f32, f32),
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: String,
    pub pos: (f32, f32),
    pub a_in: Input,
    pub b_in: Input,
}

// --- not sure where these should go ---

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        println!("constant {:?}", self.value);
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Constants do not take any inputs
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                // Single output value
                outputs: vec![Output::Constant(self.value)],
            },
        )
    }

    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        simulator.set_id_index(sim_state, &self.id, 0, self.value);
    }
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        println!("mux");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        let mut inputs = vec![self.select.clone()];
        let mut m = self.m_in.clone();
        inputs.append(&mut m);

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate selected input value to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input value
        let select = simulator.get_input_val(sim_state, &self.select) as usize;
        let value = simulator.get_input_val(sim_state, &self.m_in[select]);

        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);
    }
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

        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);

        println!(
            "eval: add id {} in {} {} out {}",
            self.id, a_in, b_in, value
        );
    }
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
    fn view(&self, cx: &mut Context, _sim_state: &SimState) {
        println!("----Register View ");
        View::build(RegisterView {}, cx, |_cx| {})
            .position_type(PositionType::SelfDirected)
            .left(Pixels(self.pos.0))
            .top(Pixels(self.pos.1))
            .width(Pixels(10.0))
            .height(Pixels(10.0))
            .bind(SimState::lens_values, |_, y| println!("lense {:?}", y.0));
    }
}

// views
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

pub struct RegisterView {}

impl View for RegisterView {
    fn element(&self) -> Option<&'static str> {
        Some("Register")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.stroke_path(&mut path, &paint);
    }
}
