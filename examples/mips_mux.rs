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
            Mux::rc_new(
                "mux",
                (200.0, 200.0),
                Input::new("ctrl", "out"),
                vec![Input::new("c0", "out"), Input::new("c1", "out")],
            ),
            ProbeEdit::rc_new("ctrl", (190.0, 100.0)),
            Wire::rc_new(
                "w0",
                vec![(190.0, 110.0), (190.0, 150.0)],
                Input::new("ctrl", "out"),
            ),
            Constant::rc_new("c0", (140.0, 170.0), 0),
            Constant::rc_new("c1", (140.0, 190.0), 1),
            // Wire::rc_new(
            //     "w1",
            //     vec![(150.0, 170.0), (180.0, 170.0)],
            //     Input::new("c1", "out"),
            // ),
            // Wire::rc_new(
            //     "w2",
            //     vec![(150.0, 190.0), (180.0, 190.0)],
            //     Input::new("c2", "out"),
            // ),
            // Wire::rc_new(
            //     "w3",
            //     vec![(150.0, 210.0), (180.0, 210.0)],
            //     Input::new("c3", "out"),
            // ),
            // Wire::rc_new(
            //     "w4",
            //     vec![(150.0, 230.0), (180.0, 230.0)],
            //     Input::new("c4", "out"),
            // ),
            // Wire::rc_new(
            //     "w5",
            //     vec![(220.0, 200.0), (250.0, 200.0)],
            //     Input::new("mux", "out"),
            // ),
            Probe::rc_new("p_mux", (260.0, 200.0), Input::new("mux", "out")),
        ],
    };

    let path = PathBuf::from("mux_edit.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
