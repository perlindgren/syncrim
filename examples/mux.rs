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
            Rc::new(Mux {
                id: "mux".to_string(),
                pos: (200.0, 200.0),
                select: Input::new("c0", "out"),
                m_in: vec![
                    Input::new("c1", "out"),
                    Input::new("c2", "out"),
                    Input::new("c3", "out"),
                    Input::new("c4", "out"),
                ],
            }),
            Rc::new(Constant {
                id: "c0".to_string(),
                pos: (190.0, 100.0),
                value: 3,
            }),
            Rc::new(Wire {
                id: "w0".to_string(),
                pos: vec![(190.0, 110.0), (190.0, 150.0)],
                input: Input::new("c0", "out"),
            }),
            Rc::new(Constant {
                id: "c1".to_string(),
                pos: (140.0, 170.0),
                value: 0,
            }),
            Rc::new(Constant {
                id: "c2".to_string(),
                pos: (140.0, 190.0),
                value: 1,
            }),
            Rc::new(Constant {
                id: "c3".to_string(),
                pos: (140.0, 210.0),
                value: 2,
            }),
            Rc::new(Constant {
                id: "c4".to_string(),
                pos: (140.0, 230.0),
                value: 3,
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: vec![(150.0, 170.0), (180.0, 170.0)],
                input: Input::new("c1", "out"),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: vec![(150.0, 190.0), (180.0, 190.0)],
                input: Input::new("c2", "out"),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: vec![(150.0, 210.0), (180.0, 210.0)],
                input: Input::new("c3", "out"),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: vec![(150.0, 230.0), (180.0, 230.0)],
                input: Input::new("c4", "out"),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: vec![(220.0, 200.0), (250.0, 200.0)],
                input: Input::new("mux", "out"),
            }),
            Rc::new(Probe {
                id: "p_mux".to_string(),
                pos: (260.0, 200.0),
                input: Input::new("mux", "out"),
            }),
        ],
    };

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
