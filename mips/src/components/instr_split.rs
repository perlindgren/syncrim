use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct InstrSplit {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub instr: Input,
}

#[typetag::serde()]
impl Component for InstrSplit {
    fn to_(&self) {
        println!("InstrSplit");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.instr.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    Output::Function, // 0 op
                    Output::Function, // 1 rs
                    Output::Function, // 2 rt
                    Output::Function, // 3 rd
                    Output::Function, // 4 imm16
                    Output::Function, // 5 imm26
                ],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {
        let instr = simulator.get_input_val(&self.instr);

        simulator.set_id_index(&self.id, 0, instr >> 26);
        simulator.set_id_index(&self.id, 1, (instr >> 21) & 0b11111);
        simulator.set_id_index(&self.id, 2, (instr >> 16) & 0b11111);
        simulator.set_id_index(&self.id, 3, (instr >> 11) & 0b11111);
        simulator.set_id_index(&self.id, 4, instr & ((1 << 16) - 1));
        simulator.set_id_index(&self.id, 5, instr & ((1 << 26) - 1));
    }
}

#[cfg(test)]
mod test_instr_split {
    use super::*;
    use std::rc::Rc;
    use syncrim::{common::ComponentStore, components::ProbeOut};

    #[test]
    fn test_split() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instr")),
                Rc::new(InstrSplit {
                    id: "split".to_string(),
                    pos: (0.0, 0.0),
                    width: 0.0,
                    height: 0.0,

                    instr: Input::new("instr", 0),
                }),
            ],
        };

        let mut clock = 0;
        let mut simulator = Simulator::new(&cs, &mut clock);

        assert_eq!(clock, 1);

        // outputs
        let op = &Input::new("split", 0);
        let rs = &Input::new("split", 1);
        let rt = &Input::new("split", 2);
        let rd = &Input::new("split", 3);
        let imm16 = &Input::new("split", 4);
        let imm26 = &Input::new("split", 5);

        // reset
        assert_eq!(simulator.get_input_val(op), 0);
        assert_eq!(simulator.get_input_val(rs), 0);
        assert_eq!(simulator.get_input_val(rt), 0);
        assert_eq!(simulator.get_input_val(rd), 0);
        assert_eq!(simulator.get_input_val(imm16), 0);
        assert_eq!(simulator.get_input_val(imm26), 0);

        println!("<setup for split high>");

        simulator.set_id_index("instr", 0, 0b100000_10000_10000_10000_00000000000);

        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock(&mut clock);
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(clock, 2);
        assert_eq!(simulator.get_input_val(op), 0b100000);
        assert_eq!(simulator.get_input_val(rs), 0b10000);
        assert_eq!(simulator.get_input_val(rt), 0b10000);
        assert_eq!(simulator.get_input_val(rd), 0b10000);
        assert_eq!(simulator.get_input_val(imm16), 0b10000_00000000000);
        assert_eq!(
            simulator.get_input_val(imm26),
            0b10000_10000_10000_00000000000
        );

        println!("<setup for split low>");

        simulator.set_id_index("instr", 0, 0b000001_00001_00001_00001_00000000001);

        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock(&mut clock);
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(clock, 3);
        assert_eq!(simulator.get_input_val(op), 0b000001);
        assert_eq!(simulator.get_input_val(rs), 0b00001);
        assert_eq!(simulator.get_input_val(rt), 0b00001);
        assert_eq!(simulator.get_input_val(rd), 0b00001);
        assert_eq!(simulator.get_input_val(imm16), 0b00001_00000000001);
        assert_eq!(
            simulator.get_input_val(imm26),
            0b00001_00001_00001_00000000001
        );

        println!("<setup for split mix>");

        simulator.set_id_index("instr", 0, 0b000000_00001_00010_00011_00000000100);

        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock(&mut clock);
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(clock, 4);
        assert_eq!(simulator.get_input_val(op), 0b000000);
        assert_eq!(simulator.get_input_val(rs), 0b00001);
        assert_eq!(simulator.get_input_val(rt), 0b00010);
        assert_eq!(simulator.get_input_val(rd), 0b00011);
        assert_eq!(simulator.get_input_val(imm16), 0b00011_00000000100);
        assert_eq!(
            simulator.get_input_val(imm26),
            0b00001_00010_00011_00000000100
        );
    }

    #[test]
    fn test_split_r() {}

    #[test]
    fn test_split_i() {}

    #[test]
    fn test_split_j() {}
}
