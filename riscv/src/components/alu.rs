use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::{
    common::{Component, Condition, Input, OutputType, Ports, SignalValue, Simulator},
    signal::SignalSigned,
};

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
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let operator_i = match simulator.get_input_value(&self.operator_i) {
            SignalValue::Data(data) => data,
            _ => {
                simulator.set_out_value(&self.id, "result_o", SignalValue::Unknown);
                return Ok(());
            }
        };
        //if i is set, these two must be set or panic is reasonable.
        let operand_a_i: u32 = simulator
            .get_input_value(&self.operand_a_i)
            .try_into()
            .unwrap();
        let operand_b_i: u32 = simulator
            .get_input_value(&self.operand_b_i)
            .try_into()
            .unwrap();
        trace!("ALU operand A: {}, operand B:{}", operand_a_i, operand_b_i);
        let mut result_o = 0;
        match operator_i {
            1 => {
                result_o = SignalSigned::overflowing_add(
                    operand_a_i as SignalSigned,
                    operand_b_i as SignalSigned,
                )
                .0 as u32;
                trace!("ALU ADD")
            } //ADD/ADDI
            2 => {
                result_o = SignalSigned::overflowing_sub(
                    operand_a_i as SignalSigned,
                    operand_b_i as SignalSigned,
                )
                .0 as u32;
                trace!("ALU SUB");
            } //SUB
            3 => {
                result_o = operand_a_i << operand_b_i;
                trace!("ALU SHIFT LEFT LOGICAL");
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
        trace!("ALU result_o:{:08x}", result_o);
        simulator.set_out_value(&self.id, "result_o", result_o);
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };
    #[allow(arithmetic_overflow)]
    #[test]
    fn test_alu() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("operator_i")),
                Rc::new(ProbeOut::new("operand_a_i")),
                Rc::new(ProbeOut::new("operand_b_i")),
                Rc::new(ALU {
                    id: "alu".to_string(),
                    pos: (0.0, 0.0),
                    operator_i: Input::new("operator_i", "out"),
                    operand_a_i: Input::new("operand_a_i", "out"),
                    operand_b_i: Input::new("operand_b_i", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let alu_out = &Input::new("alu", "result_o");

        simulator.set_out_value("operator_i", "out", 1); //add
        simulator.set_out_value("operand_a_i", "out", -41i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 42);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(alu_out),
            (SignalSigned::overflowing_add(-41i32, 42).0 as u32).into()
        );

        simulator.set_out_value("operator_i", "out", 2); //sub
        simulator.set_out_value("operand_a_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 1);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(alu_out),
            (SignalSigned::overflowing_sub(-2147483648, 1).0 as u32).into()
        );

        simulator.set_out_value("operator_i", "out", 3); //sll
        simulator.set_out_value("operand_a_i", "out", 58);
        simulator.set_out_value("operand_b_i", "out", 4);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), (58u32 << 4u32).into());

        simulator.set_out_value("operator_i", "out", 4); //srl
        simulator.set_out_value("operand_a_i", "out", 58);
        simulator.set_out_value("operand_b_i", "out", 4);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), (58u32 >> 4u32).into());

        simulator.set_out_value("operator_i", "out", 5); //sra
        simulator.set_out_value("operand_a_i", "out", -1i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 4);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(alu_out),
            ((-1i32 >> 4i32) as u32).into()
        );
        assert_ne!(
            simulator.get_input_value(alu_out),
            ((-1i32 as u32 >> 4).into())
        );

        simulator.set_out_value("operator_i", "out", 6); //xor
        simulator.set_out_value("operand_a_i", "out", 7327239 as u32);
        simulator.set_out_value("operand_b_i", "out", 184771);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(alu_out),
            (7327239 ^ 184771).into()
        );

        simulator.set_out_value("operator_i", "out", 7); //or
        simulator.set_out_value("operand_a_i", "out", 7327239 as u32);
        simulator.set_out_value("operand_b_i", "out", 184771);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(alu_out),
            (7327239 | 184771).into()
        );

        simulator.set_out_value("operator_i", "out", 8); //and
        simulator.set_out_value("operand_a_i", "out", 7327239 as u32);
        simulator.set_out_value("operand_b_i", "out", 184771);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(alu_out),
            (7327239 & 184771).into()
        );

        simulator.set_out_value("operator_i", "out", 9); //SLTU
        simulator.set_out_value("operand_a_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 1);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 0.into());
        simulator.set_out_value("operator_i", "out", 9); //SLTU
        simulator.set_out_value("operand_a_i", "out", 1 as u32);
        simulator.set_out_value("operand_b_i", "out", -2147483648i32 as u32);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());

        simulator.set_out_value("operator_i", "out", 10); //SLT
        simulator.set_out_value("operand_a_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 1);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());
        simulator.set_out_value("operator_i", "out", 10); //SLT
        simulator.set_out_value("operand_a_i", "out", 1 as u32);
        simulator.set_out_value("operand_b_i", "out", -2147483648i32 as u32);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 0.into());

        simulator.set_out_value("operator_i", "out", 11); //BEQ
        simulator.set_out_value("operand_a_i", "out", 52);
        simulator.set_out_value("operand_b_i", "out", 52);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());
        simulator.set_out_value("operator_i", "out", 11); //BEQ
        simulator.set_out_value("operand_a_i", "out", 52);
        simulator.set_out_value("operand_b_i", "out", 53);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 0.into());

        simulator.set_out_value("operator_i", "out", 12); //BNE
        simulator.set_out_value("operand_a_i", "out", 52);
        simulator.set_out_value("operand_b_i", "out", 53);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());
        simulator.set_out_value("operator_i", "out", 12); //BNE
        simulator.set_out_value("operand_a_i", "out", 53);
        simulator.set_out_value("operand_b_i", "out", 53);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 0.into());

        simulator.set_out_value("operator_i", "out", 13); //BGE
        simulator.set_out_value("operand_a_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 0.into());
        simulator.set_out_value("operator_i", "out", 13); //BGE
        simulator.set_out_value("operand_b_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_a_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());
        simulator.set_out_value("operator_i", "out", 13); //BGE
        simulator.set_out_value("operand_b_i", "out", 0);
        simulator.set_out_value("operand_a_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());

        simulator.set_out_value("operator_i", "out", 14); //BGEU
        simulator.set_out_value("operand_a_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_b_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());
        simulator.set_out_value("operator_i", "out", 14); //BGEU
        simulator.set_out_value("operand_b_i", "out", -2147483648i32 as u32);
        simulator.set_out_value("operand_a_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 0.into());
        simulator.set_out_value("operator_i", "out", 14); //BGEU
        simulator.set_out_value("operand_b_i", "out", 0);
        simulator.set_out_value("operand_a_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_out), 1.into());
    }
}
