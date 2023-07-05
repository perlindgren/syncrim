use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Constant {
                id: "c0".to_string(),
                pos: (100.0, 110.0),
                value: 32768,
            }),
            Rc::new(Wire {
                id: "w0".to_string(),
                pos: (110.0, 110.0),
                delta: (30.0, 0.0),
                input: Input::new("c0", 0),
            }),
            Rc::new(Sext {
                id: "sxt0".to_string(),
                pos: (180.0, 100.0),
                sext_in: Input::new("c0", 0),
                in_size: 16,
                out_size: 24,
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (220.0, 100.0),
                delta: (30.0, 0.0),
                input: Input::new("sxt0", 0),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (260.0, 100.0),
                input: Input::new("sxt0", 0),
            }),
        ],
    };

    let path = PathBuf::from("sext.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
