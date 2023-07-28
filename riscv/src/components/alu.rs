use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Signal, Simulator};

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
        let operator_i: u32;
        match simulator.get_input_val(&self.operator_i) {
            Signal::Data(data) => operator_i = data,
            _ => {
                simulator.set_out_val(&self.id, "result_o", Signal::Unknown);
                return;
            }
        }
        //if i is set, these two must be set or panic is reasonable.
        let operand_a_i: u32 = simulator
            .get_input_val(&self.operand_a_i)
            .try_into()
            .unwrap();
        let operand_b_i: u32 = simulator
            .get_input_val(&self.operand_b_i)
            .try_into()
            .unwrap();
        trace!("ALU operand A: {}, operand B:{}", operand_a_i, operand_b_i);
        let mut result_o = 0;
        match operator_i {
            1 => {
                result_o = (operand_a_i as i32 + operand_b_i as i32) as u32;
                trace!("ALU ADD")
            } //ADD/ADDI
            2 => {
                result_o = (operand_a_i as i32 - operand_b_i as i32) as u32;
                trace!("ALU SUB");
            } //SUB
            3 => {
                result_o = operand_a_i << operand_b_i;
                trace!("ALU SHIFT LEFT LOCIVAL");
            } //SLL/SLLI
            4 => {
                result_o = operand_a_i >> operand_b_i;
                trace!("ALU SHIFT RIGHT LOGICAL");
            } //SRL/SRLI
            5 => {
                result_o = (operand_a_i as i32 >> operand_b_i as i32) as u32;
                trace!("ALU SHIFT RIGHT ARITHMETIC");
            } //SRA/SRAI
            6 => {
                result_o = operand_a_i ^ operand_b_i;
                trace!("ALU XOR");
            } //XOR/XORI
            7 => {
                result_o = operand_a_i | operand_b_i;
                trace!("ALU OR");
            } //OR/ORI
            8 => {
                result_o = operand_a_i & operand_b_i;
                trace!("ALU AND");
            } //AND/ANDI
            9 => {
                result_o = (operand_a_i < operand_b_i) as u32;
                trace!("ALU SET LESS THAN UNSIGNED");
            } //SLTU/SLTIU/BLTU
            10 => {
                result_o = ((operand_a_i as i32) < (operand_b_i as i32)) as u32;
                trace!("ALU SET LESS THAN");
            } //SLT/SLTI/BLT
            11 => {
                result_o = (operand_a_i == operand_b_i) as u32;
                trace!("ALU EQUAL");
            } //BEQ
            12 => {
                result_o = (operand_a_i != operand_b_i) as u32;
                trace!("ALU NOT EQUAL");
            } //BNE
            13 => {
                result_o = (operand_a_i as i32 >= operand_b_i as i32) as u32;
                trace!("ALU GREATER OR EQUAL");
            } //BGE
            14 => {
                result_o = (operand_a_i >= operand_b_i) as u32;
                trace!("ALU GREATER OR EQUAL UNSIGNED");
            } //BGEU
            _ => {}
        }
        trace!("ALU result_o:{}", result_o);
        simulator.set_out_val(&self.id, "result_o", result_o);
    }
}
