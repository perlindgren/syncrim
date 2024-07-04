use std::path::PathBuf;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Sub::rc_new(
                "sub",
                (200.0, 120.0),
                Input::new("c1", "out"),
                Input::new("c2", "out"),
            ),
            Constant::rc_new("c1", (60.0, 100.0), 3),
            Constant::rc_new("c2", (60.0, 140.0), 4),
            Wire::rc_new(
                "w1",
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c1", "out"),
            ),
            Wire::rc_new(
                "w2",
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("c2", "out"),
            ),
            Wire::rc_new(
                "w3",
                vec![(220.0, 120.0), (260.0, 120.0)],
                Input::new("sub", SUB_OUT_ID),
            ),
            Probe::rc_new("p1", (270.0, 120.0), Input::new("sub", SUB_OUT_ID)),
        ],
    };

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
