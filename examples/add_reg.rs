use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Add {
                id: "add".to_string(),
                pos: (200.0, 120.0),
                a_in: Input::new("c", 0),
                b_in: Input::new("reg", 0),
            }),
            Rc::new(Constant {
                id: "c".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Register {
                id: "reg".to_string(),
                pos: (100.0, 140.0),
                r_in: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (110.0, 100.0),
                delta: (70.0, 0.0),
                input: Input::new("c", 0),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (110.0, 140.0),
                delta: (70.0, 0.0),
                input: Input::new("reg", 0),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: (220.0, 120.0),
                delta: (40.0, 0.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: (260.0, 120.0),
                delta: (0.0, 60.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: (60.0, 180.0),
                delta: (200.0, 0.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "w6".to_string(),
                pos: (60.0, 140.0),
                delta: (0.0, 40.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "w7".to_string(),
                pos: (60.0, 140.0),
                delta: (30.0, 0.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Probe {
                id: "p_add".to_string(),
                pos: (280.0, 120.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Probe {
                id: "p_reg".to_string(),
                pos: (130.0, 120.0),
                input: Input::new("reg", 0),
            }),
        ],
    };

    let path = PathBuf::from("add_reg.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
