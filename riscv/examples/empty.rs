use clap::Parser;
use std::path::PathBuf;
use syncrim::common::ComponentStore;

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
    let path = PathBuf::from("autosave.json");
    let cs = ComponentStore::load_file(&path);
    #[cfg(feature = "gui-egui")]
    {
        use riscv::components::*;
        use std::{
            cell::RefCell,
            collections::{BTreeMap, HashMap, HashSet},
            ops::Range,
            rc::Rc,
        };
        use syncrim::common::Input;
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
                    breakpoints: Rc::new(RefCell::new(HashSet::new())),
                    le: true,
                    range: Range { start: 0, end: 0 },
                    symbols: HashMap::new(),
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
                    int: dummy.clone(),
                    mret: dummy.clone(),
                    enable: dummy.clone(),
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
        syncrim::gui_egui::gui(cs, &path, syncrim::gui_egui::editor::Library(component_vec)).ok();
    }

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
