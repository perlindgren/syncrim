use serde::{Deserialize, Serialize};
use std::cell::Cell;
use syncrim::{
    common::{Component, Input, Output, OutputType, Ports, Signal, Simulator},
    gui_vizia::tooltip::new_component_tooltip,
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[derive(Serialize, Deserialize)]
pub struct RegFile {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub read_addr1: Input,
    pub read_addr2: Input,
    pub write_data: Input,
    pub write_addr: Input,
    pub write_enable: Input,

    // data, should be an array of 32 Cells, but its harder to manage in Rust (Cell not Copy)
    pub registers: Vec<Cell<u32>>,
}

impl RegFile {
    fn read_reg(&self, simulator: &Simulator, input: &Input) -> u32 {
        let read_addr = simulator.get_input_val(input) as usize;
        println!("read_addr {}", read_addr);

        // mips always reads 0;
        if read_addr > 0 {
            self.registers[read_addr].get()
        } else {
            0
        }
    }
}

#[typetag::serde()]
impl Component for RegFile {
    fn to_(&self) {
        println!("RegFile");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.read_addr1.clone(), self.read_addr2.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function; 2],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {
        if simulator.get_input_val(&self.write_enable) == true as Signal {
            let data = simulator.get_input_val(&self.write_data);
            println!("data {}", data);
            let write_addr = simulator.get_input_val(&self.write_addr) as usize;
            println!("write_addr {}", write_addr);
            self.registers[write_addr].set(data);
        }

        let base = simulator.get_id_start_index(&self.id);
        println!("base {}", base);
        println!("sim_state {:?}", simulator.sim_state);

        let reg_value = self.read_reg(simulator, &self.read_addr1);
        println!("reg_value {}", reg_value);
        simulator.set(base, reg_value);

        let reg_value = self.read_reg(simulator, &self.read_addr2);
        println!("reg_value {}", reg_value);
        simulator.set(base + 1, reg_value);
    }

    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create RegFile View");
        View::build(RegFileView {}, cx, |cx| {
            Label::new(cx, "Register File")
                .left(Pixels(10.0))
                .top(Pixels(10.0));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - self.width / 2.0))
        .top(Pixels(self.pos.1 - self.height / 2.0))
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct RegFileView {}

impl View for RegFileView {
    fn element(&self) -> Option<&'static str> {
        Some("InstMem")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("InstMem draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(Color::rgbf(0.0, 1.0, 1.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.fill_path(&path, &paint);
    }
}
