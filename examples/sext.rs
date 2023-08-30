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
            Constant::rc_new("c0", (50.0, 110.0), 32768),
            Wire::rc_new(
                "w0",
                vec![(110.0, 110.0), (140.0, 110.0)],
                Input::new("c0", "out"),
            ),
            Sext::rc_new("sxt0", (180.0, 100.0), Input::new("c0", "out"), 16, 24),
            Wire::rc_new(
                "w1",
                vec![(220.0, 100.0), (250.0, 100.0)],
                Input::new("sxt0", "out"),
            ),
            Probe::rc_new("p1", (260.0, 100.0), Input::new("sxt0", "out")),
        ],
    };

    let path = PathBuf::from("sext.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
