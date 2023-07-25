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
            Constant::rc_new("c", (150.0, 100.0), 3),
            Rc::new(Register::new("reg", (200.0, 100.0), Input::new("c", "out"))),
            Rc::new(Wire::new(
                "w1",
                vec![(160.0, 100.0), (190.0, 100.0)],
                Input::new("c", "out"),
            )),
            Rc::new(Wire::new(
                "w2",
                vec![(210.0, 100.0), (240.0, 100.0)],
                Input::new("reg", "out"),
            )),
            Rc::new(Probe::new(
                "p_reg",
                (250.0, 100.0),
                Input::new("reg", "out"),
            )),
        ],
    };

    let path = PathBuf::from("reg.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
