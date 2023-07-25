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
                Input::new("c", "out"),
                Input::new("r1", "out"),
            )),
            Constant::rc_new("c", (100.0, 100.0), 1),
            Rc::new(Register::new(
                "r1",
                (100.0, 140.0),
                Input::new("add", "out"),
            )),
            Rc::new(Wire::new(
                "w1",
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c", "out"),
            )),
            Rc::new(Wire::new(
                "w2",
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("r1", "out"),
            )),
            Rc::new(Wire::new(
                "w3",
                vec![(220.0, 120.0), (260.0, 120.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Wire::new(
                "w4",
                vec![(260.0, 120.0), (260.0, 180.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Wire::new(
                "w5",
                vec![(60.0, 180.0), (260.0, 180.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Wire::new(
                "w6",
                vec![(60.0, 140.0), (60.0, 180.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Wire::new(
                "w7",
                vec![(60.0, 140.0), (90.0, 140.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Probe::new(
                "p_add",
                (280.0, 120.0),
                Input::new("add", "out"),
            )),
            Rc::new(Probe::new("p_reg", (130.0, 120.0), Input::new("r1", "out"))),
            Rc::new(Mux::new(
                "mux",
                (270.0, 300.0),
                Input::new("add", "out"),
                vec![
                    Input::new("mc1", "out"),
                    Input::new("mc2", "out"),
                    Input::new("mc3", "out"),
                    Input::new("mc4", "out"),
                ],
            )),
            Constant::rc_new("mc1", (150.0, 270.0), 0),
            Constant::rc_new("mc2", (150.0, 290.0), 1),
            Constant::rc_new("mc3", (150.0, 310.0), 2),
            Constant::rc_new("mc4", (150.0, 330.0), 3),
            Rc::new(Wire::new(
                "wm_sel",
                vec![(260.0, 180.0), (260.0, 250.0)],
                Input::new("add", "out"),
            )),
            Rc::new(Wire::new(
                "wm1",
                vec![(200.0, 270.0), (250.0, 270.0)],
                Input::new("mc1", "out"),
            )),
            Rc::new(Wire::new(
                "wm2",
                vec![(200.0, 290.0), (250.0, 290.0)],
                Input::new("mc2", "out"),
            )),
            Rc::new(Wire::new(
                "wm3",
                vec![(200.0, 310.0), (250.0, 310.0)],
                Input::new("mc3", "out"),
            )),
            Rc::new(Wire::new(
                "wm4",
                vec![(200.0, 330.0), (250.0, 330.0)],
                Input::new("mc4", "out"),
            )),
            Rc::new(Wire::new(
                "wm_o0",
                vec![(290.0, 300.0), (340.0, 300.0)],
                Input::new("mux", "out"),
            )),
            Rc::new(Probe::new(
                "p_mux",
                (350.0, 300.0),
                Input::new("mux", "out"),
            )),
        ],
    };

    let path = PathBuf::from("add_mux.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
