use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

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
                outputs: vec!["instruction".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        // get instr at pc/4
        let pc:u32 = simulator.get_input_val(&self.pc).try_into().unwrap();

        println!("--- evaluate instr mem: pc {}", pc);
        let instr = self.instr[(pc / 4) as usize];
        // set output
        println!("--- output {}", instr);
        simulator.set_out_val(&self.id, "instruction", instr);
    }
}
