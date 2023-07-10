use mips::components::*;
use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

// TODO: fix wires and layout
fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Constant {
                id: "instr".to_string(),
                pos: (100.0, 100.0),
                value: 0b000000_00001_00010_00011_00000000100,
            }),
            // instr_split
            Rc::new(InstrSplit {
                id: "instr_split".to_string(),
                pos: (150.0, 100.0),
                width: 10.0,
                height: 200.0,

                // ports
                instr: Input::new("instr", 0),
            }),
            Rc::new(Probe {
                id: "op".to_string(),
                pos: (200.0, 100.0),
                input: Input::new("instr_split", 0),
            }),
            Rc::new(Probe {
                id: "rs".to_string(),
                pos: (200.0, 120.0),
                input: Input::new("instr_split", 1),
            }),
            Rc::new(Probe {
                id: "rt".to_string(),
                pos: (200.0, 140.0),
                input: Input::new("instr_split", 2),
            }),
            Rc::new(Probe {
                id: "rd".to_string(),
                pos: (200.0, 160.0),
                input: Input::new("instr_split", 3),
            }),
            Rc::new(Probe {
                id: "imm16".to_string(),
                pos: (200.0, 180.0),
                input: Input::new("instr_split", 4),
            }),
            Rc::new(Probe {
                id: "imm26".to_string(),
                pos: (200.0, 200.0),
                input: Input::new("instr_split", 5),
            }),
        ],
    };

    let path = PathBuf::from("instr_split.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
