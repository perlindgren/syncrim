use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

/// Instruction decoder for single cycle MIPS
#[derive(Serialize, Deserialize)]
pub struct InstrDecode {
    pub id: String,
    pub pos: (f32, f32),
    pub instr: Input,
}

#[typetag::serde()]
impl Component for InstrDecode {
    fn to_(&self) {
        println!("InstrMem");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.instr.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    // Currently placeholders
                    Output::Function, // 0,
                    Output::Function, // 1,
                    Output::Function, // 2,
                    Output::Function, // 32
                ],
            },
        )
    }

    fn evaluate(&self, _simulator: &mut Simulator) {
        // // get instr at pc/4
        // let pc = simulator.get_input_val(&self.instr);

        // println!("--- evaluate instr mem: pc {}", pc);
        // let instr = self.instr[(pc / 4) as usize];
        // // set output
        // println!("--- output {}", instr);
        // simulator.set_id_index(&self.id, 0, instr);
    }
}

#[test]
fn test_instr_decode() {
    println!("test_instr_decode");
}
