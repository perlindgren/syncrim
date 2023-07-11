use mips::components::*;
use std::cell::Cell;
use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input, Signal},
    components::*,
};

// TODO: fix wires and layout
fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Constant {
                id: "c_reg_a".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "c_reg_b".to_string(),
                pos: (100.0, 200.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "c_ctrl".to_string(),
                pos: (100.0, 140.0),
                value: 42,
            }),
            // regfile
            Rc::new(BranchLogic {
                id: "branch".to_string(),
                pos: (200.0, 150.0),
                width: 100.0,
                height: 150.0,

                // ports
                reg_a: Input::new("c_reg_a", 0),
                reg_b: Input::new("c_reg_b", 0),
                ctrl: Input::new("c_ctrl", 0),
            }),
        ],
    };

    let path = PathBuf::from("branch_logic.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
