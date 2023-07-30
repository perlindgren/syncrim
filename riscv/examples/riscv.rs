use clap::Parser;
// An example MIPS model
use fern;
use riscv::components::*;
use riscv_elf_parse;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    rc::Rc,
};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to source file
    #[arg(short, long, default_value = "riscv32-unknown-elf-")]
    toolchain_prefix: String,
}

fn main() {
    fern_setup_riscv();
    let args = Args::parse();
    let memory =
        riscv_elf_parse::Memory::new_from_assembly("asm.s", "memory.x", &args.toolchain_prefix);
    println!("{}", memory);
    let mut instr_mem = BTreeMap::new();
    let mut data_mem = HashMap::new();
    for element in memory.bytes {
        if element.0 < 0x5000_0000 {
            instr_mem.insert(element.0, element.1);
        } else {
            data_mem.insert(element.0, element.1);
        }
    }
    let cs = ComponentStore {
        store: vec![
            Add::rc_new(
                "pc_adder",
                (150.0, 120.0),
                Input::new("pc_adder_c", "out"),
                Input::new("reg", "out"),
            ),
            Constant::rc_new("pc_adder_c", (100.0, 100.0), 4),
            Register::rc_new("reg", (100.0, 140.0), Input::new("pc_adder_mux", "out")),
            Mux::rc_new(
                "pc_adder_mux",
                (100.0, 120.0),
                Input::new("branch_logic", "out"),
                vec![
                    Input::new("pc_adder", "out"),
                    Input::new("jalr_stripper", "out"),
                    Input::new("branch_adder", "out"),
                ],
            ),
            Add::rc_new(
                "jalr_adder",
                (100.0, 200.0),
                Input::new("reg_file", "reg_a"),
                Input::new("jalr_se", "out"),
            ),
            Rc::new(BranchLogic {
                id: "branch_logic".to_string(),
                pos: (725.0, 300.0),
                rs1: Input::new("reg_file", "reg_a"),
                rs2: Input::new("reg_file", "reg_b"),
                ctrl: Input::new("decoder", "branch_logic_ctl"),
                enable: Input::new("decoder", "branch_logic_enable"),
            }),
            Rc::new(LSBZero {
                id: "jalr_stripper".to_string(),
                pos: (600.0, 1000.0),
                data_i: Input::new("jalr_adder", "out"),
            }),
            Sext::rc_new(
                "jalr_se",
                (900.0, 900.0),
                Input::new("decoder", "jalr_imm"),
                12,
                32,
            ),
            Mux::rc_new(
                "branch_adder_mux",
                (500.0, 1000.0),
                Input::new("decoder", "pc_imm_sel"),
                vec![
                    Input::new("jal_imm_sext", "out"),
                    Input::new("branch_imm_sext", "out"),
                ],
            ),
            Add::rc_new(
                "branch_adder",
                (50.0, 400.0),
                Input::new("reg", "out"),
                Input::new("branch_adder_mux", "out"),
            ),
            Sext::rc_new(
                "jal_imm_sext",
                (500.0, 1000.0),
                Input::new("decoder", "big_imm"),
                21,
                32,
            ),
            Sext::rc_new(
                "branch_imm_sext",
                (500.0, 1000.0),
                Input::new("decoder", "branch_imm"),
                13,
                32,
            ),
            Rc::new(InstrMem {
                id: "instr_mem".to_string(),
                pos: (180.0, 400.0),
                pc: Input::new("reg", "out"),
                // fake instructions just to show the relation between input address and instruction
                bytes: instr_mem,
                //vec![
                //The results are calculated from the reg file start state defined above
                //and assuming the blocks are the only code over that reg file.
                //OP TEST BLOCK
                // 0x003100b3,//add x1, x2, x3 -> x1 = 0x5
                // 0x00308133,//add x2, x1, x3 -> x2 = 0x8
                // 0x40410133,//sub x2, x2, x4 -> x2 = -2
                // 0x004121b3,//slt x3, x2, x4 -> x3 = 0x1 //signed -2<10
                // 0x004131b3,//sltu x3, x2, x4 x3 = 0x0 //unsigned -2>10
                // 0x001151b3,//srl x3, x2, x1 # x3 = 0x07ffffff
                // 0x401151b3,//sra x3, x2, x1 # x3 = 0xffffffff
                // 0x001111b3,//sll x3, x2, x1 # x3 = 0xffffffc0
                // 0x0020c1b3,//xor x3, x1, x2 # x3 = 0xfffffffb //fel här
                // 0x0020f1b3,//and x3, x1, x2 # x3 = 0x00000004
                // 0x0060e1b3,//or x3, x1, x6 #  x3 = 0x00000007
                // 0x00000033,//add x0, x0, x0, basically nop before panicking so we can see result.
                // 0x00940023,//sb x8, 0(x9) # should panic over opcode for now
                //OP_IMM, AUIPC, LUI, STORE, LOAD, OP_IMM TEST BLOCK
                // 0x00310093,//addi x1, x2, 3 # x1=0x5
                // 0xffd0a093,//slti x1, x1, -3 # x1=0x0
                // 0x0030a093,//slti x1, x1, 3 # x1=0x1
                // 0xffd0b093,//sltiu x1, x1, -3 #x1=0x1
                // 0x00313093,//sltiu x1, x2, 3 #x1=0x1
                // 0x00324093,//xori x1, x4, 3 #x1 = 0x9
                // 0x00326093,//ori x1, x4, 3 #x1=0xb
                // 0x00327093,//andi x1, x4, 3 #x1=0x2
                // 0x00c19093,//slli x1, x3, 12 #x1=0x3000
                // 0x0011d093,//srli x1, x3, 1 #x1=0x1
                // 0xffa00093,//addi x1, x0, -6 #x1=0xfffffffa
                // 0x4020d093,//srai x1, x1, 2 #x1=0xfffffffe
                // 0x00500093,//addi x1, x0, 5 #x1=0x5
                // 0x4020d093,//srai x1, x1, 2 #x1=0x1
                // 0xfffff0b7,//lui x1, 0xFFFFF #x1=0xFFFFF000
                // 0xfffff097,//auipc x1, 0xFFFFF #x1=0xFFFFF040
                // 0x00000093,//addi, x1, x0, 0 x1 = 0
                // 0x00408093,//addi x1, x1, 4 x1+=4
                // //0xff9ff16f,//jal, x2, -8 should jump to the addi before and keep incrementing x1.
                // 0x00000033,//add x0, x0, x0, basically nop before panicking so we can see result.
                // //0x00940023,//sb x8, 0(x9) # should panic over opcode for now
                // 0x00102023,//sw x1, 0(x0) store x1=4 at 0
                // 0x00002283,//lw x5, 0(x0) x5=4
                // 0x00228213,//addi x4, x5, 2 //x4=6
                // 0x00002303, //lw x6, 0(x0), x6 = 4
                // 0xfff00093,//set x1 to -1 addi x1, x0, -1
                // 0x00102023,//store -1 at 0
                // 0x00000203,//load via lb -1 to x4
                // 0x00004203,//lbu x4 -1 again.
                // 0x004002a3,//sb x4, 5(0)
                // 0x00000033,//add x0, x0, x0, basically nop before panicking so we can see result.
                // 0x00000033,//add x0, x0, x0, basically nop before panicking so we can see result.
                //JAL, JALR, BRANCHES TEST BLOCK
                //     0x00000093, //addi, x1, x0, 0 x1=0
                //     0x0080016f, //jal x2, 8
                //     0x0000016f, //jal x2, 0 this is to be jumped over or we will get stuck
                //     0x00410167, //jalr x2, x2, 4
                //     0x0000016f, //jal x2, 0 this is to be jumped over or we will get stuck
                //     0x01000113, //addi x2, x0, 16
                //     0x00000093, //addi x1, x0, 0
                //     0x00408093, //addi x1, x1, 4
                //     0xfe209ee3, //bne x1, x2, -4
                //     0x00208463, //beq, x1, x2, 8
                //     0x0000006f, //jal x0, 0
                //     0xff800093, //addi x1, x0, -8
                //     0x00000113, //addi x2, x0, 0
                //     0x00408093, //addi x1, x1, 4
                //     0xfe20cee3, //blt x1, x2, -4
                //     0xff800093, //addi x1, x0, -8
                //     0x00000113, //addi x2, x0, 0
                //     0x00408093, //addi x1, x1, 4
                //     0xfe116ee3, //bltu, x2, x1, -4
                //     0xff800093, //addi x1, x0, -8
                //     0x00000113, //addi x2, x0, 0
                //     0x00408093, //addi x1, x1, 4
                //     0xfe115ee3, //bge x2, x1, -4
                //     0xff800093, //addi x1, x0, -8
                //     0x00800113, //addi x2, x0, 8
                //     0x00408093, //addi x1, x1, 4
                //     0xfe20fee3, //bgeu x1, x2, -4
                //     0x00408093, 0x00408093, 0x00408093, 0x00408093, 4, 5, 6, 7, 8, 9,
                // ],
            }),
            Rc::new(Decoder {
                id: "decoder".to_string(),
                pos: (300.0, 150.0),
                instruction: Input::new("instr_mem", "instruction"),
            }),
            Register::rc_new(
                "regfile_we_reg",
                (450.0, 50.0),
                Input::new("decoder", "regfile_we"),
            ),
            Register::rc_new(
                "regfile_rd_reg",
                (480.0, 50.0),
                Input::new("decoder", "regfile_rd"),
            ),
            Rc::new(SZExt {
                id: "imm_szext".to_string(),
                pos: (450.0, 1000.0),
                data_i: Input::new("decoder", "sign_zero_ext_data"),
                sel_i: Input::new("decoder", "sign_zero_ext_sel"),
            }),
            Rc::new(RegFile {
                id: "reg_file".into(),
                pos: (450.0, 150.0),
                width: 100.0,
                height: 100.0,
                read_addr1: Input::new("decoder", "regfile_rs1"),
                read_addr2: Input::new("decoder", "regfile_rs2"),
                write_data: Input::new("wb_mux", "out"),
                write_addr: Input::new("regfile_rd_reg", "out"),
                write_enable: Input::new("regfile_we_reg", "out"),
                registers: RegStore::new(Rc::new(RefCell::new([
                    0, 1, 2, 3, 10, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                ]))),
                // registers: vec![Cell::new(0),
                // RefCell::new(1),
                // Cell::new(2),
                // Cell::new(3),
                // Cell::new(10),
                // Cell::new(5),
                // Cell::new(6),
                // Cell::new(7),
                // Cell::new(8),
                // Cell::new(9),
                // Cell::new(10),
                // Cell::new(11),
                // Cell::new(12),
                // Cell::new(13),
                // Cell::new(14),
                // Cell::new(15),
                // Cell::new(16),
                // Cell::new(17),
                // Cell::new(18),
                // Cell::new(19),
                // Cell::new(20),
                // Cell::new(21),
                // Cell::new(22),
                // Cell::new(23),
                // Cell::new(24),
                // Cell::new(25),
                // Cell::new(26),
                // Cell::new(27),
                // Cell::new(28),
                // Cell::new(29),
                // Cell::new(30),
                // Cell::new(31),
                // ],
                history: RegHistory::new(),
            }),
            Mem::rc_new_from_bytes(
                "data_memory",
                (700.0, 600.0),
                100.0,
                100.0,
                false,
                Input::new("reg_file", "reg_b"),
                Input::new("alu", "result_o"),
                Input::new("decoder", "data_mem_ctrl"),
                Input::new("decoder", "data_se"),
                Input::new("decoder", "data_mem_size"),
                data_mem,
            ),
            Constant::rc_new("zero_c", (680.0, 150.0), 0),
            Mux::rc_new(
                "alu_operand_a_mux",
                (700.0, 150.0),
                Input::new("decoder", "alu_operand_a_sel"),
                vec![
                    Input::new("reg_file", "reg_a"),
                    Input::new("decoder", "imm_a_mux_data"),
                    Input::new("zero_c", "out"),
                ],
            ),
            Mux::rc_new(
                "alu_operand_b_mux",
                (700.0, 300.0),
                Input::new("decoder", "alu_operand_b_sel"),
                vec![
                    Input::new("reg_file", "reg_b"),
                    Input::new("imm_szext", "out"),
                    Input::new("pc_adder", "out"),
                ],
            ),
            Rc::new(ALU {
                id: "alu".to_string(),
                pos: (800.0, 225.0),
                operator_i: Input::new("decoder", "alu_operator"),
                operand_a_i: Input::new("alu_operand_a_mux", "out"),
                operand_b_i: Input::new("alu_operand_b_mux", "out"),
            }),
            Mux::rc_new(
                "wb_mux",
                (900.0, 225.0),
                Input::new("decoder", "wb_mux"),
                vec![
                    Input::new("alu", "result_o"),
                    Input::new("data_memory", "data"),
                ],
            ),
        ],
    };

    let path = PathBuf::from("riscv.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
#[allow(unused_imports)]
use log::LevelFilter;
fn fern_setup_riscv() {
    let f = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        // .level(log::LevelFilter::Debug);
        .level(log::LevelFilter::Warn);

    // - and per-module overrides
    #[cfg(feature = "gui-vizia")]
    let f = f
        .level_for("syncrim::components::mem", LevelFilter::Trace)
        .level_for("riscv::components::instr_mem", LevelFilter::Trace);
    //.level_for("riscv", LevelFilter::Trace);

    f
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply()
        .unwrap()
}
