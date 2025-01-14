use clap::Parser;
// An example MIPS model
use fern;
use riscv::components::*;
use riscv_elf_parse;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    fs,
    ops::Range,
    path::PathBuf,
    process::Command,
    rc::Rc,
};
use syncrim::common::{ComponentStore, Input};

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
        end: 0x5000_2000u32,
    };
    let instr_range = Range {
        start: 0x0000_0000usize,
        end: 0x0000_1000usize,
    };
    for address in range.clone() {
        data_mem.insert(address as usize, 0);
    }
    for address in instr_range.clone() {
        instr_mem.insert(address as usize, 0);
    }
    for element in memory.bytes.clone() {
        if element.0 < 0x5000_0000 {
            instr_mem.insert(element.0, element.1);
        } else {
            data_mem.insert(element.0, element.1);
        }
    }
    let path = PathBuf::from("riscv.json");
    let mut cs = ComponentStore::load_file(&path);
    let mut i = 0;
    let mut store = cs.store.clone();
    for component in store.clone() {
        // if the component is a data memory
        if component.get_id_ports().0 == "data_memory" {
            // pull the trait object from the Component vector
            let comp = store.remove(i);
            // and downcast it to an InstrMem, own by cloning
            let mut data_mem_comp: RVMem = comp
                .as_any()
                .downcast_ref::<RVMem>()
                .expect(&format!("Downcast failed for {:?}", comp.to_()))
                .clone();
            // replace the memory contents with ELF contents
            data_mem_comp.memory = Memory::new(data_mem.clone());
            // also, set the initial state for reset
            data_mem_comp.init_state = data_mem;
            // repush the mutated RVMem to the Component vector
            store.push(Rc::new(data_mem_comp));
            //satisfy borrow checker
            break;
        }
        i += 1
    }
    let mut i = 0;
    for component in store.clone() {
        // if the component is an instr mem
        if component.get_id_ports().0 == "instr_mem" {
            // pull the trait object from the Component vector
            let comp = store.remove(i);
            // and downcast it to an InstrMem, own by cloning
            let mut instr_mem_comp: InstrMem = comp
                .as_any()
                .downcast_ref::<InstrMem>()
                .expect(&format!("Downcast failed for {:?}", comp.to_()))
                .clone();
            // replace the memory contents with ELF contents
            instr_mem_comp.bytes = instr_mem;
            // replace the symbols with ELF symbols
            instr_mem_comp.symbols = memory.symbols;
            // repush the mutated InstrMem to the Component vector
            store.push(Rc::new(instr_mem_comp));
            //satisfy borrow checker
            break;
        }
        i += 1
    }
    cs.store = store;
    #[cfg(feature = "gui-egui")]
    {
        use std::collections::HashMap;
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
                    // int: dummy.clone(),
                    // mret: dummy.clone(),
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
                Rc::new(RegFile::dummy()),
                Rc::new(SZExt {
                    height: SIGN_ZERO_EXT_HEIGHT,
                    width: SIGN_ZERO_EXT_WIDTH,
                    id: "dummy_szext".to_string(),
                    pos: (0.0, 0.0),
                    data_i: dummy.clone(),
                    sel_i: dummy.clone(),
                }),
                Rc::new(WBCtl {
                    height: WB_CTL_HEIGHT,
                    width: WB_CTL_WIDTH,
                    id: "dummy_wbctl".to_string(),
                    pos: (0.0, 0.0),
                    clic_i: dummy.clone(),
                    dec_i: dummy.clone(),
                }),
                Rc::new(GPIO {
                    height: GPIO_HEIGHT,
                    width: GPIO_WIDTH,
                    id: "dummy_gpio".to_string(),
                    pos: (0.0, 0.0),
                    data_i: dummy.clone(),
                    addr_i: dummy.clone(),
                    size_i: dummy.clone(),
                    we_i: dummy.clone(),
                    se_i: dummy.clone(),
                    csr_d: dummy.clone(),
                    csr_a: dummy.clone(),
                    csr_ctl: dummy.clone(),
                    csrstore: GPIOCsrStore::default(),
                    pins: Pins::default(),
                    memory: Memory::default(),
                }),
                ProbeLabel::rc_new("dummy_labeled_probe", (0.0, 0.0), dummy.clone()),
            ],
        };
        let mut component_vec = lib.store.clone();
        component_vec.append(&mut syncrim::gui_egui::editor::Library::default().0.clone());
        let _ =
            syncrim::gui_egui::gui(cs, &path, syncrim::gui_egui::editor::Library(component_vec));
    }

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
    #[cfg(feature = "headless")]
    {
        let mut simulator = syncrim::common::Simulator::new(cs).unwrap();
        simulator.set_running();
        simulator.run_until_halt();
    }
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
        .level_for("riscv::components::clic", log::LevelFilter::Trace)
        .level(log::LevelFilter::Error);

    // - and per-module overrides
    #[cfg(feature = "gui-vizia")]
    let f = f.level_for("riscv::gui_egui::components::instr_mem", LevelFilter::Trace);

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
            .args(["/C", "cargo clean"])
            .status()
        {
            Ok(_) => {}
            Err(_) => {
                panic!("cargo clean unsuccessful")
            }
        }
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
