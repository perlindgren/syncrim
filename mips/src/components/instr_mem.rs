use serde::{Deserialize, Serialize};
use syncrim::{
    common::{Component, Input, Output, OutputType, Ports, SimState, Simulator},
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub instr: Vec<u32>,
    pub pc: Input,
}

#[typetag::serde()]
impl Component for InstrMem {
    fn to_(&self) {
        println!("InstrMem");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.pc.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get instr at pc/4
        let pc = simulator.get_input_val(sim_state, &self.pc);

        println!("--- evaluate instr mem: pc {}", pc);
        let instr = self.instr[(pc / 4) as usize];
        // set output
        println!("--- output {}", instr);
        simulator.set_id_index(sim_state, &self.id, 0, instr);
    }

    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create InsrMem View");
        View::build(
            InstMem {
                // simulator,
                // select: self.select.clone(),
            },
            cx,
            |cx| {
                Label::new(cx, "Inst Mem")
                    .left(Percentage(20.0))
                    .top(Percentage(45.0));
            },
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 50.0))
        .top(Pixels(self.pos.1 - 100.0))
        .width(Pixels(100.0))
        .height(Pixels(200.0));
    }
}

pub struct InstMem {
    //simulator: Rc<Simulator>,
    //select: Input,
}

impl View for InstMem {
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
