use std::{cell::RefCell, path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Rc::new(RefCell::new(Add::new(
                "add".to_string(),
                (200.0, 120.0),
                Input::new("c1", "out"),
                Input::new("c2", "out"),
            ))),
            Rc::new(RefCell::new(Constant::new(
                "c1".to_string(),
                (100.0, 100.0),
                3,
            ))),
            Rc::new(RefCell::new(Constant::new(
                "c2".to_string(),
                (100.0, 140.0),
                4,
            ))),
            Rc::new(RefCell::new(Wire::new(
                "w1".to_string(),
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c1", "out"),
            ))),
            Rc::new(RefCell::new(Wire::new(
                "w2".to_string(),
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("c2", "out"),
            ))),
            Rc::new(RefCell::new(Wire::new(
                "w3".to_string(),
                vec![(220.0, 120.0), (260.0, 120.0)],
                Input::new("add", "out"),
            ))),
            Rc::new(RefCell::new(Probe::new(
                "p1".to_string(),
                (270.0, 120.0),
                Input::new("add", "out"),
            ))),
        ],
    };

    let path = PathBuf::from("add.json");
    ////cs.save_file(&path);
    //let cs2 = ComponentStore::load_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
