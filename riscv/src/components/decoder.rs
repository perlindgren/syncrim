use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Condition, Input, OutputType, Ports, SignalValue, Simulator};
use syncrim::components::MemCtrl;

#[derive(Serialize, Deserialize)]
pub struct Decoder {
    pub id: String,
    pub pos: (f32, f32),

    pub instruction: Input,
}

#[typetag::serde()]
impl Component for Decoder {
    fn to_(&self) {
        println!("Decoder");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.instruction.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "wb_mux".into(),
                    "alu_operand_a_sel".into(),
                    "alu_operand_b_sel".into(),
                    "alu_operator".into(),
                    "regfile_rd".into(),
                    "regfile_rs1".into(),
                    "regfile_rs2".into(),
                    "regfile_we".into(),
                    "sign_zero_ext_sel".into(),
                    "sign_zero_ext_data".into(),
                    "imm_a_mux_data".into(),
                    //"pc_se_data".into(),
                    //"pc_mux_sel".into(),
                    "data_mem_size".into(),
                    "data_se".into(),
                    "data_mem_ctrl".into(),
                    "pc_imm_sel".into(),
                    "big_imm".into(),
                    "branch_imm".into(),
                    "branch_logic_ctl".into(),
                    "branch_logic_enable".into(),
                    "jalr_imm".into(),
                ],
            },
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let instruction: u32 = simulator
            .get_input_value(&self.instruction)
            .try_into()
            .unwrap();
        let opcode = instruction & 0b1111111;
        let funct3 = (instruction & (0b111 << 12)) >> 12;
        let funct7 = (instruction & (0b1111111 << 25)) >> 25;
        let imm = instruction >> 20;
        let shamt = (instruction & (0b11111 << 20)) >> 20;
        let imm_big = instruction & 0xFFFFF000;
        let imm_big_shuffled = (((instruction & (0b1 << 31)) >> (31 - 20))
            | ((instruction & (0b1111111111 << 21)) >> (30 - 10))
            | ((instruction & (0b1 << 20)) >> (20 - 11))
            | (instruction & (0b11111111 << 12)))
            & 0b1111_1111_1111_1111_1111_1111_1111_1110;
        //no idea why this is encoded this way but the ISA is what it is
        let imm_store =
            ((instruction & (0b11111 << 7)) >> 7) | ((instruction & (0b1111111 << 25)) >> 20);

        let mut wb_mux = SignalValue::Uninitialized;
        let mut alu_operand_a_sel = SignalValue::Uninitialized;
        let mut alu_operand_b_sel = SignalValue::Uninitialized;
        let mut regfile_rd = SignalValue::Uninitialized;
        let mut regfile_rs1 = SignalValue::Uninitialized;
        let mut regfile_rs2 = SignalValue::Uninitialized;
        let mut regfile_we = SignalValue::from(0); //this must be 0
        let mut alu_operator = SignalValue::Uninitialized;
        let mut sign_zero_ext_sel = SignalValue::Uninitialized;
        let mut sign_zero_ext_data = SignalValue::Uninitialized;
        let mut imm_a_mux_data = SignalValue::Uninitialized;
        let mut data_mem_size = SignalValue::Uninitialized;
        let mut data_se = SignalValue::Uninitialized;
        let mut data_mem_ctrl = SignalValue::from(MemCtrl::None as u32);
        let mut big_imm = SignalValue::Uninitialized;
        let mut pc_imm_sel = SignalValue::Uninitialized;
        let mut branch_imm = SignalValue::Uninitialized;
        let mut branch_logic_ctl = SignalValue::Uninitialized;
        let mut branch_logic_enable = SignalValue::from(0); //this must be 0
        let mut jalr_imm = SignalValue::Uninitialized;
        match opcode {
            0b0110011 => {
                //OP
                alu_operand_a_sel = SignalValue::from(0); //rs1
                alu_operand_b_sel = SignalValue::from(0); //rs2
                                                          //rs1 [19:15] rs2 [24:20] rd [11:7]
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                regfile_rs2 = SignalValue::from((instruction & (0b11111 << 20)) >> 20);
                regfile_we = SignalValue::from(1); //enable write
                wb_mux = SignalValue::from(0); //ALU source
                trace!("opcode=OP");
                match funct3 {
                    0b000 => {
                        // add/sub
                        match funct7 {
                            0b0000000 => {
                                alu_operator = SignalValue::from(1);
                            } //add
                            0b0100000 => {
                                alu_operator = SignalValue::from(2);
                            } //sub
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b001 => {
                        match funct7 {
                            // sll
                            0b0000000 => {
                                alu_operator = SignalValue::from(3);
                            } //sll
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b010 => {
                        match funct7 {
                            // slt
                            0b0000000 => {
                                alu_operator = SignalValue::from(10);
                            } //slt
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b011 => {
                        match funct7 {
                            // sltu
                            0b0000000 => {
                                alu_operator = SignalValue::from(9);
                            } //sltu
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b100 => {
                        match funct7 {
                            // xor
                            0b0000000 => {
                                alu_operator = SignalValue::from(6);
                            } //xor
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b101 => {
                        match funct7 {
                            // srl
                            0b0000000 => {
                                alu_operator = SignalValue::from(4);
                            } //srl
                            0b0100000 => {
                                alu_operator = SignalValue::from(5);
                            } //sra
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b110 => {
                        match funct7 {
                            // or
                            0b0000000 => {
                                alu_operator = SignalValue::from(7);
                            } //or
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b111 => {
                        //and
                        match funct7 {
                            0b0000000 => {
                                alu_operator = SignalValue::from(8);
                            } //and
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    _ => {
                        panic!("Invalid funct3 {:b}", funct3)
                    }
                }
            }
            0b0010011 => {
                //OP_IMM
                alu_operand_a_sel = SignalValue::from(0); //rs1
                alu_operand_b_sel = SignalValue::from(1); //imm
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                regfile_we = SignalValue::from(1); //enable write
                wb_mux = SignalValue::from(0); //ALU source
                trace!("opcode=OP_IMM");
                match funct3 {
                    0b000 => {
                        //ADDI
                        alu_operator = SignalValue::from(1);
                        sign_zero_ext_sel = SignalValue::from(0);
                        sign_zero_ext_data = SignalValue::from(imm);
                    }
                    0b010 => {
                        //SLTI
                        alu_operator = SignalValue::from(10);
                        sign_zero_ext_sel = SignalValue::from(0);
                        sign_zero_ext_data = SignalValue::from(imm);
                    }
                    0b011 => {
                        //SLTIU
                        alu_operator = SignalValue::from(9);
                        sign_zero_ext_sel = SignalValue::from(1);
                        sign_zero_ext_data = SignalValue::from(imm);
                    }
                    0b100 => {
                        //XORI
                        alu_operator = SignalValue::from(6);
                        sign_zero_ext_sel = SignalValue::from(1);
                        sign_zero_ext_data = SignalValue::from(imm);
                    }
                    0b110 => {
                        //ORI
                        alu_operator = SignalValue::from(7);
                        sign_zero_ext_sel = SignalValue::from(1);
                        sign_zero_ext_data = SignalValue::from(imm);
                    }
                    0b111 => {
                        //ANDI
                        alu_operator = SignalValue::from(8);
                        sign_zero_ext_sel = SignalValue::from(1);
                        sign_zero_ext_data = SignalValue::from(imm);
                    }
                    0b001 => {
                        //SLLI
                        alu_operator = SignalValue::from(3);
                        sign_zero_ext_sel = SignalValue::from(1);
                        sign_zero_ext_data = SignalValue::from(shamt);
                    }
                    0b101 => {
                        //SRLI SRAI
                        match funct7 {
                            0b0000000 => {
                                alu_operator = SignalValue::from(4);
                                sign_zero_ext_sel = SignalValue::from(1);
                                sign_zero_ext_data = SignalValue::from(shamt);
                            } //SRLI
                            0b0100000 => {
                                alu_operator = SignalValue::from(5);
                                sign_zero_ext_sel = SignalValue::from(1);
                                sign_zero_ext_data = SignalValue::from(shamt);
                            } //SRAI
                            _ => panic!("Invalid funct7! {:b}", funct7),
                        }
                    }
                    _ => {
                        panic!("Invalid funct3! {:b}", funct3)
                    }
                }
            }
            0b0110111 => {
                //LUI
                trace!("opcode=LUI");
                alu_operand_a_sel = SignalValue::from(1); //big-imm
                alu_operand_b_sel = SignalValue::from(1); //imm
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = 0; //x0 dont care
                regfile_we = SignalValue::from(1); //enable write
                wb_mux = SignalValue::from(0); //ALU source
                alu_operator = SignalValue::from(1); //ADD
                sign_zero_ext_data = SignalValue::from(0); //add 0
                sign_zero_ext_sel = SignalValue::from(1); //zero-extend
                imm_a_mux_data = SignalValue::from(imm_big);
            }
            0b0010111 => {
                //AUIPC
                trace!("opcode=AUIPC");
                alu_operand_a_sel = SignalValue::from(1); //big-imm
                alu_operand_b_sel = SignalValue::from(3); //PC
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = SignalValue::from(0); //x0 dont care
                regfile_we = SignalValue::from(1); //enable write
                wb_mux = SignalValue::from(0); //ALU source
                alu_operator = SignalValue::from(1); //ADD
                                                     //sign_zero_ext_data = SignalValue::from(0); //don't care
                                                     //sign_zero_ext_sel = SignalValue::from(1); //don't care
                imm_a_mux_data = SignalValue::from(imm_big);
            }
            0b1101111 => {
                //JAL
                trace!("opcode=JAL");
                alu_operand_a_sel = SignalValue::from(2); //0
                alu_operand_b_sel = SignalValue::from(2); //PC
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = SignalValue::from(0); //dont care
                regfile_we = SignalValue::from(1); //enable write
                wb_mux = SignalValue::from(0); //ALU source
                alu_operator = SignalValue::from(1); //ADD
                                                     //sign_zero_ext_data = 0; //don't care
                                                     //sign_zero_ext_sel = 1; //don't care
                big_imm = SignalValue::from(imm_big_shuffled);
                pc_imm_sel = SignalValue::from(0);
                branch_logic_ctl = SignalValue::from(0b010); //jal
                branch_logic_enable = SignalValue::from(0b1);
            }
            0b1100111 => {
                //JALR
                trace!("opcode=JALR");
                alu_operand_a_sel = SignalValue::from(2); //0
                alu_operand_b_sel = SignalValue::from(2); //PC
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                regfile_we = SignalValue::from(1); //enable write
                wb_mux = SignalValue::from(0); //ALU source
                alu_operator = SignalValue::from(1); //ADD
                                                     //sign_zero_ext_data = 0; //don't care
                                                     //sign_zero_ext_sel = 1; //don't care
                                                     //big_imm = imm_big_shuffled; //don't care
                                                     //pc_imm_sel = 0; //don't care
                branch_logic_ctl = SignalValue::from(0b011); //jalr
                branch_logic_enable = SignalValue::from(0b1);
                jalr_imm = SignalValue::from(imm);
            }
            0b1100011 => {
                //BRANCH
                trace!("opcode=BRANCH");
                regfile_rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                regfile_rs2 = SignalValue::from((instruction & (0b11111 << 20)) >> 20);
                //pc_imm_sel = 1;
                //branch_imm = imm;
                //regfile_rd = 0; //don't care
                //regfile_we = 0; //no link
                //wb_mux = 0; //don't care
                //alu_operator = 0; //don't care
                //sign_zero_ext_data = 0; //don't care
                //big_imm = 0; //don't care
                pc_imm_sel = SignalValue::from(1); //branch imm
                branch_logic_ctl = SignalValue::from(funct3); //use funct3
                branch_logic_enable = SignalValue::from(0b1); //enable branch logic
                branch_imm = (((instruction & (0b1 << 31)) >> 19)
                    | ((instruction & (0b111111 << 25)) >> 20)
                    | ((instruction & (0b1111 << 8)) >> 7)
                    | ((instruction & (0b1 << 7)) << 4))
                    .into();
            }

            0b0000011 => {
                //LOAD
                trace!("opcode=LOAD");
                alu_operand_a_sel = SignalValue::from(0); //rs1
                alu_operand_b_sel = SignalValue::from(1); //imm
                regfile_rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                regfile_we = SignalValue::from(1);
                wb_mux = SignalValue::from(1); //data memory
                alu_operator = SignalValue::from(1); //ADD
                sign_zero_ext_data = SignalValue::from(imm); //immediate
                sign_zero_ext_sel = SignalValue::from(0); //sign extend

                data_mem_ctrl = SignalValue::from(MemCtrl::Read as u32);
                match funct3 {
                    0b000 => {
                        data_mem_size = SignalValue::from(1);
                        data_se = SignalValue::from(1)
                    } //lb
                    0b001 => {
                        data_mem_size = SignalValue::from(2);
                        data_se = SignalValue::from(1)
                    } //lh
                    0b010 => {
                        data_mem_size = SignalValue::from(4);
                        data_se = SignalValue::from(1)
                    } //lw
                    0b100 => {
                        data_mem_size = SignalValue::from(1);
                        data_se = SignalValue::from(0)
                    } //lbu
                    0b101 => {
                        data_mem_size = SignalValue::from(2);
                        data_se = SignalValue::from(0)
                    } //lhu
                    _ => {
                        panic!("Unsupported funct3 {:b}", funct3)
                    }
                }
            }
            0b0100011 => {
                //STORE
                trace!("opcode=STORE");
                alu_operand_a_sel = SignalValue::from(0); //rs1
                alu_operand_b_sel = SignalValue::from(1); //imm
                regfile_rd = SignalValue::Uninitialized;
                regfile_rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                regfile_rs2 = SignalValue::from((instruction & (0b11111 << 20)) >> 20);
                regfile_we = SignalValue::from(0);
                //wb_mux = 0; //don't care
                alu_operator = SignalValue::from(1); //ADD
                sign_zero_ext_data = SignalValue::from(imm_store); //immediate store type
                sign_zero_ext_sel = SignalValue::from(0); //sign extend

                data_mem_ctrl = SignalValue::from(MemCtrl::Write as u32);
                match funct3 {
                    //size
                    0b000 => {
                        data_mem_size = SignalValue::from(1);
                    }
                    0b001 => {
                        data_mem_size = SignalValue::from(2);
                    }
                    0b010 => {
                        data_mem_size = SignalValue::from(4);
                    }
                    _ => panic!("Unsupported funct3 {:b}", funct3),
                }
            }
            0b0 => {}
            _ => {
                panic!("Invalid opcode! {:b}", opcode);
            }
        };

        simulator.set_out_value(&self.id, "wb_mux", wb_mux);
        simulator.set_out_value(&self.id, "alu_operand_a_sel", alu_operand_a_sel);
        simulator.set_out_value(&self.id, "alu_operand_b_sel", alu_operand_b_sel);
        simulator.set_out_value(&self.id, "regfile_rs1", regfile_rs1);
        simulator.set_out_value(&self.id, "regfile_rs2", regfile_rs2);
        simulator.set_out_value(&self.id, "regfile_rd", regfile_rd);
        simulator.set_out_value(&self.id, "regfile_we", regfile_we);
        simulator.set_out_value(&self.id, "alu_operator", alu_operator);
        simulator.set_out_value(&self.id, "sign_zero_ext_sel", sign_zero_ext_sel);
        simulator.set_out_value(&self.id, "sign_zero_ext_data", sign_zero_ext_data);
        simulator.set_out_value(&self.id, "imm_a_mux_data", imm_a_mux_data);
        simulator.set_out_value(&self.id, "data_mem_size", data_mem_size);
        simulator.set_out_value(&self.id, "data_se", data_se);
        simulator.set_out_value(&self.id, "data_mem_ctrl", data_mem_ctrl);
        simulator.set_out_value(&self.id, "big_imm", big_imm);
        simulator.set_out_value(&self.id, "pc_imm_sel", pc_imm_sel);
        simulator.set_out_value(&self.id, "branch_imm", branch_imm);
        simulator.set_out_value(&self.id, "branch_logic_ctl", branch_logic_ctl);
        simulator.set_out_value(&self.id, "branch_logic_enable", branch_logic_enable);
        simulator.set_out_value(&self.id, "jalr_imm", jalr_imm);
        Ok(())
    }
}

mod test {
    #![allow(unused_imports)]
    use super::*;
    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };

    #[test]
    fn test_op() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(&cs);

        // outputs
        let wb_mux = &Input::new("decoder", "wb_mux");
        let alu_operand_a_sel = &Input::new("decoder", "alu_operand_a_sel");
        let alu_operand_b_sel = &Input::new("decoder", "alu_operand_b_sel");
        let regfile_rs1 = &Input::new("decoder", "regfile_rs1");
        let regfile_rs2 = &Input::new("decoder", "regfile_rs2");
        let regfile_rd = &Input::new("decoder", "regfile_rd");
        let regfile_we = &Input::new("decoder", "regfile_we");
        let alu_operator = &Input::new("decoder", "alu_operator");
        let sign_zero_ext_sel = &Input::new("decoder", "sign_zero_ext_sel");
        let sign_zero_ext_data = &Input::new("decoder", "sign_zero_ext_data");
        let imm_a_mux_data = &Input::new("decoder", "imm_a_mux_data");
        let data_mem_size = &Input::new("decoder", "data_mem_size");
        let data_se = &Input::new("decoder", "data_se");
        let data_mem_ctrl = &Input::new("decoder", "data_mem_ctrl");
        let big_imm = &Input::new("decoder", "big_imm");
        let pc_imm_sel = &Input::new("decoder", "pc_imm_sel");
        let branch_imm = &Input::new("decoder", "branch_imm");
        let branch_logic_ctl = &Input::new("decoder", "branch_logic_ctl");
        let branch_logic_enable = &Input::new("decoder", "branch_logic_enable");
        let jalr_imm = &Input::new("decoder", "jalr_imm");

        simulator.set_out_value("instruction", "out", 0x003100b3); //add x1, x2, x3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x40410133); //sub x2, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 2.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x004121b3); //slt x3, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 10.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x004131b3); //sltu x3, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 9.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x001151b3); //srl x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 4.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x401151b3); //sra x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 5.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x001111b3); //sll x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 3.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0020c1b3); //xor x3, x1, x2
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 6.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0020f1b3); //and x3, x1, x2)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 8.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0060e1b3); //or x3, x1, x6
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 6.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 7.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );
    }
    #[test]
    fn test_op_imm() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(&cs);

        // outputs
        let wb_mux = &Input::new("decoder", "wb_mux");
        let alu_operand_a_sel = &Input::new("decoder", "alu_operand_a_sel");
        let alu_operand_b_sel = &Input::new("decoder", "alu_operand_b_sel");
        let regfile_rs1 = &Input::new("decoder", "regfile_rs1");
        let regfile_rs2 = &Input::new("decoder", "regfile_rs2");
        let regfile_rd = &Input::new("decoder", "regfile_rd");
        let regfile_we = &Input::new("decoder", "regfile_we");
        let alu_operator = &Input::new("decoder", "alu_operator");
        let sign_zero_ext_sel = &Input::new("decoder", "sign_zero_ext_sel");
        let sign_zero_ext_data = &Input::new("decoder", "sign_zero_ext_data");
        let imm_a_mux_data = &Input::new("decoder", "imm_a_mux_data");
        let data_mem_size = &Input::new("decoder", "data_mem_size");
        let data_se = &Input::new("decoder", "data_se");
        let data_mem_ctrl = &Input::new("decoder", "data_mem_ctrl");
        let big_imm = &Input::new("decoder", "big_imm");
        let pc_imm_sel = &Input::new("decoder", "pc_imm_sel");
        let branch_imm = &Input::new("decoder", "branch_imm");
        let branch_logic_ctl = &Input::new("decoder", "branch_logic_ctl");
        let branch_logic_enable = &Input::new("decoder", "branch_logic_enable");
        let jalr_imm = &Input::new("decoder", "jalr_imm");

        simulator.set_out_value("instruction", "out", 0x00310093); //addi x1, x2, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xffd0a093); //slti x1, x1, -3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 10.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            ((-3i32 as u32) & 0b111111111111).into()
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xffd0b093); //sltiu x1, x1, -3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 9.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            ((-3i32 as u32) & 0b111111111111).into()
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00324093); //xori x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 4.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 6.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00326093); //ori x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 4.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 7.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00327093); //andi x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 4.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 8.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00c19093); //slli x1, x3, 12
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 3.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 3.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 12.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0011d093); //srli x1, x3, 1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 3.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 4.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 1.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x4020d093); //srai x1, x1, 2
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 5.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 2.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );
    }
    #[test]
    fn test_lui_auipc_store_load() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(&cs);

        // outputs
        let wb_mux = &Input::new("decoder", "wb_mux");
        let alu_operand_a_sel = &Input::new("decoder", "alu_operand_a_sel");
        let alu_operand_b_sel = &Input::new("decoder", "alu_operand_b_sel");
        let regfile_rs1 = &Input::new("decoder", "regfile_rs1");
        let regfile_rs2 = &Input::new("decoder", "regfile_rs2");
        let regfile_rd = &Input::new("decoder", "regfile_rd");
        let regfile_we = &Input::new("decoder", "regfile_we");
        let alu_operator = &Input::new("decoder", "alu_operator");
        let sign_zero_ext_sel = &Input::new("decoder", "sign_zero_ext_sel");
        let sign_zero_ext_data = &Input::new("decoder", "sign_zero_ext_data");
        let imm_a_mux_data = &Input::new("decoder", "imm_a_mux_data");
        let data_mem_size = &Input::new("decoder", "data_mem_size");
        let data_se = &Input::new("decoder", "data_se");
        let data_mem_ctrl = &Input::new("decoder", "data_mem_ctrl");
        let big_imm = &Input::new("decoder", "big_imm");
        let pc_imm_sel = &Input::new("decoder", "pc_imm_sel");
        let branch_imm = &Input::new("decoder", "branch_imm");
        let branch_logic_ctl = &Input::new("decoder", "branch_logic_ctl");
        let branch_logic_enable = &Input::new("decoder", "branch_logic_enable");
        let jalr_imm = &Input::new("decoder", "jalr_imm");

        simulator.set_out_value("instruction", "out", 0xfffff0b7); //lui x1, 0xFFFFF
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs1),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 0.into());
        assert_eq!(simulator.get_input_value(imm_a_mux_data), 0xFFFFF000.into());
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xfffff097); //auipc x1, 0xFFFFF
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 3.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs1),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(imm_a_mux_data), 0xFFFFF000.into());
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0082a223); //sw x8, 4(x5)
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 8.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 4.into());
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00829223); //sh x8, 4(x5)
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 8.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 2.into());
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00828223); //sb x8, 4(x5)
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 8.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 1.into());
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0042a403); //lw x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 4.into());
        assert_eq!(simulator.get_input_value(data_se), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00429403); //lh x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_value(data_se), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00428403); //lb x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_value(data_se), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0042d403); //lhu x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_value(data_se), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x0042c403); //lbu x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_value(data_se), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_logic_ctl),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );
    }

    #[test]
    fn test_jal_jalr_branch() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(&cs);

        // outputs
        let wb_mux = &Input::new("decoder", "wb_mux");
        let alu_operand_a_sel = &Input::new("decoder", "alu_operand_a_sel");
        let alu_operand_b_sel = &Input::new("decoder", "alu_operand_b_sel");
        let regfile_rs1 = &Input::new("decoder", "regfile_rs1");
        let regfile_rs2 = &Input::new("decoder", "regfile_rs2");
        let regfile_rd = &Input::new("decoder", "regfile_rd");
        let regfile_we = &Input::new("decoder", "regfile_we");
        let alu_operator = &Input::new("decoder", "alu_operator");
        let sign_zero_ext_sel = &Input::new("decoder", "sign_zero_ext_sel");
        let sign_zero_ext_data = &Input::new("decoder", "sign_zero_ext_data");
        let imm_a_mux_data = &Input::new("decoder", "imm_a_mux_data");
        let data_mem_size = &Input::new("decoder", "data_mem_size");
        let data_se = &Input::new("decoder", "data_se");
        let data_mem_ctrl = &Input::new("decoder", "data_mem_ctrl");
        let big_imm = &Input::new("decoder", "big_imm");
        let pc_imm_sel = &Input::new("decoder", "pc_imm_sel");
        let branch_imm = &Input::new("decoder", "branch_imm");
        let branch_logic_ctl = &Input::new("decoder", "branch_logic_ctl");
        let branch_logic_enable = &Input::new("decoder", "branch_logic_enable");
        let jalr_imm = &Input::new("decoder", "jalr_imm");

        simulator.set_out_value("instruction", "out", 0x0080016f); //jal x2, 8
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 2.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs1),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(big_imm), 8.into());
        assert_eq!(simulator.get_input_value(pc_imm_sel), 0.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 2.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00410167); //jalr x2, x2, 4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 2.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rs2),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rd), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(pc_imm_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 3.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(simulator.get_input_value(jalr_imm), 4.into());

        simulator.set_out_value("instruction", "out", 0xfe209ee3); //bne x1, x2, -4
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_a_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_b_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(alu_operator),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(pc_imm_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((-4i32 & 0b1111111111111) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 1.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0x00208463); //beq, x1, x2, 8
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_a_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_b_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(alu_operator),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(pc_imm_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((8i32 & 0b1111111111111) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xfe20cee3); //blt x1, x2, -4
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_a_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_b_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(alu_operator),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(pc_imm_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((-4i32 & 0b1111111111111) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b100.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xfe116ee3); //bltu, x2, x1, -4
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_a_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_b_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(alu_operator),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(pc_imm_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((-4i32 & 0b1111111111111) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b110.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xfe115ee3); //bge x2, x1, -4
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_a_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_b_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(alu_operator),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(pc_imm_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((-4i32 & 0b1111111111111) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b101.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );

        simulator.set_out_value("instruction", "out", 0xfe20fee3); //bgeu x1, x2, -4
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(wb_mux),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_a_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(alu_operand_b_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(
            simulator.get_input_value(regfile_rd),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(alu_operator),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_sel),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(imm_a_mux_data),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_size),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_se),
            SignalValue::Uninitialized
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(big_imm),
            SignalValue::Uninitialized
        );
        assert_eq!(simulator.get_input_value(pc_imm_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((-4i32 & 0b1111111111111) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b111.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(
            simulator.get_input_value(jalr_imm),
            SignalValue::Uninitialized
        );
    }
}

// 0x0080016f, //jal x2, 8
// 0x00410167, //jalr x2, x2, 4
// 0xfe209ee3, //bne x1, x2, -4
// 0x00208463, //beq, x1, x2, 8
// 0xfe20cee3, //blt x1, x2, -4
// 0xfe116ee3, //bltu, x2, x1, -4
// 0xfe115ee3, //bge x2, x1, -4
// 0xfe20fee3, //bgeu x1, x2, -4
