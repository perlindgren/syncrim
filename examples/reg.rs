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
            Rc::new(Register {
                id: "reg".to_string(),
                pos: (200.0, 100.0),
                r_in: Input::new("c", "out"),
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: vec![(160.0, 100.0), (190.0, 100.0)],
                input: Input::new("c", "out"),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: vec![(210.0, 100.0), (240.0, 100.0)],
                input: Input::new("reg", "out"),
            }),
            Rc::new(Probe {
                id: "p_reg".to_string(),
                pos: (250.0, 100.0),
                input: Input::new("reg", "out"),
            }),
        ],
    };

    let path = PathBuf::from("reg.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
