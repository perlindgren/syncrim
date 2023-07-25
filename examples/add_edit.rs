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
                a_in: Input::new("c1", "out"),

                b_in: Input::new("c2", "out"),
            }),
            Rc::new(ProbeEdit::new("c1", (60.0, 100.0))),
            Rc::new(ProbeEdit::new("c2", (60.0, 140.0))),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: vec![(110.0, 100.0), (180.0, 100.0)],
                input: Input::new("c1", "out"),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: vec![(110.0, 140.0), (180.0, 140.0)],
                input: Input::new("c2", "out"),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: vec![(220.0, 120.0), (260.0, 120.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (270.0, 120.0),
                input: Input::new("add", "out"),
            }),
        ],
    };

    let path = PathBuf::from("add_edit.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
