use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, SignalData, Simulator};
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
    fn clock(&self, simulator: &mut Simulator) {
        let instruction: u32 = simulator
            .get_input_val(&self.instruction)
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

        let mut wb_mux = SignalData::Uninitialized;
        let mut alu_operand_a_sel = SignalData::Uninitialized;
        let mut alu_operand_b_sel = SignalData::Uninitialized;
        let mut regfile_rd = SignalData::Uninitialized;
        let mut regfile_rs1 = SignalData::Uninitialized;
        let mut regfile_rs2 = SignalData::Uninitialized;
        let mut regfile_we = SignalData::from(0); //this must be 0
        let mut alu_operator = SignalData::Uninitialized;
        let mut sign_zero_ext_sel = SignalData::Uninitialized;
        let mut sign_zero_ext_data = SignalData::Uninitialized;
        let mut imm_a_mux_data = SignalData::Uninitialized;
        let mut data_mem_size = SignalData::Uninitialized;
        let mut data_se = SignalData::Uninitialized;
        let mut data_mem_ctrl = SignalData::from(MemCtrl::None as u32);
        let mut big_imm = SignalData::Uninitialized;
        let mut pc_imm_sel = SignalData::Uninitialized;
        let mut branch_imm = SignalData::Uninitialized;
        let mut branch_logic_ctl = SignalData::Uninitialized;
        let mut branch_logic_enable = SignalData::from(0); //this must be 0
        let mut jalr_imm = SignalData::Uninitialized;
        match opcode {
            0b0110011 => {
                //OP
                alu_operand_a_sel = SignalData::from(0); //rs1
                alu_operand_b_sel = SignalData::from(0); //rs2
                                                         //rs1 [19:15] rs2 [24:20] rd [11:7]
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalData::from((instruction & (0b11111 << 15)) >> 15);
                regfile_rs2 = SignalData::from((instruction & (0b11111 << 20)) >> 20);
                regfile_we = SignalData::from(1); //enable write
                wb_mux = SignalData::from(0); //ALU source
                trace!("opcode=OP");
                match funct3 {
                    0b000 => {
                        // add/sub
                        match funct7 {
                            0b0000000 => {
                                alu_operator = SignalData::from(1);
                            } //add
                            0b0100000 => {
                                alu_operator = SignalData::from(2);
                            } //sub
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b001 => {
                        match funct7 {
                            // sll
                            0b0000000 => {
                                alu_operator = SignalData::from(3);
                            } //sll
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b010 => {
                        match funct7 {
                            // slt
                            0b0000000 => {
                                alu_operator = SignalData::from(10);
                            } //slt
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b011 => {
                        match funct7 {
                            // sltu
                            0b0000000 => {
                                alu_operator = SignalData::from(9);
                            } //sltu
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b100 => {
                        match funct7 {
                            // xor
                            0b0000000 => {
                                alu_operator = SignalData::from(6);
                            } //xor
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b101 => {
                        match funct7 {
                            // srl
                            0b0000000 => {
                                alu_operator = SignalData::from(4);
                            } //srl
                            0b0100000 => {
                                alu_operator = SignalData::from(5);
                            } //sra
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b110 => {
                        match funct7 {
                            // or
                            0b0000000 => {
                                alu_operator = SignalData::from(7);
                            } //or
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b111 => {
                        //and
                        match funct7 {
                            0b0000000 => {
                                alu_operator = SignalData::from(8);
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
                alu_operand_a_sel = SignalData::from(0); //rs1
                alu_operand_b_sel = SignalData::from(1); //imm
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalData::from((instruction & (0b11111 << 15)) >> 15);
                regfile_we = SignalData::from(1); //enable write
                wb_mux = SignalData::from(0); //ALU source
                trace!("opcode=OP_IMM");
                match funct3 {
                    0b000 => {
                        //ADDI
                        alu_operator = SignalData::from(1);
                        sign_zero_ext_sel = SignalData::from(0);
                        sign_zero_ext_data = SignalData::from(imm);
                    }
                    0b010 => {
                        //SLTI
                        alu_operator = SignalData::from(10);
                        sign_zero_ext_sel = SignalData::from(0);
                        sign_zero_ext_data = SignalData::from(imm);
                    }
                    0b011 => {
                        //SLTIU
                        alu_operator = SignalData::from(9);
                        sign_zero_ext_sel = SignalData::from(1);
                        sign_zero_ext_data = SignalData::from(imm);
                    }
                    0b100 => {
                        //XORI
                        alu_operator = SignalData::from(6);
                        sign_zero_ext_sel = SignalData::from(1);
                        sign_zero_ext_data = SignalData::from(imm);
                    }
                    0b110 => {
                        //ORI
                        alu_operator = SignalData::from(7);
                        sign_zero_ext_sel = SignalData::from(1);
                        sign_zero_ext_data = SignalData::from(imm);
                    }
                    0b111 => {
                        //ANDI
                        alu_operator = SignalData::from(8);
                        sign_zero_ext_sel = SignalData::from(1);
                        sign_zero_ext_data = SignalData::from(imm);
                    }
                    0b001 => {
                        //SLLI
                        alu_operator = SignalData::from(3);
                        sign_zero_ext_sel = SignalData::from(1);
                        sign_zero_ext_data = SignalData::from(shamt);
                    }
                    0b101 => {
                        //SRLI SRAI
                        match funct7 {
                            0b0000000 => {
                                alu_operator = SignalData::from(4);
                                sign_zero_ext_sel = SignalData::from(1);
                                sign_zero_ext_data = SignalData::from(shamt);
                            } //SRLI
                            0b0100000 => {
                                alu_operator = SignalData::from(5);
                                sign_zero_ext_sel = SignalData::from(1);
                                sign_zero_ext_data = SignalData::from(shamt);
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
                alu_operand_a_sel = SignalData::from(1); //big-imm
                alu_operand_b_sel = SignalData::from(1); //imm
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = 0; //x0 dont care
                regfile_we = SignalData::from(1); //enable write
                wb_mux = SignalData::from(0); //ALU source
                alu_operator = SignalData::from(1); //ADD
                sign_zero_ext_data = SignalData::from(0); //add 0
                sign_zero_ext_sel = SignalData::from(1); //zero-extend
                imm_a_mux_data = SignalData::from(imm_big);
            }
            0b0010111 => {
                //AUIPC
                trace!("opcode=AUIPC");
                alu_operand_a_sel = SignalData::from(1); //big-imm
                alu_operand_b_sel = SignalData::from(2); //PC
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = SignalData::from(0); //x0 dont care
                regfile_we = SignalData::from(1); //enable write
                wb_mux = SignalData::from(0); //ALU source
                alu_operator = SignalData::from(1); //ADD
                                                    //sign_zero_ext_data = SignalData::from(0); //don't care
                                                    //sign_zero_ext_sel = SignalData::from(1); //don't care
                imm_a_mux_data = SignalData::from(imm_big);
            }
            0b1101111 => {
                //JAL
                trace!("opcode=JAL");
                alu_operand_a_sel = SignalData::from(2); //0
                alu_operand_b_sel = SignalData::from(2); //PC
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = SignalData::from(0); //dont care
                regfile_we = SignalData::from(1); //enable write
                wb_mux = SignalData::from(0); //ALU source
                alu_operator = SignalData::from(1); //ADD
                                                    //sign_zero_ext_data = 0; //don't care
                                                    //sign_zero_ext_sel = 1; //don't care
                big_imm = SignalData::from(imm_big_shuffled);
                pc_imm_sel = SignalData::from(0);
                branch_logic_ctl = SignalData::from(0b010); //jal
                branch_logic_enable = SignalData::from(0b1);
            }
            0b1100111 => {
                //JALR
                trace!("opcode=JALR");
                alu_operand_a_sel = SignalData::from(2); //0
                alu_operand_b_sel = SignalData::from(2); //PC
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalData::from((instruction & (0b11111 << 15)) >> 15);
                regfile_we = SignalData::from(1); //enable write
                wb_mux = SignalData::from(0); //ALU source
                alu_operator = SignalData::from(1); //ADD
                                                    //sign_zero_ext_data = 0; //don't care
                                                    //sign_zero_ext_sel = 1; //don't care
                                                    //big_imm = imm_big_shuffled; //don't care
                                                    //pc_imm_sel = 0; //don't care
                branch_logic_ctl = SignalData::from(0b011); //jalr
                branch_logic_enable = SignalData::from(0b1);
                jalr_imm = SignalData::from(imm);
            }
            0b1100011 => {
                //BRANCH
                trace!("opcode=BRANCH");
                regfile_rs1 = SignalData::from((instruction & (0b11111 << 15)) >> 15);
                regfile_rs2 = SignalData::from((instruction & (0b11111 << 20)) >> 20);
                //pc_imm_sel = 1;
                //branch_imm = imm;
                //regfile_rd = 0; //don't care
                //regfile_we = 0; //no link
                //wb_mux = 0; //don't care
                //alu_operator = 0; //don't care
                //sign_zero_ext_data = 0; //don't care
                //big_imm = 0; //don't care
                pc_imm_sel = SignalData::from(1); //branch imm
                branch_logic_ctl = SignalData::from(funct3); //use funct3
                branch_logic_enable = SignalData::from(0b1); //enable branch logic
                branch_imm = (((instruction & (0b1 << 31)) >> 19)
                    | ((instruction & (0b111111 << 25)) >> 20)
                    | ((instruction & (0b1111 << 8)) >> 7)
                    | ((instruction & (0b1 << 7)) << 4))
                    .into();
            }

            0b0000011 => {
                //LOAD
                trace!("opcode=LOAD");
                alu_operand_a_sel = SignalData::from(0); //rs1
                alu_operand_b_sel = SignalData::from(1); //imm
                regfile_rd = SignalData::from((instruction & (0b11111 << 7)) >> 7);
                regfile_rs1 = SignalData::from((instruction & (0b11111 << 15)) >> 15);
                regfile_we = SignalData::from(1);
                wb_mux = SignalData::from(1); //data memory
                alu_operator = SignalData::from(1); //ADD
                sign_zero_ext_data = SignalData::from(imm); //immediate
                sign_zero_ext_sel = SignalData::from(0); //sign extend

                data_mem_ctrl = SignalData::from(MemCtrl::Read as u32);
                match funct3 {
                    0b000 => {
                        data_mem_size = SignalData::from(1);
                        data_se = SignalData::from(1)
                    } //lb
                    0b001 => {
                        data_mem_size = SignalData::from(2);
                        data_se = SignalData::from(1)
                    } //lh
                    0b010 => {
                        data_mem_size = SignalData::from(4);
                        data_se = SignalData::from(1)
                    } //lw
                    0b100 => {
                        data_mem_size = SignalData::from(1);
                        data_se = SignalData::from(0)
                    } //lbu
                    0b101 => {
                        data_mem_size = SignalData::from(2);
                        data_se = SignalData::from(0)
                    } //lhu
                    _ => {
                        panic!("Unsupported funct3 {:b}", funct3)
                    }
                }
            }
            0b0100011 => {
                //STORE
                trace!("opcode=STORE");
                alu_operand_a_sel = SignalData::from(0); //rs1
                alu_operand_b_sel = SignalData::from(1); //imm
                regfile_rd = SignalData::Uninitialized;
                regfile_rs1 = SignalData::from((instruction & (0b11111 << 15)) >> 15);
                regfile_rs2 = SignalData::from((instruction & (0b11111 << 20)) >> 20);
                regfile_we = SignalData::from(0);
                //wb_mux = 0; //don't care
                alu_operator = SignalData::from(1); //ADD
                sign_zero_ext_data = SignalData::from(imm_store); //immediate store type
                sign_zero_ext_sel = SignalData::from(0); //sign extend

                data_mem_ctrl = SignalData::from(MemCtrl::Write as u32);
                match funct3 {
                    //size
                    0b000 => {
                        data_mem_size = SignalData::from(1);
                    }
                    0b001 => {
                        data_mem_size = SignalData::from(2);
                    }
                    0b010 => {
                        data_mem_size = SignalData::from(4);
                    }
                    _ => panic!("Unsupported funct3 {:b}", funct3),
                }
            }
            0b0 => {}
            _ => {
                panic!("Invalid opcode! {:b}", opcode);
            }
        };

        simulator.set_out_val(&self.id, "wb_mux", wb_mux);
        simulator.set_out_val(&self.id, "alu_operand_a_sel", alu_operand_a_sel);
        simulator.set_out_val(&self.id, "alu_operand_b_sel", alu_operand_b_sel);
        simulator.set_out_val(&self.id, "regfile_rs1", regfile_rs1);
        simulator.set_out_val(&self.id, "regfile_rs2", regfile_rs2);
        simulator.set_out_val(&self.id, "regfile_rd", regfile_rd);
        simulator.set_out_val(&self.id, "regfile_we", regfile_we);
        simulator.set_out_val(&self.id, "alu_operator", alu_operator);
        simulator.set_out_val(&self.id, "sign_zero_ext_sel", sign_zero_ext_sel);
        simulator.set_out_val(&self.id, "sign_zero_ext_data", sign_zero_ext_data);
        simulator.set_out_val(&self.id, "imm_a_mux_data", imm_a_mux_data);
        simulator.set_out_val(&self.id, "data_mem_size", data_mem_size);
        simulator.set_out_val(&self.id, "data_se", data_se);
        simulator.set_out_val(&self.id, "data_mem_ctrl", data_mem_ctrl);
        simulator.set_out_val(&self.id, "big_imm", big_imm);
        simulator.set_out_val(&self.id, "pc_imm_sel", pc_imm_sel);
        simulator.set_out_val(&self.id, "branch_imm", branch_imm);
        simulator.set_out_val(&self.id, "branch_logic_ctl", branch_logic_ctl);
        simulator.set_out_val(&self.id, "branch_logic_enable", branch_logic_enable);
        simulator.set_out_val(&self.id, "jalr_imm", jalr_imm);
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

        simulator.set_out_val("instruction", "out", 0x003100b3); //add x1, x2, x3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 3.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x40410133); //sub x2, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 2.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 2.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x004121b3); //slt x3, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 10.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x004131b3); //sltu x3, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 9.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x001151b3); //srl x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 4.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x401151b3); //sra x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 5.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x001111b3); //sll x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 3.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0020c1b3); //xor x3, x1, x2
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 6.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0020f1b3); //and x3, x1, x2)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 8.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0060e1b3); //or x3, x1, x6
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 6.into());
        assert_eq!(simulator.get_input_val(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 7.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);
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

        simulator.set_out_val("instruction", "out", 0x00310093); //addi x1, x2, 3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 2.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0xffd0a093); //slti x1, x1, -3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 1.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 10.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            ((-3i32 as u32) & 0b111111111111).into()
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0xffd0b093); //sltiu x1, x1, -3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 1.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 9.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            ((-3i32 as u32) & 0b111111111111).into()
        );
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00324093); //xori x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 4.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 6.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00326093); //ori x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 4.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 7.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00327093); //andi x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 4.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 8.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00c19093); //slli x1, x3, 12
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 3.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 3.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 12.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0011d093); //srli x1, x3, 1
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 3.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 4.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 1.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x4020d093); //srai x1, x1, 2
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 1.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 5.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 2.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);
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

        simulator.set_out_val("instruction", "out", 0xfffff0b7); //lui x1, 0xFFFFF
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs1),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 0.into());
        assert_eq!(simulator.get_input_val(imm_a_mux_data), 0xFFFFF000.into());
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0xfffff097); //auipc x1, 0xFFFFF
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 2.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs1),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(sign_zero_ext_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(imm_a_mux_data), 0xFFFFF000.into());
        assert_eq!(
            simulator.get_input_val(data_mem_size),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0082a223); //sw x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), SignalData::Uninitialized);
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 8.into());
        assert_eq!(
            simulator.get_input_val(regfile_rd),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_we), 0.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 4.into());
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00829223); //sh x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), SignalData::Uninitialized);
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 8.into());
        assert_eq!(
            simulator.get_input_val(regfile_rd),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_we), 0.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00828223); //sb x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), SignalData::Uninitialized);
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_val(regfile_rs2), 8.into());
        assert_eq!(
            simulator.get_input_val(regfile_rd),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_we), 0.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_val(data_se), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0042a403); //lw x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 4.into());
        assert_eq!(simulator.get_input_val(data_se), 1.into());
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00429403); //lh x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_val(data_se), 1.into());
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x00428403); //lb x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_val(data_se), 1.into());
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0042d403); //lhu x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_val(data_se), 0.into());
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);

        simulator.set_out_val("instruction", "out", 0x0042c403); //lbu x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_val(wb_mux), 1.into());
        assert_eq!(simulator.get_input_val(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_val(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_val(regfile_rs1), 5.into());
        assert_eq!(
            simulator.get_input_val(regfile_rs2),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_val(regfile_we), 1.into());
        assert_eq!(simulator.get_input_val(alu_operator), 1.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_val(sign_zero_ext_data), 4.into());
        assert_eq!(
            simulator.get_input_val(imm_a_mux_data),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_val(data_se), 0.into());
        assert_eq!(
            simulator.get_input_val(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_val(big_imm), SignalData::Uninitialized);
        assert_eq!(
            simulator.get_input_val(pc_imm_sel),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_imm),
            SignalData::Uninitialized
        );
        assert_eq!(
            simulator.get_input_val(branch_logic_ctl),
            SignalData::Uninitialized
        );
        assert_eq!(simulator.get_input_val(branch_logic_enable), 0.into());
        assert_eq!(simulator.get_input_val(jalr_imm), SignalData::Uninitialized);
    }
}

// 0xfffff0b7);//lui x1, 0xFFFFF #x1=0xFFFFF000
// 0xfffff097);//auipc x1, 0xFFFFF #x1=0xFFFFF040
// //0x00828223);//sb x8, 4(x5) # should panic over opcode for now
// 0x0082a223);//sw x8, 4(x5) store x1=4 at 0
// 0x00829223);//sh x8, 4(x5)
// 0x0042a403);//lw x8, 4(x5) x5=4
// 0x00429403);//lh x8, 4(x5)
// 0x00428403);//lb x8, 4(x5)
// 0x0042d403);//lhu x8, 4(x5)
// 0x0042c403);//lbu x8, 4(x5)
