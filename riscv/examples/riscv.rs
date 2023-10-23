use clap::Parser;
// An example MIPS model
use fern;
use riscv::components::*;
use riscv_elf_parse;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    ops::Range,
    path::PathBuf,
    process::Command,
    rc::Rc,
};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

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
    #[arg(short, long, default_value = "false")]
    rust: bool,
}

fn main() {
    fern_setup_riscv();
    let args = Args::parse();
    let memory = if !args.use_elf && !args.rust {
        elf_from_asm(&args);
        let bytes = fs::read("./output").expect("The elf file could not be found");
        riscv_elf_parse::Memory::new_from_file(&bytes, false)
    } else if args.use_elf && !args.rust {
        let bytes =
            fs::read(format!("{}", args.elf_path)).expect("The elf file could not be found");
        riscv_elf_parse::Memory::new_from_file(&bytes, false)
    } else {
        compile_rust_crate();
        let bytes = fs::read("./output").expect("The elf file could not be found");
        riscv_elf_parse::Memory::new_from_file(&bytes, false)
    };

    let mut instr_mem = BTreeMap::new();
    let mut data_mem = BTreeMap::new();
    let mut breakpoints = HashSet::new();

    breakpoints.insert(0x0000_0008);

    //init data memory with 0's
    let range = Range {
        start: 0x5000_0000u32,
        end: 0x5000_0500u32,
    };
    let instr_range = Range {
        start: 0x0000_0000usize,
        end: 0x0000_2000usize,
    };
    for address in range.clone() {
        data_mem.insert(address as usize, 0);
    }
    for address in instr_range.clone() {
        instr_mem.insert(address as usize, 0);
    }
    for element in memory.bytes {
        if element.0 < 0x5000_0000 {
            instr_mem.insert(element.0, element.1);
        } else {
            data_mem.insert(element.0, element.1);
        }
    }
    /*
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
                    height: BRANCH_LOGIC_HEIGHT,
                    width: BRANCH_LOGIC_WIDTH,
                    id: "branch_logic".to_string(),
                    pos: (725.0, 300.0),
                    rs1: Input::new("reg_file", "reg_a"),
                    rs2: Input::new("reg_file", "reg_b"),
                    ctrl: Input::new("decoder", "branch_logic_ctl"),
                    enable: Input::new("decoder", "branch_logic_enable"),
                }),
                Rc::new(LSBZero {
                    height: LSB_ZERO_HEIGHT,
                    width: LSB_ZERO_WIDTH,
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
                    height: INSTR_MEM_HEIGHT,
                    width: INSTR_MEM_WIDTH,
                    id: "instr_mem".to_string(),
                    pos: (180.0, 400.0),
                    pc: Input::new("reg", "out"),
                    bytes: instr_mem,
                    range: instr_range,
                    breakpoints: Rc::new(RefCell::new(breakpoints)),
                }),
                Rc::new(Decoder {
                    height: DECODER_HEIGHT,
                    width: DECODER_WIDTH,
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
                    height: SIGN_ZERO_EXT_HEIGHT,
                    width: SIGN_ZERO_EXT_WIDTH,
                    id: "imm_szext".to_string(),
                    pos: (450.0, 1000.0),
                    data_i: Input::new("decoder", "sign_zero_ext_data"),
                    sel_i: Input::new("decoder", "sign_zero_ext_sel"),
                }),
                Register::rc_new("wb_reg", (100.0, 140.0), Input::new("wb_mux", "out")),
                Rc::new(RegFile {
                    id: "reg_file".into(),
                    pos: (450.0, 150.0),
                    width: REG_FILE_WIDTH,
                    height: REG_FILE_HEIGHT,
                    read_addr1: Input::new("decoder", "regfile_rs1"),
                    read_addr2: Input::new("decoder", "regfile_rs2"),
                    write_data: Input::new("wb_reg", "out"),
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
    */
    let path = PathBuf::from("riscv.json");
    let mut cs = ComponentStore::load_file(&path);
    /*cs.store.push(Mux::rc_new(
        "mmio_data_mux",
        (750.0, 300.0),
        Input::new("decoder", "csr_data_mux"),
        vec![
            Input::new("reg_file", "reg_a"),
            Input::new("decoder", "csr_data"),
        ],
    ));
    cs.store.push(RVMem::rc_new_from_bytes(
        "data_memory",
        (1540.0, 900.0),
        100.0,
        100.0,
        false,
        Input::new("reg_file", "reg_b"),
        Input::new("alu", "result_o"),
        Input::new("decoder", "data_mem_ctrl"),
        Input::new("decoder", "data_se"),
        Input::new("decoder", "data_mem_size"),
        Input::new("clic", "mem_int_addr"),
        data_mem,
        range,
    ));
    cs.store.push(Rc::new(InstrMem {
        height: INSTR_MEM_HEIGHT,
        width: INSTR_MEM_WIDTH,
        id: "instr_mem".to_string(),
        pos: (650.0, 900.0),
        pc: Input::new("reg", "out"),
        bytes: instr_mem,
        range: instr_range,
        breakpoints: Rc::new(RefCell::new(breakpoints)),
        symbols: memory.symbols,
        le: true,
    }));
    cs.store.push(Rc::new(CLIC::new(
        "clic".to_string(),
        (300.0, 500.0),
        100.0,
        100.0,
        Input::new("reg_file", "reg_b"),        //MMIO data
        Input::new("alu", "result_o"),          //MMIO address
        Input::new("decoder", "data_mem_ctrl"), //R/W for MMIO
        Input::new("decoder", "data_mem_size"), //size for MMIO
        Input::new("csr_mux", "out"),           //Immediate or register data for CSR op
        Input::new("decoder", "csr_addr"),      //CSR address
        Input::new("decoder", "csr_ctl"),       //CSR op
        Input::new("decoder", "mret"),          //mret signal
        Input::new("pc_adder", "out"),          //mepc
    )));
    cs.store.push(Mux::rc_new(
        "csr_mux",
        (650.0, 300.0),
        Input::new("decoder", "csr_data_mux"),
        vec![
            Input::new("reg_file", "reg_a"),
            Input::new("decoder", "csr_data"),
        ],
    ));
    cs.store.push(Mux::rc_new(
                "pc_adder_mux",
                (400.0, 740.0),
                Input::new("branch_logic", "out"),
                vec![
                    Input::new("pc_adder", "out"),
                    Input::new("jalr_stripper", "out"),
                    Input::new("branch_adder", "out"),
                    Input::new("data_memory", "isr_addr"),
                    Input::new("clic", "mepc")
                ],
            ));*/
    //let path = PathBuf::from("riscv.json");
    //cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    {
        let dummy = Input::new("id", "field");
        let lib = ComponentStore {
            store: vec![
                Rc::new(InstrMem {
                    width: INSTR_MEM_WIDTH,
                    height: INSTR_MEM_HEIGHT,
                    id: "dummy_instr_mem".to_string(),
                    pos: (0.0, 0.0),
                    pc: dummy.clone(),
                    bytes: BTreeMap::new(),
                    range: Range {
                        start: 0,
                        end: 0x1000,
                    },
                    breakpoints: Rc::new(RefCell::new(HashSet::new())),
                    symbols: HashMap::new(),
                    le: true,
                }),
                Rc::new(ALU {
                    id: "dummy_alu".to_string(),
                    pos: (0.0, 0.0),
                    operator_i: dummy.clone(),
                    operand_a_i: dummy.clone(),
                    operand_b_i: dummy.clone(),
                }),
                Rc::new(BranchLogic {
                    width: BRANCH_LOGIC_WIDTH,
                    height: BRANCH_LOGIC_HEIGHT,
                    id: "dummy_blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: dummy.clone(),
                    rs2: dummy.clone(),
                    ctrl: dummy.clone(),
                    enable: dummy.clone(),
                    int: dummy.clone(),
                    mret: dummy.clone(),
                }),
                Rc::new(Decoder {
                    width: DECODER_WIDTH,
                    height: DECODER_HEIGHT,
                    id: "dummy_decoder".to_string(),
                    pos: (0.0, 0.0),
                    instruction: dummy.clone(),
                }),
                Rc::new(LSBZero {
                    height: LSB_ZERO_HEIGHT,
                    width: LSB_ZERO_WIDTH,
                    id: "dummy_lsbzero".to_string(),
                    pos: (0.0, 0.0),
                    data_i: dummy.clone(),
                }),
                Rc::new(RegFile {
                    id: "dummy_reg_file".into(),
                    pos: (0.0, 0.0),
                    width: REG_FILE_WIDTH,
                    height: REG_FILE_HEIGHT,
                    read_addr1: dummy.clone(),
                    read_addr2: dummy.clone(),
                    write_data: dummy.clone(),
                    write_addr: dummy.clone(),
                    write_enable: dummy.clone(),
                    registers: RegStore::new(Rc::new(RefCell::new([0; 32]))),
                    history: RegHistory::new(),
                }),
                Rc::new(SZExt {
                    height: SIGN_ZERO_EXT_HEIGHT,
                    width: SIGN_ZERO_EXT_WIDTH,
                    id: "dummy_szext".to_string(),
                    pos: (0.0, 0.0),
                    data_i: dummy.clone(),
                    sel_i: dummy.clone(),
                }),
            ],
        };
        let mut component_vec = lib.store.clone();
        component_vec.append(&mut syncrim::gui_egui::editor::Library::default().0.clone());
        syncrim::gui_egui::gui(cs, &path, syncrim::gui_egui::editor::Library(component_vec));
    }

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
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
        .level_for(
            "riscv::components::clic",
            // "riscv::gui_vizia::components::instr_mem",
            log::LevelFilter::Trace,
        )
        .level_for("riscv::components::branch_logic", log::LevelFilter::Trace)
        .level(log::LevelFilter::Error);

    // - and per-module overrides
    #[cfg(feature = "gui-vizia")]
    let f = f
        //.level_for("riscv::components::instr_mem", LevelFilter::Trace)
        //.level_for("riscv::components::clic", LevelFilter::Trace)
        //.level_for("riscv::components::mem", LevelFilter::Trace)
        //.level_for("syncrim::simulator", LevelFilter::Trace)
        .level_for("riscv::gui_egui::components::instr_mem", LevelFilter::Trace);

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
        match Command::new("cmd")
            .current_dir(".\\riscv_asm\\")
            .args(["/C", "cargo build --release"])
            .status()
        {
            Ok(_) => {}
            Err(_) => {
                panic!("cargo build unsuccessful")
            }
        }
    } else {
        match Command::new("sh")
            .current_dir("./riscv_asm/")
            .arg("-c")
            .arg(format!("cargo build --release"))
            .status()
        {
            Ok(exit_status) => match exit_status.success() {
                true => {}
                false => {
                    panic!("cargo build unsuccessful")
                }
            }, //25856
            Err(_) => {
                panic!()
            }
        }
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

fn compile_rust_crate() {
    let _ = if cfg!(target_os = "windows") {
        match Command::new("cmd")
            .current_dir(".\\riscv_basic\\")
            .args(["/C", "cargo build --release"])
            .status()
        {
            Ok(_) => {}
            Err(_) => {
                panic!("cargo build unsuccessful")
            }
        }
    } else {
        match Command::new("sh")
            .current_dir("./riscv_basic/")
            .arg("-c")
            .arg(format!("cargo build --release"))
            .status()
        {
            Ok(exit_status) => match exit_status.success() {
                true => {}
                false => {
                    panic!("cargo build unsuccessful")
                }
            }, //25856
            Err(_) => {
                panic!()
            }
        }
    };
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(".\\riscv_basic\\")
            .args([
                "/C",
                "move /y .\\target\\riscv32i-unknown-none-elf\\release\\riscv_basic ..\\output",
            ])
            .status()
            .unwrap()
    } else {
        Command::new("sh")
            .current_dir("./")
            .arg("-c")
            .arg(format!(
                "mv ./riscv_basic/target/riscv32i-unknown-none-elf/release/riscv_basic ./output"
            ))
            .status()
            .unwrap()
    };
}

// fn dump_file(object: &object::File, endian: gimli::RunTimeEndian) -> Result<(), gimli::Error> {
//     // Load a section and return as `Cow<[u8]>`.
//     let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
//         match object.section_by_name(id.name()) {
//             Some(ref section) => Ok(section
//                 .uncompressed_data()
//                 .unwrap_or(borrow::Cow::Borrowed(&[][..]))),
//             None => Ok(borrow::Cow::Borrowed(&[][..])),
//         }
//     };

//     // Load all of the sections.
//     let dwarf_cow = gimli::Dwarf::load(&load_section)?;

//     // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
//     let borrow_section: &dyn for<'a> Fn(
//         &'a borrow::Cow<[u8]>,
//     ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
//         &|section| gimli::EndianSlice::new(&*section, endian);

//     // Create `EndianSlice`s for all of the sections.
//     let dwarf = dwarf_cow.borrow(&borrow_section);

//     // Iterate over the compilation units.
//     let mut iter = dwarf.units();
//     while let Some(header) = iter.next()? {
//         println!(
//             "Unit at <.debug_info+0x{:x}>",
//             header.offset().as_debug_info_offset().unwrap().0
//         );
//         let unit = dwarf.unit(header)?;

//         // Iterate over the Debugging Information Entries (DIEs) in the unit.
//         let mut depth = 0;
//         let mut entries = unit.entries();
//         while let Some((delta_depth, entry)) = entries.next_dfs()? {
//             depth += delta_depth;
//             println!("<{}><{:x}> {}", depth, entry.offset().0, entry.tag());

//             // Iterate over the attributes in the DIE.
//             let mut attrs = entry.attrs();
//             while let Some(attr) = attrs.next()? {
//                 println!("   {}: {:?}", attr.name(), attr.value());
//             }
//         }
//     }
//     Ok(())
// }
