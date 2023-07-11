use mips::components::CtrlLogic;
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
                id: "c_op".to_string(),
                pos: (100.0, 80.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "c_shamt".to_string(),
                pos: (100.0, 100.0),
                value: 42,
            }),
            Rc::new(Constant {
                id: "c_funct".to_string(),
                pos: (100.0, 120.0),
                value: 42,
            }),
            // regfile
            Rc::new(CtrlLogic {
                id: "ctrl_logic".to_string(),
                pos: (660.0, 100.0),
                width: 800.0,
                height: 70.0,

                // ports
                op: Input::new("c_op", 0),
                shamt: Input::new("c_shamt", 0),
                funct: Input::new("c_funct", 0),
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
