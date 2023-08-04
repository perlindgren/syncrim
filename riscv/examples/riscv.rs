use clap::Parser;
// An example MIPS model
use fern;
use riscv::components::*;
use riscv_elf_parse;
use std::{
    cell::RefCell, collections::BTreeMap, fs, ops::Range, path::PathBuf, process::Command, rc::Rc,
};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};
use xmas_elf::ElfFile;

#[derive(Parser, Debug)]
struct Args {
    /// Use a pre-compiled elf file instead of compiling one
    #[arg(short, long, default_value = "false")]
    use_elf: bool,
    /// Path to the pre-compiled elf file
    #[arg(short, long, default_value = "")]
    elf_path: String,
    /// Path to the assembly source file
    #[arg(short, long, default_value = "asm.s")]
    asm_path: String,
    /// Path to the linker script
    #[arg(short, long, default_value = "memory.x")]
    ls_path: String,
}

fn main() {
    fern_setup_riscv();
    let args = Args::parse();
    let memory = if !args.use_elf {
        elf_from_asm(&args);
        let bytes = fs::read("./output").expect("The elf file could not be found");
        let elf = ElfFile::new(&bytes).unwrap();
        riscv_elf_parse::Memory::new_from_elf(elf)
    } else {
        let bytes =
            fs::read(format!("{}", args.elf_path)).expect("The elf file could not be found");
        let elf = ElfFile::new(&bytes).unwrap();
        riscv_elf_parse::Memory::new_from_elf(elf)
    };

    println!("{}", memory);
    let mut instr_mem = BTreeMap::new();
    let mut data_mem = BTreeMap::new();

    //init data memory with 0's
    let range = Range {
        start: 0x8000_0000u32,
        end: 0x8000_1000u32,
    };
    for address in range.clone() {
        data_mem.insert(address as usize, 0);
    }
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
                bytes: instr_mem,
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
                registers: RegStore::new(Rc::new(RefCell::new([0; 32]))),
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
                range,
            ),
            // Mem::rc_new_from_bytes(
            //     "data_memory",
            //     (700.0, 600.0),
            //     100.0,
            //     100.0,
            //     false,
            //     Input::new("reg_file", "reg_b"),
            //     Input::new("alu", "result_o"),
            //     Input::new("decoder", "data_mem_ctrl"),
            //     Input::new("decoder", "data_se"),
            //     Input::new("decoder", "data_mem_size"),
            //     data_mem,
            // ),
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
                    Input::new("reg", "out"),
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
        //.level_for("syncrim::components::mem", LevelFilter::Trace)
        //.level_for("riscv::components::instr_mem", LevelFilter::Trace)
        .level_for("syncrim::gui_vizia::components::mem", LevelFilter::Trace)
        .level_for("riscv::gui_vizia::components::reg_file", LevelFilter::Trace)
        //.level_for("riscv::components::alu", LevelFilter::Trace);
        .level_for("syncrim::components::mem", LevelFilter::Trace);

    f
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply()
        .unwrap()
}

fn elf_from_asm(args: &Args) {
    let source_path = &args.asm_path;
    let linker_path = &args.ls_path;
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(".\\")
            .args(["/C", &format!("copy {} .\\riscv_asm\\asm.s", source_path)])
            .status()
            .unwrap()
    } else {
        Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(format!("cp {} ./riscv_asm/asm.s", source_path))
            .status()
            .unwrap()
    };
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(".\\")
            .args([
                "/C",
                &format!("copy {} .\\riscv_asm\\memory.x", linker_path),
            ])
            .status()
            .unwrap()
    } else {
        Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(format!("cp {} ./riscv_asm/memory.x", linker_path))
            .status()
            .unwrap()
    };
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(".\\riscv_asm\\")
            .args(["/C", "cargo build --release"])
            .status()
            .unwrap()
    } else {
        Command::new("sh")
            .current_dir("./riscv_asm/")
            .arg("-c")
            .arg(format!("cargo build --release"))
            .status()
            .unwrap()
    };
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(".\\riscv_asm\\")
            .args([
                "/C",
                "move /y .\\target\\riscv32i-unknown-none-elf\\release\\riscv_asm ..\\output",
            ])
            .status()
            .unwrap()
    } else {
        Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(format!(
                "mv ./riscv_asm/target/riscv32i-unknown-none-elf/release/riscv_asm ./output"
            ))
            .status()
            .unwrap()
    };
}
