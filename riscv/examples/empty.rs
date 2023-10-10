use clap::Parser;
use riscv::components::*;
use std::{collections::BTreeMap, path::PathBuf, rc::Rc};
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
}

fn main() {
    let cs = ComponentStore { store: vec![] };
    let dummy = Input::new("id", "field");
    let lib = ComponentStore {
        store: vec![Rc::new(InstrMem {
            id: "dummy_instr_mem".to_string(),
            pos: (0.0, 0.0),
            pc: dummy.clone(),
            bytes: BTreeMap::new(),
        })],
    };
    #[cfg(feature = "gui-egui")]
    let _library = syncrim::gui_egui::editor::Library(lib.store);
    let path = PathBuf::from("riscv.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, syncrim::gui_egui::editor::Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
