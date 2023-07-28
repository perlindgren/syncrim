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
                Input::new("r1", "out"),
            ),
            Constant::rc_new("c", (100.0, 100.0), 1),
            Register::rc_new("r1", (100.0, 140.0), Input::new("add", "out")),
            Wire::rc_new(
                "w1",
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c", "out"),
            ),
            Wire::rc_new(
                "w2",
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("r1", "out"),
            ),
            Wire::rc_new(
                "w3",
                vec![(220.0, 120.0), (260.0, 120.0)],
                Input::new("add", "out"),
            ),
            Wire::rc_new(
                "w4",
                vec![(260.0, 120.0), (260.0, 180.0)],
                Input::new("add", "out"),
            ),
            Wire::rc_new(
                "w5",
                vec![(60.0, 180.0), (260.0, 180.0)],
                Input::new("add", "out"),
            ),
            Wire::rc_new(
                "w6",
                vec![(60.0, 140.0), (60.0, 180.0)],
                Input::new("add", "out"),
            ),
            Wire::rc_new(
                "w7",
                vec![(60.0, 140.0), (90.0, 140.0)],
                Input::new("add", "out"),
            ),
            Probe::rc_new("p_add", (280.0, 120.0), Input::new("add", "out")),
            Probe::rc_new("p_reg", (130.0, 120.0), Input::new("r1", "out")),
            Mux::rc_new(
                "mux",
                (270.0, 300.0),
                Input::new("add", "out"),
                vec![
                    Input::new("mc1", "out"),
                    Input::new("mc2", "out"),
                    Input::new("mc3", "out"),
                    Input::new("mc4", "out"),
                ],
            ),
            Constant::rc_new("mc1", (150.0, 270.0), 0),
            Constant::rc_new("mc2", (150.0, 290.0), 1),
            Constant::rc_new("mc3", (150.0, 310.0), 2),
            Constant::rc_new("mc4", (150.0, 330.0), 3),
            Wire::rc_new(
                "wm_sel",
                vec![(260.0, 180.0), (260.0, 250.0)],
                Input::new("add", "out"),
            ),
            Wire::rc_new(
                "wm1",
                vec![(200.0, 270.0), (250.0, 270.0)],
                Input::new("mc1", "out"),
            ),
            Wire::rc_new(
                "wm2",
                vec![(200.0, 290.0), (250.0, 290.0)],
                Input::new("mc2", "out"),
            ),
            Wire::rc_new(
                "wm3",
                vec![(200.0, 310.0), (250.0, 310.0)],
                Input::new("mc3", "out"),
            ),
            Wire::rc_new(
                "wm4",
                vec![(200.0, 330.0), (250.0, 330.0)],
                Input::new("mc4", "out"),
            ),
            Wire::rc_new(
                "wm_o0",
                vec![(290.0, 300.0), (340.0, 300.0)],
                Input::new("mux", "out"),
            ),
            Probe::rc_new("p_mux", (350.0, 300.0), Input::new("mux", "out")),
        ],
    };

    let path = PathBuf::from("add_mux.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
