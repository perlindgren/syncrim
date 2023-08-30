use std::path::PathBuf;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Add::rc_new(
                "add",
                (200.0, 120.0),
                Input::new("c", "out"),
                Input::new("reg", "out"),
            ),
            Constant::rc_new("c", (100.0, 100.0), 3),
            Register::rc_new("reg", (100.0, 140.0), Input::new("add", "out")),
            Wire::rc_new(
                "w1",
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c", "out"),
            ),
            Wire::rc_new(
                "w2",
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("reg", "out"),
            ),
            Wire::rc_new(
                "w3",
                vec![
                    (220.0, 120.0),
                    (260.0, 120.0),
                    (260.0, 180.0),
                    (60.0, 180.0),
                    (60.0, 140.0),
                    (90.0, 140.0),
                ],
                Input::new("add", "out"),
            ),
            Probe::rc_new("p_add", (280.0, 120.0), Input::new("add", "out")),
            Probe::rc_new("p_reg", (130.0, 120.0), Input::new("reg", "out")),
        ],
    };

    let path = PathBuf::from("add_reg_compound.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
