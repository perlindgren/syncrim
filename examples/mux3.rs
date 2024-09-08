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
                vec![
                    Input::new("c1", "out"),
                    Input::new("c2", "out"),
                    Input::new("c3", "out"),
                    Input::new("c4", "out"),
                ],
            ),
            Mux::rc_new(
                "mux2",
                (300.0, 300.0),
                Input::new("ctrl2", "out"),
                vec![
                    Input::new("mux", "out"),
                    Input::new("c2_2", "out"),
                    Input::new("c3_2", "out"),
                    Input::new("c4_2", "out"),
                ],
            ),
            ProbeEdit::rc_new("ctrl", (190.0, 100.0)),
            ProbeEdit::rc_new("ctrl2", (290.0, 200.0)),
            Wire::rc_new(
                "w0",
                vec![(190.0, 110.0), (190.0, 150.0)],
                Input::new("ctrl", "out"),
            ),
            Wire::rc_new(
                "w0_2",
                vec![(290.0, 210.0), (290.0, 250.0)],
                Input::new("ctrl2", "out"),
            ),
            Constant::rc_new("c1", (140.0, 170.0), 0),
            Constant::rc_new("c2", (140.0, 190.0), 1),
            Constant::rc_new("c3", (140.0, 210.0), 2),
            Constant::rc_new("c4", (140.0, 230.0), 3),
            Constant::rc_new("c2_2", (140.0, 290.0), 21),
            Constant::rc_new("c3_2", (140.0, 310.0), 22),
            Constant::rc_new("c4_2", (140.0, 330.0), 23),
            Wire::rc_new(
                "w1",
                vec![(150.0, 170.0), (180.0, 170.0)],
                Input::new("c1", "out"),
            ),
            Wire::rc_new(
                "w2",
                vec![(150.0, 190.0), (180.0, 190.0)],
                Input::new("c2", "out"),
            ),
            Wire::rc_new(
                "w3",
                vec![(150.0, 210.0), (180.0, 210.0)],
                Input::new("c3", "out"),
            ),
            Wire::rc_new(
                "w4",
                vec![(150.0, 230.0), (180.0, 230.0)],
                Input::new("c4", "out"),
            ),
            // Wire::rc_new(
            //     "w1_2",
            //     vec![(150.0, 270.0), (280.0, 270.0)],
            //     Input::new("c1_2", "out"),
            // ),
            Wire::rc_new(
                "w2_2",
                vec![(150.0, 290.0), (280.0, 290.0)],
                Input::new("c2_2", "out"),
            ),
            Wire::rc_new(
                "w3_2",
                vec![(150.0, 310.0), (280.0, 310.0)],
                Input::new("c3_2", "out"),
            ),
            Wire::rc_new(
                "w4_2",
                vec![(150.0, 330.0), (280.0, 330.0)],
                Input::new("c4_2", "out"),
            ),
            Wire::rc_new(
                "w5",
                vec![
                    (210.0, 200.0),
                    (230.0, 200.0),
                    (230.0, 270.0),
                    (280.0, 270.0),
                ],
                Input::new("mux", "out"),
            ),
            Wire::rc_new(
                "w6",
                vec![(310.0, 300.0), (350.0, 300.0)],
                Input::new("mux", "out"),
            ),
            Probe::rc_new("probe_mux", (360.0, 300.0), Input::new("mux2", "out")),
        ],
    };

    let path = PathBuf::from("mux2.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
