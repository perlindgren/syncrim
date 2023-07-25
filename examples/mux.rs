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
            Rc::new(Mux::new(
                "mux",
                (200.0, 200.0),
                Input::new("c0", "out"),
                vec![
                    Input::new("c1", "out"),
                    Input::new("c2", "out"),
                    Input::new("c3", "out"),
                    Input::new("c4", "out"),
                ],
            )),
            Rc::new(Constant::new("c0", (190.0, 100.0), 3)),
            Rc::new(Wire::new(
                "w0",
                vec![(190.0, 110.0), (190.0, 150.0)],
                Input::new("c0", "out"),
            )),
            Rc::new(Constant::new("c1", (140.0, 170.0), 0)),
            Rc::new(Constant::new("c2", (140.0, 190.0), 1)),
            Rc::new(Constant::new("c3", (140.0, 210.0), 2)),
            Rc::new(Constant::new("c4", (140.0, 230.0), 3)),
            Rc::new(Wire::new(
                "w1",
                vec![(150.0, 170.0), (180.0, 170.0)],
                Input::new("c1", "out"),
            )),
            Rc::new(Wire::new(
                "w2",
                vec![(150.0, 190.0), (180.0, 190.0)],
                Input::new("c2", "out"),
            )),
            Rc::new(Wire::new(
                "w3",
                vec![(150.0, 210.0), (180.0, 210.0)],
                Input::new("c3", "out"),
            )),
            Rc::new(Wire::new(
                "w4",
                vec![(150.0, 230.0), (180.0, 230.0)],
                Input::new("c4", "out"),
            )),
            Rc::new(Wire::new(
                "w5",
                vec![(220.0, 200.0), (250.0, 200.0)],
                Input::new("mux", "out"),
            )),
            Rc::new(Probe::new(
                "p_mux",
                (260.0, 200.0),
                Input::new("mux", "out"),
            )),
        ],
    };

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
