use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

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
            _ => {panic!("Invalid opcode! {:b}", opcode)}
        }

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



    }
}
