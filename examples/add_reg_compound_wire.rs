use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Rc::new(Add {
                id: "add".to_string(),
                pos: (200.0, 120.0),
                a_in: Input::new("c", "out"),
                b_in: Input::new("reg", "out"),
            }),
            Rc::new(Constant {
                id: "c".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Register {
                id: "reg".to_string(),
                pos: (100.0, 140.0),
                r_in: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: vec![(110.0, 100.0), (180.0, 100.0)],
                input: Input::new("c", "out"),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: vec![(110.0, 140.0), (180.0, 140.0)],
                input: Input::new("reg", "out"),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: vec![(220.0, 120.0), (260.0, 120.0) , (260.0, 180.0), (60.0, 180.0), (60.0, 140.0), (90.0, 140.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Probe {
                id: "p_add".to_string(),
                pos: (280.0, 120.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Probe {
                id: "p_reg".to_string(),
                pos: (130.0, 120.0),
                input: Input::new("reg", "out"),
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
