use log::trace;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, RunningState, SignalValue,
    Simulator,
};
use syncrim::components::MemCtrl;

pub const DECODER_INSTRUCTION_ID: &str = "instruction";

pub const DECODER_WB_MUX_SEL_ID: &str = "decoder_wb_mux_sel";
pub const DECODER_ALU_A_MUX_SEL_ID: &str = "decoder_alu_a_mux_sel";
pub const DECODER_ALU_B_MUX_SEL_ID: &str = "decoder_alu_b_mux_sel";
pub const DECODER_ALU_OP_ID: &str = "decoder_alu_op";
// reg file signals
pub const DECODER_RD_ID: &str = "decoder_rd";
pub const DECODER_RS1_ID: &str = "decoder_rs1";
pub const DECODER_RS2_ID: &str = "decoder_rs2";
pub const DECODER_WB_WRITE_ENABLE_ID: &str = "decoder_wb_write_enable";
//unsure here
pub const DECODER_SIGN_ZERO_EXT_SEL_ID: &str = "sign_zero_ext_sel";

//this is imm
// consolidate these as immediate
//pub const DECODER_IMM_ID: &str = "decoder_imm";
pub const DECODER_PC_IMM_SEL_ID: &str = "pc_imm_sel";

pub const DECODER_LUI_AUIPC_IMM_ID: &str = "decoder_lui_auipc_imm";
pub const DECODER_SHAMT_ID: &str = "decoder_shamt";
pub const DECODER_IMM_ID: &str = "decoder_imm";
pub const DECODER_STORE_OFFSET_IMM_ID: &str = "decoder_store_offset_imm";
pub const DECODER_ZIMM_ID: &str = "decoder_zimm";
pub const DECODER_JAL_IMM_ID: &str = "decoder_jal_imm";
pub const DECODER_BRANCH_IMM_ID: &str = "decoder_branch_imm";
pub const DECODER_IMM_SEL_ID: &str = "decoder_imm_sel";
//"pc_se_data".into(),
//"pc_mux_sel".into(),
pub const DECODER_DATA_MEM_SIZE_ID: &str = "data_mem_size";
pub const DECODER_DATA_SE_ID: &str = "data_se";
pub const DECODER_DATA_MEM_CTRL_ID: &str = "data_mem_ctrl";

pub const DECODER_BRANCH_OP: &str = "decoder_branch_op";
pub const DECODER_BRANCH_INSTR: &str = "decoder_branch_instr";
pub const DECODER_BRANCH_ALWAYS: &str = "decoder_branch_always";

pub const DECODER_MRET_ID: &str = "mret";
pub const DECODER_MEPC_ID: &str = "mepc";
pub const DECODER_CSR_CTL_ID: &str = "csr_ctl";
pub const DECODER_CSR_DATA_MUX_ID: &str = "csr_data_mux";
pub const DECODER_CSR_ADDR_ID: &str = "csr_addr";

pub const DECODER_HEIGHT: f32 = 600.0;
pub const DECODER_WIDTH: f32 = 30.0;

#[derive(Serialize, Deserialize)]
pub struct Decoder {
    pub width: f32,
    pub height: f32,
    pub id: String,
    pub pos: (f32, f32),

    pub instruction: Input,
}
#[repr(u8)]
pub enum ImmSel {
    LuiAuipc = 0,
    Shamt = 1,
    Imm = 2,
    StoreImm = 3,
    Zimm = 4,
    JalImm = 5,
}

impl Into<SignalValue> for ImmSel {
    fn into(self) -> SignalValue {
        SignalValue::Data(self as u32)
    }
}

fn sign_zero_extend(sign: bool, width: u8, val: u32) -> u32 {
    assert!(width > 0);
    if sign {
        let sign_bit = val >> (width - 1);
        let mask = !(2u32.pow(width as u32) - 1);
        //println!("MASK: {:08x}", mask);
        if sign_bit == 1 {
            val | mask
        } else {
            val
        }
    } else {
        val
    }
}

#[typetag::serde()]
impl Component for Decoder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        println!("Decoder");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Decoder {
            width: DECODER_WIDTH,
            height: DECODER_HEIGHT,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            instruction: dummy_input,
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == DECODER_INSTRUCTION_ID {
            self.instruction = new_input
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: DECODER_INSTRUCTION_ID.to_string(),
                    input: self.instruction.clone(),
                }],
                OutputType::Combinatorial,
                vec![
                    DECODER_WB_MUX_SEL_ID,
                    DECODER_ALU_A_MUX_SEL_ID,
                    DECODER_ALU_B_MUX_SEL_ID,
                    DECODER_ALU_OP_ID,
                    DECODER_RD_ID,
                    DECODER_RS1_ID,
                    DECODER_RS2_ID,
                    DECODER_WB_WRITE_ENABLE_ID,
                    DECODER_SIGN_ZERO_EXT_SEL_ID,
                    DECODER_DATA_MEM_SIZE_ID,
                    DECODER_DATA_SE_ID,
                    DECODER_DATA_MEM_CTRL_ID,
                    DECODER_BRANCH_OP,
                    DECODER_BRANCH_INSTR,
                    DECODER_MEPC_ID,
                    DECODER_MRET_ID,
                    DECODER_CSR_CTL_ID,
                    DECODER_CSR_DATA_MUX_ID,
                    DECODER_CSR_ADDR_ID,
                    DECODER_PC_IMM_SEL_ID,
                    DECODER_BRANCH_ALWAYS,
                    DECODER_IMM_ID,
                    DECODER_LUI_AUIPC_IMM_ID,
                    DECODER_SHAMT_ID,
                    DECODER_STORE_OFFSET_IMM_ID,
                    DECODER_ZIMM_ID,
                    DECODER_JAL_IMM_ID,
                    DECODER_IMM_SEL_ID,
                    DECODER_BRANCH_IMM_ID,
                ],
            ),
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let instruction: u32 = simulator
            .get_input_value(&self.instruction)
            .try_into()
            .unwrap();

        //constant instruction field values
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
        let branch_imm = ((instruction & (0b1 << 31)) >> 19)
            | ((instruction & (0b111111 << 25)) >> 20)
            | ((instruction & (0b1111 << 8)) >> 7)
            | ((instruction & (0b1 << 7)) << 4);
        //no idea why this is encoded this way but the ISA is what it is
        let imm_store =
            ((instruction & (0b11111 << 7)) >> 7) | ((instruction & (0b1111111 << 25)) >> 20);
        let zimm = (instruction & (0b11111 << 15)) >> 15;
        //outputs
        //let mut imm_sig = SignalValue::Uninitialized;
        let branch_imm_sig = SignalValue::Data(sign_zero_extend(true, 13, branch_imm));
        let lui_auipc_imm_sig = SignalValue::Data(imm_big);
        let shamt_sig = SignalValue::Data(shamt);
        let mut imm_sig = SignalValue::Data(imm);
        let store_offset_sig = SignalValue::Data(imm_store);
        let zimm_sig = SignalValue::Data(zimm);
        let jal_imm_sig = SignalValue::Data(sign_zero_extend(true, 21, imm_big_shuffled));
        let mut imm_sel_sig = SignalValue::Uninitialized;
        let mut wb_mux_sel = SignalValue::Uninitialized;
        let mut alu_a_mux_sel = SignalValue::Uninitialized;
        let mut alu_b_mux_sel = SignalValue::Uninitialized;
        let mut rd = SignalValue::Uninitialized;
        let mut rs1 = SignalValue::Uninitialized;
        let mut rs2 = SignalValue::Uninitialized;
        let mut wb_write_enable = SignalValue::from(0); //this must be 0
        let mut alu_op = SignalValue::Uninitialized;
        let mut sub_arith = SignalValue::Uninitialized;
        let mut dmem_width = SignalValue::Uninitialized;
        let mut dmem_sign_extend = SignalValue::Uninitialized;
        let mut dmem_write_enable = SignalValue::from(MemCtrl::None as u32);
        // ??
        let pc_imm_sel = SignalValue::Uninitialized;
        let mut branch_instr = SignalValue::Uninitialized;
        let mut branch_logic_enable = SignalValue::from(0); //this must be 0
        let mut csr_ctl = SignalValue::Uninitialized;
        let mut csr_data_mux = SignalValue::Uninitialized;
        let mut csr_addr = SignalValue::Uninitialized;
        let mut mret = SignalValue::Uninitialized;
        match opcode {
            0b0110011 => {
                //OP
                alu_a_mux_sel = SignalValue::from(4); //rs1
                alu_b_mux_sel = SignalValue::from(0); //rs2
                                                      //rs1 [19:15] rs2 [24:20] rd [11:7]
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                rs2 = SignalValue::from((instruction & (0b11111 << 20)) >> 20);
                wb_write_enable = SignalValue::from(1); //enable write
                wb_mux_sel = SignalValue::from(0); //ALU source
                trace!("opcode=OP");
                match funct3 {
                    0b000 => {
                        // add/sub
                        match funct7 {
                            0b0000000 => {
                                alu_op = SignalValue::from(1);
                            } //add
                            0b0100000 => {
                                alu_op = SignalValue::from(2);
                            } //sub
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b001 => {
                        match funct7 {
                            // sll
                            0b0000000 => {
                                alu_op = SignalValue::from(3);
                            } //sll
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b010 => {
                        match funct7 {
                            // slt
                            0b0000000 => {
                                alu_op = SignalValue::from(10);
                            } //slt
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b011 => {
                        match funct7 {
                            // sltu
                            0b0000000 => {
                                alu_op = SignalValue::from(9);
                            } //sltu
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b100 => {
                        match funct7 {
                            // xor
                            0b0000000 => {
                                alu_op = SignalValue::from(6);
                            } //xor
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b101 => {
                        match funct7 {
                            // srl
                            0b0000000 => {
                                alu_op = SignalValue::from(4);
                            } //srl
                            0b0100000 => {
                                alu_op = SignalValue::from(5);
                            } //sra
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b110 => {
                        match funct7 {
                            // or
                            0b0000000 => {
                                alu_op = SignalValue::from(7);
                            } //or
                            _ => panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b111 => {
                        //and
                        match funct7 {
                            0b0000000 => {
                                alu_op = SignalValue::from(8);
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
                alu_a_mux_sel = SignalValue::from(4); //rs1
                alu_b_mux_sel = SignalValue::from(2); //imm
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                wb_write_enable = SignalValue::from(1); //enable write
                wb_mux_sel = SignalValue::from(0); //ALU source
                imm_sig = SignalValue::from(sign_zero_extend(true, 12, imm));
                imm_sel_sig = ImmSel::Imm.into();
                trace!("opcode=OP_IMM");
                match funct3 {
                    0b000 => {
                        //ADDI
                        alu_op = SignalValue::from(1);
                        sub_arith = SignalValue::from(0);
                    }
                    0b010 => {
                        //SLTI
                        alu_op = SignalValue::from(10);
                        sub_arith = SignalValue::from(0);
                    }
                    0b011 => {
                        //SLTIU
                        alu_op = SignalValue::from(9);
                        sub_arith = SignalValue::from(1);
                        imm_sig = SignalValue::from(sign_zero_extend(false, 12, imm));
                    }
                    0b100 => {
                        //XORI
                        alu_op = SignalValue::from(6);
                        sub_arith = SignalValue::from(0);
                    }
                    0b110 => {
                        //ORI
                        alu_op = SignalValue::from(7);
                        sub_arith = SignalValue::from(0);
                    }
                    0b111 => {
                        //ANDI
                        alu_op = SignalValue::from(8);
                        sub_arith = SignalValue::from(0);
                    }
                    0b001 => {
                        //SLLI
                        alu_op = SignalValue::from(3);
                        sub_arith = SignalValue::from(1);
                        //imm_sel_sig = ImmSel::Shamt.into();
                        alu_b_mux_sel = SignalValue::from(3); //shamt
                    }
                    0b101 => {
                        //SRLI SRAI
                        match funct7 {
                            0b0000000 => {
                                alu_op = SignalValue::from(4);
                                sub_arith = SignalValue::from(1);
                                alu_b_mux_sel = SignalValue::from(3); //shamt
                            } //SRLI
                            0b0100000 => {
                                alu_op = SignalValue::from(5);
                                sub_arith = SignalValue::from(1);
                                alu_b_mux_sel = SignalValue::from(3); //shamt
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
                alu_a_mux_sel = SignalValue::from(3); //0
                alu_b_mux_sel = SignalValue::from(4); //lui imm
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = 0; //x0 dont care
                wb_write_enable = SignalValue::from(1); //enable write
                wb_mux_sel = SignalValue::from(0); //ALU source
                alu_op = SignalValue::from(1); //ADD

                //THIS NEEDS FIX
                //sign_zero_ext_data = SignalValue::from(0); //add 0
                sub_arith = SignalValue::from(1); //zero-extend
                                                  //imm_sig = SignalValue::from(imm_big);
                imm_sel_sig = ImmSel::LuiAuipc.into();
            }
            0b0010111 => {
                //AUIPC
                trace!("opcode=AUIPC");
                alu_a_mux_sel = SignalValue::from(0); //auipc imm
                alu_b_mux_sel = SignalValue::from(5); //PC
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = SignalValue::from(0); //x0 dont care
                wb_write_enable = SignalValue::from(1); //enable write
                wb_mux_sel = SignalValue::from(0); //ALU source
                alu_op = SignalValue::from(1); //ADD
                                               //sign_zero_ext_data = SignalValue::from(0); //don't care
                                               //sign_zero_ext_sel = SignalValue::from(1); //don't care
                imm_sel_sig = ImmSel::LuiAuipc.into();
                //imm_sig = SignalValue::from(imm_big);
            }
            0b1101111 => {
                //JAL
                trace!("opcode=JAL");
                alu_a_mux_sel = SignalValue::from(1); //jal imm
                alu_b_mux_sel = SignalValue::from(5); //PC
                sub_arith = SignalValue::from(0); //sign extend
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                //regfile_rs1 = SignalValue::from(0); //dont care
                wb_write_enable = SignalValue::from(1); //enable write
                wb_mux_sel = SignalValue::from(3); //PC_p4
                alu_op = SignalValue::from(1); //ADD
                                               //sign_zero_ext_data = 0; //don't care
                                               //sign_zero_ext_sel = 1; //don't care
                                               //imm_sig = SignalValue::from(sign_zero_extend(true, 21, imm_big_shuffled));
                                               //pc_imm_sel = SignalValue::from(0);
                imm_sel_sig = ImmSel::JalImm.into();
                branch_instr = SignalValue::from(0b010); //jal
                branch_logic_enable = SignalValue::from(0b1);
            }
            0b1100111 => {
                //JALR
                trace!("opcode=JALR");
                alu_a_mux_sel = SignalValue::from(4); //rs1
                alu_b_mux_sel = SignalValue::from(2); //imm
                sub_arith = SignalValue::from(0); //sign extend
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                wb_write_enable = SignalValue::from(1); //enable write
                wb_mux_sel = SignalValue::from(3); //PC_p4
                alu_op = SignalValue::from(1); //ADD
                                               //sign_zero_ext_data = 0; //don't care
                                               //sign_zero_ext_sel = 1; //don't care
                                               //big_imm = imm_big_shuffled; //don't care
                                               //pc_imm_sel = 0; //don't care
                branch_instr = SignalValue::from(0b011); //jalr
                branch_logic_enable = SignalValue::from(0b1);
                imm_sig = SignalValue::from(sign_zero_extend(true, 12, imm));
                //imm_sig = SignalValue::from(sign_zero_extend(true, 12, imm));
            }
            0b1100011 => {
                //BRANCH
                trace!("opcode=BRANCH");
                rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                rs2 = SignalValue::from((instruction & (0b11111 << 20)) >> 20);
                alu_a_mux_sel = SignalValue::from(2); //branch imm
                alu_b_mux_sel = SignalValue::from(5); //PC
                alu_op = SignalValue::from(1); //add
                sub_arith = SignalValue::from(0); //sign extend
                branch_instr = SignalValue::from(funct3); //use funct3
                branch_logic_enable = SignalValue::from(0b1); //enable branch logic
                imm_sel_sig = ImmSel::Imm.into();
                //imm_sig = sign_zero_extend(true, 13, imm_int).into();
            }

            0b0000011 => {
                //LOAD
                trace!("opcode=LOAD");
                alu_a_mux_sel = SignalValue::from(4); //rs1
                alu_b_mux_sel = SignalValue::from(2); //imm
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                wb_write_enable = SignalValue::from(1);
                wb_mux_sel = SignalValue::from(1); //data memory
                alu_op = SignalValue::from(1); //ADD
                imm_sel_sig = ImmSel::Imm.into();
                imm_sig = SignalValue::from(sign_zero_extend(true, 13, imm)); //immediate
                sub_arith = SignalValue::from(0); //sign extend

                dmem_write_enable = SignalValue::from(MemCtrl::Read as u32);
                match funct3 {
                    0b000 => {
                        dmem_width = SignalValue::from(1);
                        dmem_sign_extend = SignalValue::from(1)
                    } //lb
                    0b001 => {
                        dmem_width = SignalValue::from(2);
                        dmem_sign_extend = SignalValue::from(1)
                    } //lh
                    0b010 => {
                        dmem_width = SignalValue::from(4);
                        dmem_sign_extend = SignalValue::from(1)
                    } //lw
                    0b100 => {
                        dmem_width = SignalValue::from(1);
                        dmem_sign_extend = SignalValue::from(0)
                    } //lbu
                    0b101 => {
                        dmem_width = SignalValue::from(2);
                        dmem_sign_extend = SignalValue::from(0)
                    } //lhu
                    _ => {
                        panic!("Unsupported funct3 {:b}", funct3)
                    }
                }
            }
            0b0100011 => {
                //STORE
                trace!("opcode=STORE");
                alu_a_mux_sel = SignalValue::from(4); //rs1
                alu_b_mux_sel = SignalValue::from(1); // store imm
                rd = SignalValue::Uninitialized;
                rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                rs2 = SignalValue::from((instruction & (0b11111 << 20)) >> 20);
                wb_write_enable = SignalValue::from(0);
                //wb_mux = 0; //don't care
                alu_op = SignalValue::from(1); //ADD
                imm_sel_sig = ImmSel::StoreImm.into();
                //imm_sig = SignalValue::from(imm_store); //immediate store type
                sub_arith = SignalValue::from(0); //sign extend

                dmem_write_enable = SignalValue::from(MemCtrl::Write as u32);
                match funct3 {
                    //size
                    0b000 => {
                        dmem_width = SignalValue::from(1);
                    }
                    0b001 => {
                        dmem_width = SignalValue::from(2);
                    }
                    0b010 => {
                        dmem_width = SignalValue::from(4);
                    }
                    _ => panic!("Unsupported funct3 {:b}", funct3),
                }
            }
            0b1110011 => {
                //SYSTEM
                csr_addr = SignalValue::from(imm); //imm
                wb_write_enable = SignalValue::from(1); //write enable
                wb_mux_sel = SignalValue::from(2); //csr data out
                rd = SignalValue::from((instruction & (0b11111 << 7)) >> 7);
                if instruction == 807403635
                //mret, basically magic number
                {
                    mret = SignalValue::from(1);
                }
                // ebreak, also magic number
                // for now, this can put simulator in halted state
                else if instruction == 1048691 {
                    simulator.running_state = RunningState::Halt;
                } else {
                    match funct3 {
                        0b001 => {
                            //CSRRW
                            csr_ctl = SignalValue::from(1); //write
                            csr_data_mux = SignalValue::from(0); //register
                            rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                            //rs1
                        }
                        0b010 => {
                            //CSRRS
                            csr_ctl = SignalValue::from(2); //set
                            csr_data_mux = SignalValue::from(0); //register
                            rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                            //rs1
                        }
                        0b011 => {
                            //CSRRC
                            csr_ctl = SignalValue::from(3); //clear
                            csr_data_mux = SignalValue::from(0); //register
                            rs1 = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                            //rs1
                        }
                        0b101 => {
                            //CSRRWI
                            csr_ctl = SignalValue::from(1); //write
                            csr_data_mux = SignalValue::from(1); //immediate
                            imm_sel_sig = ImmSel::Zimm.into();
                            //imm_sig = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                            //zimm
                        }
                        0b110 => {
                            //CSRRSI
                            csr_ctl = SignalValue::from(2); //set
                            csr_data_mux = SignalValue::from(1); //immediate
                            imm_sel_sig = ImmSel::Zimm.into();

                            // imm_sig = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                            //zimm
                        }
                        0b111 => {
                            //CSRRCI
                            csr_ctl = SignalValue::from(3); //clear
                            csr_data_mux = SignalValue::from(1); //immediate
                            imm_sel_sig = ImmSel::Zimm.into();

                            //imm_sig = SignalValue::from((instruction & (0b11111 << 15)) >> 15);
                            //zimm
                        }
                        _ => panic!("Unsupported funct3 {:b}", funct3),
                    }
                }
            }
            _ => {
                if !(opcode == 0 && simulator.cycle == 0) {
                    panic!("Invalid opcode! {:b}", opcode)
                }
            }
        };

        simulator.set_out_value(&self.id, DECODER_WB_MUX_SEL_ID, wb_mux_sel);
        simulator.set_out_value(&self.id, DECODER_ALU_A_MUX_SEL_ID, alu_a_mux_sel);
        simulator.set_out_value(&self.id, DECODER_ALU_B_MUX_SEL_ID, alu_b_mux_sel);
        simulator.set_out_value(&self.id, DECODER_RS1_ID, rs1);
        simulator.set_out_value(&self.id, DECODER_RS2_ID, rs2);
        simulator.set_out_value(&self.id, DECODER_RD_ID, rd);
        simulator.set_out_value(&self.id, DECODER_WB_WRITE_ENABLE_ID, wb_write_enable);
        simulator.set_out_value(&self.id, DECODER_ALU_OP_ID, alu_op);
        simulator.set_out_value(&self.id, DECODER_SIGN_ZERO_EXT_SEL_ID, sub_arith);
        simulator.set_out_value(&self.id, DECODER_IMM_ID, imm_sig);
        //simulator.set_out_value(&self.id, DECODER_IMM_A_MUX_DATA_ID, imm_a_mux_data);
        simulator.set_out_value(&self.id, DECODER_DATA_MEM_SIZE_ID, dmem_width);
        simulator.set_out_value(&self.id, DECODER_DATA_SE_ID, dmem_sign_extend);
        simulator.set_out_value(&self.id, DECODER_DATA_MEM_CTRL_ID, dmem_write_enable);
        //simulator.set_out_value(&self.id, DECODER_BIG_IMM_ID, big_imm);
        simulator.set_out_value(&self.id, DECODER_PC_IMM_SEL_ID, pc_imm_sel);
        //simulator.set_out_value(&self.id, DECODER_BRANCH_IMM_ID, branch_imm);
        simulator.set_out_value(&self.id, DECODER_BRANCH_OP, branch_instr);
        simulator.set_out_value(&self.id, DECODER_BRANCH_INSTR, branch_logic_enable);
        //simulator.set_out_value(&self.id, DECODER_JALR_IMM_ID, jalr_imm);
        simulator.set_out_value(&self.id, DECODER_CSR_CTL_ID, csr_ctl);
        simulator.set_out_value(&self.id, DECODER_CSR_DATA_MUX_ID, csr_data_mux);
        // simulator.set_out_value(&self.id, DECODER_CSR_DATA_ID, csr_data);
        simulator.set_out_value(&self.id, DECODER_CSR_ADDR_ID, csr_addr);
        simulator.set_out_value(&self.id, DECODER_MRET_ID, mret);
        simulator.set_out_value(&self.id, DECODER_LUI_AUIPC_IMM_ID, lui_auipc_imm_sig);
        simulator.set_out_value(&self.id, DECODER_SHAMT_ID, shamt_sig);
        simulator.set_out_value(&self.id, DECODER_STORE_OFFSET_IMM_ID, store_offset_sig);
        simulator.set_out_value(&self.id, DECODER_ZIMM_ID, zimm_sig);
        simulator.set_out_value(&self.id, DECODER_JAL_IMM_ID, jal_imm_sig);
        simulator.set_out_value(&self.id, DECODER_IMM_SEL_ID, imm_sel_sig);
        simulator.set_out_value(&self.id, DECODER_BRANCH_IMM_ID, branch_imm_sig);
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
                    width: 0.0,
                    height: 0.0,
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();

        // outputs
        let wb_mux = &Input::new("decoder", DECODER_WB_MUX_SEL_ID);
        let alu_operand_a_sel = &Input::new("decoder", DECODER_ALU_A_MUX_SEL_ID);
        let alu_operand_b_sel = &Input::new("decoder", DECODER_ALU_B_MUX_SEL_ID);
        let regfile_rs1 = &Input::new("decoder", DECODER_RS1_ID);
        let regfile_rs2 = &Input::new("decoder", DECODER_RS2_ID);
        let regfile_rd = &Input::new("decoder", DECODER_RD_ID);
        let regfile_we = &Input::new("decoder", DECODER_WB_WRITE_ENABLE_ID);
        let alu_operator = &Input::new("decoder", DECODER_ALU_OP_ID);
        let data_mem_ctrl = &Input::new("decoder", DECODER_DATA_MEM_CTRL_ID);
        let branch_logic_enable = &Input::new("decoder", DECODER_BRANCH_INSTR);

        simulator.set_out_value("instruction", "out", 0x003100b3); //add x1, x2, x3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x40410133); //sub x2, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 2.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x004121b3); //slt x3, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 10.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x004131b3); //sltu x3, x2, x4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 9.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x001151b3); //srl x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 4.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x401151b3); //sra x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 5.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x001111b3); //sll x3, x2, x1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 3.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0020c1b3); //xor x3, x1, x2
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 6.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0020f1b3); //and x3, x1, x2)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 8.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0060e1b3); //or x3, x1, x6
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 0.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 6.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 3.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 7.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
    }
    #[test]
    fn test_op_imm() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    width: 0.0,
                    height: 0.0,
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();

        // outputs
        let wb_mux = &Input::new("decoder", DECODER_WB_MUX_SEL_ID);
        let alu_operand_a_sel = &Input::new("decoder", DECODER_ALU_A_MUX_SEL_ID);
        let alu_operand_b_sel = &Input::new("decoder", DECODER_ALU_B_MUX_SEL_ID);
        let regfile_rs1 = &Input::new("decoder", DECODER_RS1_ID);
        let regfile_rd = &Input::new("decoder", DECODER_RD_ID);
        let regfile_we = &Input::new("decoder", DECODER_WB_WRITE_ENABLE_ID);
        let alu_operator = &Input::new("decoder", DECODER_ALU_OP_ID);
        let sign_zero_ext_sel = &Input::new("decoder", DECODER_SIGN_ZERO_EXT_SEL_ID);
        let sign_zero_ext_data = &Input::new("decoder", DECODER_IMM_ID);
        let data_mem_ctrl = &Input::new("decoder", DECODER_DATA_MEM_CTRL_ID);
        let branch_logic_enable = &Input::new("decoder", DECODER_BRANCH_INSTR);

        simulator.set_out_value("instruction", "out", 0x00310093); //addi x1, x2, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0xffd0a093); //slti x1, x1, -3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 10.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            (-3i32 as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0xffd0b093); //sltiu x1, x1, -3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 9.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 1.into());
        assert_eq!(
            simulator.get_input_value(sign_zero_ext_data),
            ((-3i32 & 0b111111111111) as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x00324093); //xori x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 6.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x00326093); //ori x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 7.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x00327093); //andi x1, x4, 3
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 4.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 8.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 3.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x00c19093); //slli x1, x3, 12
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 3.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0011d093); //srli x1, x3, 1
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 4.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x4020d093); //srai x1, x1, 2
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 3.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 5.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
    }
    #[test]
    fn test_lui_auipc_store_load() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    width: 0.0,
                    height: 0.0,
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();

        // outputs
        let wb_mux = &Input::new("decoder", DECODER_WB_MUX_SEL_ID);
        let alu_operand_a_sel = &Input::new("decoder", DECODER_ALU_A_MUX_SEL_ID);
        let alu_operand_b_sel = &Input::new("decoder", DECODER_ALU_B_MUX_SEL_ID);
        let regfile_rs1 = &Input::new("decoder", DECODER_RS1_ID);
        let regfile_rs2 = &Input::new("decoder", DECODER_RS2_ID);
        let regfile_rd = &Input::new("decoder", DECODER_RD_ID);
        let regfile_we = &Input::new("decoder", DECODER_WB_WRITE_ENABLE_ID);
        let alu_operator = &Input::new("decoder", DECODER_ALU_OP_ID);
        let sign_zero_ext_sel = &Input::new("decoder", DECODER_SIGN_ZERO_EXT_SEL_ID);
        let sign_zero_ext_data = &Input::new("decoder", DECODER_IMM_ID);
        let imm_a_mux_data = &Input::new("decoder", DECODER_LUI_AUIPC_IMM_ID);
        let data_mem_size = &Input::new("decoder", DECODER_DATA_MEM_SIZE_ID);
        let data_se = &Input::new("decoder", DECODER_DATA_SE_ID);
        let data_mem_ctrl = &Input::new("decoder", DECODER_DATA_MEM_CTRL_ID);
        let branch_logic_enable = &Input::new("decoder", DECODER_BRANCH_INSTR);

        simulator.set_out_value("instruction", "out", 0xfffff0b7); //lui x1, 0xFFFFF
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 3.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 4.into());
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
        assert_eq!(simulator.get_input_value(imm_a_mux_data), 0xFFFFF000.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        simulator.set_out_value("instruction", "out", 0xfffff097); //auipc x1, 0xFFFFF
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 0.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(imm_a_mux_data), 0xFFFFF000.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0082a223); //sw x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 4.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x00829223); //sh x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 2.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        simulator.set_out_value("instruction", "out", 0x00828223); //sb x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Write as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0042a403); //lw x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 4.into());
        assert_eq!(simulator.get_input_value(data_se), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x00429403); //lh x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_value(data_se), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
        simulator.set_out_value("instruction", "out", 0x00428403); //lb x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_value(data_se), 1.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0042d403); //lhu x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 2.into());
        assert_eq!(simulator.get_input_value(data_se), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());

        simulator.set_out_value("instruction", "out", 0x0042c403); //lbu x8, 4(x5)
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs1), 5.into());
        assert_eq!(simulator.get_input_value(regfile_rd), 8.into());
        assert_eq!(simulator.get_input_value(regfile_we), 1.into());
        assert_eq!(simulator.get_input_value(alu_operator), 1.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_sel), 0.into());
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 4.into());
        assert_eq!(simulator.get_input_value(data_mem_size), 1.into());
        assert_eq!(simulator.get_input_value(data_se), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::Read as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_enable), 0.into());
    }

    #[test]
    fn test_jal_jalr_branch() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instruction")),
                Rc::new(Decoder {
                    width: 0.0,
                    height: 0.0,
                    id: "decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: Input::new("instruction", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();

        // outputs
        let wb_mux = &Input::new("decoder", DECODER_WB_MUX_SEL_ID);
        let alu_operand_a_sel = &Input::new("decoder", DECODER_ALU_A_MUX_SEL_ID);
        let alu_operand_b_sel = &Input::new("decoder", DECODER_ALU_B_MUX_SEL_ID);
        let regfile_rs1 = &Input::new("decoder", DECODER_RS1_ID);
        let regfile_rs2 = &Input::new("decoder", DECODER_RS2_ID);
        let regfile_rd = &Input::new("decoder", DECODER_RD_ID);
        let regfile_we = &Input::new("decoder", DECODER_WB_WRITE_ENABLE_ID);
        let alu_operator = &Input::new("decoder", DECODER_ALU_OP_ID);
        let sign_zero_ext_data = &Input::new("decoder", DECODER_IMM_ID);
        let data_mem_size = &Input::new("decoder", DECODER_DATA_MEM_SIZE_ID);
        let data_se = &Input::new("decoder", DECODER_DATA_SE_ID);
        let data_mem_ctrl = &Input::new("decoder", DECODER_DATA_MEM_CTRL_ID);
        let big_imm = &Input::new("decoder", DECODER_JAL_IMM_ID);
        let branch_imm = &Input::new("decoder", DECODER_BRANCH_IMM_ID);
        let branch_logic_ctl = &Input::new("decoder", DECODER_BRANCH_OP);
        let branch_logic_enable = &Input::new("decoder", DECODER_BRANCH_INSTR);
        let jalr_imm = &Input::new("decoder", DECODER_IMM_ID);

        simulator.set_out_value("instruction", "out", 0x0080016f); //jal x2, 8
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 3.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 1.into());
        assert_eq!(simulator.get_input_value(alu_operand_b_sel), 5.into());
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
        assert_eq!(simulator.get_input_value(sign_zero_ext_data), 8.into());
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
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 2.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());

        simulator.set_out_value("instruction", "out", 0x00410167); //jalr x2, x2, 4
        simulator.clock();
        assert_eq!(simulator.get_input_value(wb_mux), 3.into());
        assert_eq!(simulator.get_input_value(alu_operand_a_sel), 4.into());
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
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 3.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
        assert_eq!(simulator.get_input_value(jalr_imm), 4.into());

        simulator.set_out_value("instruction", "out", 0xfe209ee3); //bne x1, x2, -4
        simulator.clock();
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_imm), (-4i32 as u32).into());
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 1.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());

        simulator.set_out_value("instruction", "out", 0x00208463); //beq, x1, x2, 8
        simulator.clock();
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(
            simulator.get_input_value(branch_imm),
            ((8i32) as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());

        simulator.set_out_value("instruction", "out", 0xfe20cee3); //blt x1, x2, -4
        simulator.clock();
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_imm), (-4i32 as u32).into());
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b100.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());

        simulator.set_out_value("instruction", "out", 0xfe116ee3); //bltu, x2, x1, -4
        simulator.clock();
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_imm), (-4i32 as u32).into());
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b110.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());

        simulator.set_out_value("instruction", "out", 0xfe115ee3); //bge x2, x1, -4
        simulator.clock();
        assert_eq!(simulator.get_input_value(regfile_rs1), 2.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 1.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_imm), (-4i32 as u32).into());
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b101.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());

        simulator.set_out_value("instruction", "out", 0xfe20fee3); //bgeu x1, x2, -4
        simulator.clock();
        assert_eq!(simulator.get_input_value(regfile_rs1), 1.into());
        assert_eq!(simulator.get_input_value(regfile_rs2), 2.into());
        assert_eq!(simulator.get_input_value(regfile_we), 0.into());
        assert_eq!(
            simulator.get_input_value(data_mem_ctrl),
            (MemCtrl::None as u32).into()
        );
        assert_eq!(simulator.get_input_value(branch_imm), (-4i32 as u32).into());
        assert_eq!(simulator.get_input_value(branch_logic_ctl), 0b111.into());
        assert_eq!(simulator.get_input_value(branch_logic_enable), 1.into());
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
