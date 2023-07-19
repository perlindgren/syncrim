use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Signal, Output, OutputType, Ports, Simulator};
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
    fn to_string(&self)->String{"".to_string()}
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.instruction.clone(),],
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
    fn evaluate(&self, simulator: &mut Simulator) {
        let LOAD = 0b0000011;
        let STORE = 0b0100011;
        //let OP = 0b0110011;
        let OP_IMM = 0b0010011;

        let instruction = simulator.get_input_val(&self.instruction);
        let opcode = instruction & 0b1111111;
        let funct3 = (instruction & (0b111<<12))>>12;
        let funct7 = (instruction & (0b1111111<<25))>>25;
        let imm = instruction>>20;
        let shamt = (instruction&(0b11111<<20))>>20;
        let imm_big = instruction&0xFFFFF000;
        let imm_big_shuffled = 
        (((instruction & (0b1<<31))>>(31-20))|
        ((instruction & (0b1111111111<<21))>>(30-10))|
        ((instruction & (0b1<<20))>>(20-11))|
        ((instruction & (0b11111111<<12))))&0b1111_1111_1111_1111_1111_1111_1111_1110;
          //no idea why this is encoded this way but the ISA is what it is
        let imm_store = ((instruction&(0b11111<<7))>>7) | ((instruction&(0b1111111<<25))>>20);
        let branch_imm = 
        ((instruction&(0b1<<31))>>19)|
        ((instruction&(0b111111<<25))>>20)|
        ((instruction&(0b1111<<8))>>7)|
        ((instruction&(0b1<<7))<<4);
        let mut wb_mux = 0;
        let mut alu_operand_a_sel = 0;
        let mut alu_operand_b_sel = 0;
        let mut regfile_rd = 0;
        let mut regfile_rs1 = 0;
        let mut regfile_rs2 = 0;
        let mut regfile_we = 0;
        let mut alu_operator = 0;
        let mut sign_zero_ext_sel = 0;
        let mut sign_zero_ext_data = 0;
        let mut imm_a_mux_data = 0;
       // let mut pc_mux_sel = 0;
       // let mut pc_se_data = 0;
        let mut data_mem_size = 0;
        let mut data_se = 0;
        let mut data_mem_ctrl = MemCtrl::None;
        let mut big_imm = 0;
        let mut pc_imm_sel = 0;
        //let mut branch_imm = 0;
        let mut branch_logic_ctl = 0;
        let mut branch_logic_enable = 0;
        let mut jalr_imm = 0;
        println!("INSTRUCTION:0x{:08x}", instruction);

        match opcode{
            0b0110011 => { //OP
                alu_operand_a_sel = 0; //rs1
                alu_operand_b_sel = 0; //rs2
                //rs1 [19:15] rs2 [24:20] rd [11:7]
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = (instruction & (0b11111<<15)) >> 15;
                regfile_rs2 = (instruction & (0b11111<<20)) >> 20;
                regfile_we = 1; //enable write
                wb_mux = 0; //ALU source

                match funct3{
                    0b000 => { // add/sub
                        match funct7{
                            0b0000000=> {alu_operator = 1;println!("ALU ADD")},//add
                            0b0100000=> {alu_operator = 2;println!("ALU SUB")},//sub
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }
                    }
                    0b001 => {
                        match funct7{ // sll
                            0b0000000=> {alu_operator = 3;println!("ALU SLL")},//sll
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    0b010 => {
                        match funct7{ // slt
                            0b0000000=> {alu_operator = 10;;println!("ALU SLT")},//slt
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    0b011 => {
                        match funct7{ // sltu
                            0b0000000=> {alu_operator = 9;println!("ALU SLTU")},//sltu
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    0b100 => {
                        match funct7{ // xor
                            0b0000000=> {alu_operator = 6;println!("ALU XOR")},//xor
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    0b101 => {
                        match funct7{ // srl
                            0b0000000=> {alu_operator = 4;println!("ALU SRL")},//srl
                            0b0100000=> {alu_operator = 5;println!("ALU SRA")}, //sra
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    0b110 => {
                        match funct7{ // or
                            0b0000000=> {alu_operator = 7;println!("ALU OR")},//or
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    0b111 => { //and
                        match funct7{ 
                            0b0000000=> {alu_operator = 8;println!("ALU AND")},//and
                            _=>panic!("Invalid funct7 {:b}", funct7),
                        }  
                    }
                    _ => {panic!("Invalid funct3 {:b}", funct3)}
                }

            }
            0b0010011 => { //OP_IMM
                alu_operand_a_sel = 0; //rs1
                alu_operand_b_sel = 1;  //imm
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = (instruction & (0b11111<<15)) >> 15;
                regfile_we = 1; //enable write
                wb_mux = 0; //ALU source

                match funct3{
                    
                    0b000=>{//ADDI
                        alu_operator = 1;
                        sign_zero_ext_sel = 0;
                        sign_zero_ext_data = imm;
                    }
                    0b010=>{//SLTI
                        alu_operator = 10;
                        sign_zero_ext_sel = 0;
                        sign_zero_ext_data = imm;
                    }
                    0b011=>{//SLTIU
                        alu_operator = 9;
                        sign_zero_ext_sel = 1;
                        sign_zero_ext_data = imm;
                    }
                    0b100=>{//XORI
                        alu_operator = 6;
                        sign_zero_ext_sel = 1;
                        sign_zero_ext_data = imm;
                    }
                    0b110=>{//ORI
                        alu_operator = 7;
                        sign_zero_ext_sel = 1;
                        sign_zero_ext_data = imm;
                    }
                    0b111=>{//ANDI
                        alu_operator = 8;
                        sign_zero_ext_sel = 1;
                        sign_zero_ext_data = imm;
                    }
                    0b001=>{//SLLI
                        alu_operator = 3;
                        sign_zero_ext_sel = 1;
                        sign_zero_ext_data = shamt;
                    }
                    0b101=>{//SRLI SRAI
                        match funct7{
                            0b0000000=>{
                                alu_operator = 4;
                                sign_zero_ext_sel = 1;
                                sign_zero_ext_data = shamt;
                            }//SRLI
                            0b0100000=>{
                                alu_operator = 5;
                                sign_zero_ext_sel = 1;
                                sign_zero_ext_data = shamt;
                            }//SRAI
                            _=>panic!("Invalid funct7! {:b}", funct7)
                        }
                    }
                    _=>{panic!("Invalid funct3! {:b}", funct3)},
                }
            }
            0b0110111 => {//LUI
                alu_operand_a_sel = 1; //big-imm
                alu_operand_b_sel = 1;  //imm
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = 0; //x0
                regfile_we = 1; //enable write
                wb_mux = 0; //ALU source
                alu_operator = 1; //ADD
                sign_zero_ext_data = 0; //add 0
                sign_zero_ext_sel = 1; //zero-extend
                imm_a_mux_data = imm_big;
            }
            0b0010111 => {//AUIPC
                alu_operand_a_sel = 1; //big-imm
                alu_operand_b_sel = 2;  //PC
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = 0; //x0
                regfile_we = 1; //enable write
                wb_mux = 0; //ALU source
                alu_operator = 1; //ADD
                sign_zero_ext_data = 0; //don't care
                sign_zero_ext_sel = 1; //don't care
                imm_a_mux_data = imm_big;
            }
            0b1101111 => {//JAL
                alu_operand_a_sel = 2; //0
                alu_operand_b_sel = 2;  //PC
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = 0; //x0 
                regfile_we = 1; //enable write
                wb_mux = 0; //ALU source
                alu_operator = 1; //ADD
                sign_zero_ext_data = 0; //don't care
                sign_zero_ext_sel = 1; //don't care
                big_imm = imm_big_shuffled;
                pc_imm_sel = 0;
                branch_logic_ctl = 0b010; //jal
                branch_logic_enable = 0b1;
            }
            0b1100111 =>{//JALR
                alu_operand_a_sel = 2; //0
                alu_operand_b_sel = 2;  //PC
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = (instruction & (0b11111<<15)) >> 15;
                regfile_we = 1; //enable write
                wb_mux = 0; //ALU source
                alu_operator = 1; //ADD
                sign_zero_ext_data = 0; //don't care
                sign_zero_ext_sel = 1; //don't care
                big_imm = imm_big_shuffled; //don't care
                pc_imm_sel = 0; //don't care
                branch_logic_ctl = 0b011; //jalr
                branch_logic_enable = 0b1;
                jalr_imm = imm;
                             
            }
            0b1100011 =>{//BRANCH
                regfile_rs1 = (instruction & (0b11111<<15)) >> 15;
                regfile_rs2 = (instruction & (0b11111<<20)) >> 20;
                pc_imm_sel = 1;
                //branch_imm = imm;
                regfile_rd = 0; //don't care
                regfile_we = 0; //no link
                wb_mux = 0; //don't care
                alu_operator = 0; //don't care
                sign_zero_ext_data = 0; //don't care
                big_imm = 0; //don't care
                pc_imm_sel = 1; //branch imm
                branch_logic_ctl = funct3; //use funct3
                branch_logic_enable = 0b1;  //enable branch logic
                println!("BRANCH IMM:{:b}", branch_imm);
            }
            
            0b0000011 =>{//LOAD
                println!("----------LOAD");
                alu_operand_a_sel = 0; //rs1
                alu_operand_b_sel = 1;   //imm
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = (instruction & (0b11111<<15)) >> 15;
                regfile_we = 1;
                wb_mux = 1; //data memory
                alu_operator = 1;//ADD
                sign_zero_ext_data = imm; //immediate
                sign_zero_ext_sel = 0; //sign extend

                data_mem_ctrl = MemCtrl::Read;
                match(funct3){
                    0b000=>{data_mem_size=1;data_se=1}, //lb
                    0b001=>{data_mem_size=2;data_se=1}, //lh
                    0b010=>{data_mem_size=4;data_se=1}, //lw
                    0b100=>{data_mem_size=1;data_se=0}, //lbu
                    0b101=>{data_mem_size=2;data_se=0}, //lhu
                    _=>{panic!("Unsupported funct3 {:b}", funct3)},
                }


            }
            0b0100011 =>{//STORE
                println!("----------STORE");
                alu_operand_a_sel = 0; //rs1
                alu_operand_b_sel = 1;   //imm
                regfile_rd = (instruction & (0b11111<<7)) >> 7;
                regfile_rs1 = (instruction & (0b11111<<15)) >> 15;
                regfile_rs2 = (instruction & (0b11111<<20)) >> 20;
                regfile_we = 0;
                wb_mux = 0; //don't care
                alu_operator = 1;//ADD
                sign_zero_ext_data = imm_store; //immediate store type
                sign_zero_ext_sel = 0; //sign extend

                data_mem_ctrl = MemCtrl::Write;
                match(funct3){//size
                    0b000=>{data_mem_size=1;println!("BYTE")},
                    0b001=>{data_mem_size=2;println!("HALFWORD")},
                    0b010=>{data_mem_size=4;println!("WORD")},
                    _=>panic!("Unsupported funct3 {:b}", funct3),
                }
            }
            _ => {panic!("Invalid opcode! {:b}", opcode);}
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
       //simulator.set_out_val(&self.id, "pc_se_data", pc_se_data);
       //simulator.set_out_val(&self.id, "pc_mux_sel", pc_mux_sel);
        simulator.set_out_val(&self.id, "data_mem_size", data_mem_size);
        simulator.set_out_val(&self.id, "data_se", data_se);
        simulator.set_out_val(&self.id, "data_mem_ctrl", data_mem_ctrl as Signal);
        simulator.set_out_val(&self.id, "big_imm", big_imm);
        simulator.set_out_val(&self.id,"pc_imm_sel",pc_imm_sel);
        simulator.set_out_val(&self.id, "branch_imm", branch_imm);
        simulator.set_out_val(&self.id, "branch_logic_ctl", branch_logic_ctl);
        simulator.set_out_val(&self.id, "branch_logic_enable", branch_logic_enable);
        simulator.set_out_val(&self.id, "jalr_imm", jalr_imm);


    }
}
