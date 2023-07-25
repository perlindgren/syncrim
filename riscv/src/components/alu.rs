use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct ALU {
    pub id: String,
    pub pos: (f32, f32),

    pub operator_i: Input,
    pub operand_a_i: Input,
    pub operand_b_i: Input,
    //pub operand_c_i: Input,
}

#[typetag::serde()]
impl Component for ALU {
    fn to_(&self) {
        println!("ALU");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![
                    self.operator_i.clone(),
                    self.operand_a_i.clone(),
                    self.operand_b_i.clone(),
                    //self.operand_c_i.clone(),
                ],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "result_o".into(),
                    //"comparison_result_o".into(),
                    //"ready_o".into(),
                ],
            },
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) {
        let operator_i: u32 = simulator
            .get_input_val(&self.operator_i)
            .try_into()
            .unwrap();
        let operand_a_i: u32 = simulator
            .get_input_val(&self.operand_a_i)
            .try_into()
            .unwrap();
        let operand_b_i: u32 = simulator
            .get_input_val(&self.operand_b_i)
            .try_into()
            .unwrap();
        //let operand_c_i = simulator.get_input_val(&self.operand_c_i);
        let mut result_o = 0;
        match operator_i {
            1 => result_o = (operand_a_i as i32 + operand_b_i as i32) as u32, //ADD/ADDI
            2 => result_o = (operand_a_i as i32 - operand_b_i as i32) as u32, //SUB
            3 => result_o = operand_a_i << operand_b_i,                       //SLL/SLLI
            4 => result_o = operand_a_i >> operand_b_i,                       //SRL/SRLI
            5 => result_o = (operand_a_i as i32 >> operand_b_i as i32) as u32, //SRA/SRAI
            6 => result_o = operand_a_i ^ operand_b_i,                        //XOR/XORI
            7 => result_o = operand_a_i | operand_b_i,                        //OR/ORI
            8 => result_o = operand_a_i & operand_b_i,                        //AND/ANDI
            9 => result_o = (operand_a_i < operand_b_i) as u32,               //SLTU/SLTIU/BLTU
            10 => result_o = ((operand_a_i as i32) < (operand_b_i as i32)) as u32, //SLT/SLTI/BLT
            11 => result_o = (operand_a_i == operand_b_i) as u32,             //BEQ
            12 => result_o = (operand_a_i != operand_b_i) as u32,             //BNE
            13 => result_o = (operand_a_i as i32 >= operand_b_i as i32) as u32, //BGE
            14 => result_o = (operand_a_i >= operand_b_i) as u32,             //BGEU
            _ => {}
        }
        //println!("ALU result_o:{}", result_o);
        simulator.set_out_val(&self.id, "result_o", result_o);
    }
}
