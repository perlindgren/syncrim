use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,
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

    fn evaluate(&self, simulator: &mut Simulator) {
        // get instr at pc/4
        let pc = simulator.get_input_val(&self.pc);

        println!("--- evaluate instr mem: pc {}", pc);
        let instr = self.instr[(pc / 4) as usize];
        // set output
        println!("--- output {}", instr);
        simulator.set_id_index(&self.id, 0, instr);
    }
}
