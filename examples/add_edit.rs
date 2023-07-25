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
            Rc::new(Add::new(
                "add",
                (200.0, 120.0),
                Input::new("c1", "out"),
                Input::new("c2", "out"),
            )),
            Rc::new(ProbeEdit::new("c1", (60.0, 100.0))),
            Rc::new(ProbeEdit::new("c2", (60.0, 140.0))),
            Rc::new(Wire::new(
                "w1",
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c1", "out"),
            )),
            Rc::new(Wire::new(
                "w2",
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("c2", "out"),
            )),
            Rc::new(Wire::new(
                "w3",
                vec![(220.0, 120.0), (260.0, 120.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Probe::new("p1", (270.0, 120.0), Input::new("add", "out"))),
        ],
    };

    let path = PathBuf::from("add_edit.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
