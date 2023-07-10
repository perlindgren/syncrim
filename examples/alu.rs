use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Alu {
                id: "alu".to_string(),
                pos: (200.0, 200.0),
                a_in: Input::new("alu_in_a", 0),
                b_in: Input::new("alu_in_b", 0),
                op: Input::new("alu_op", 0),
            }),
            Rc::new(Constant {
                id: "alu_in_a".to_string(),
                pos: (100.0, 140.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "alu_in_b".to_string(),
                pos: (100.0, 260.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "alu_op".to_string(),
                pos: (170.0, 80.0),
                value: 4,
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (110.0, 140.0),
                delta: (50.0, 0.0),
                input: Input::new("alu_in_a", 0),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (110.0, 260.0),
                delta: (50.0, 0.0),
                input: Input::new("alu_in_b", 0),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: (240.0, 200.0),
                delta: (50.0, 0.0),
                input: Input::new("alu_op", 0),
            }),
            Rc::new(Probe {
                id: "alu_out".to_string(),
                pos: (300.0, 200.0),
                input: Input::new("alu", 0),
            }),
            Rc::new(Probe {
                id: "z".to_string(),
                pos: (190.0, 80.0),
                input: Input::new("alu", 1),
            }),
            Rc::new(Probe {
                id: "v".to_string(),
                pos: (210.0, 80.0),
                input: Input::new("alu", 1),
            }),
        ],
    };

    let path = PathBuf::from("alu.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
