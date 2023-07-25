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
                Input::new("reg", "out"),
            )),
            Constant::rc_new("c", (100.0, 100.0), 3),
            Rc::new(Register::new(
                "reg",
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
                Input::new("reg", "out"),
            )),
            Rc::new(Wire::new(
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
            )),
            Rc::new(Probe::new(
                "p_add",
                (280.0, 120.0),
                Input::new("add", "out"),
            )),
            Rc::new(Probe::new(
                "p_reg",
                (130.0, 120.0),
                Input::new("reg", "out"),
            )),
        ],
    };

    let path = PathBuf::from("add_reg_compound.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
