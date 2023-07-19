use std::{cell::RefCell, path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(RefCell::new(Add::new(
                "add".to_string(),
                (200.0, 120.0),
                Input::new("c1", 0),
                Input::new("c2", 0),
            ))),
            Rc::new(RefCell::new(Constant {
                id: "c1".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            })),
            Rc::new(RefCell::new(Constant {
                id: "c2".to_string(),
                pos: (100.0, 140.0),
                value: 4,
            })),
            Rc::new(RefCell::new(Wire {
                id: "w1".to_string(),
                pos: (110.0, 100.0),
                delta: (70.0, 0.0),
                input: Input::new("c1", 0),
            })),
            Rc::new(RefCell::new(Wire {
                id: "w2".to_string(),
                pos: (110.0, 140.0),
                delta: (70.0, 0.0),
                input: Input::new("c2", 0),
            })),
            Rc::new(RefCell::new(Wire {
                id: "w3".to_string(),
                pos: (220.0, 120.0),
                delta: (40.0, 0.0),
                input: Input::new("add", 0),
            })),
            Rc::new(RefCell::new(Probe {
                id: "p1".to_string(),
                pos: (270.0, 120.0),
                input: Input::new("add", 0),
            })),
        ],
    };

    let path = PathBuf::from("add.json");
    //cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
