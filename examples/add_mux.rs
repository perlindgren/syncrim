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
            Rc::new(Add {
                id: "add".to_string(),
                pos: (200.0, 120.0),
                a_in: Input::new("c", "out"),
                b_in: Input::new("r1", "out"),
            }),
            Rc::new(Constant {
                id: "c".to_string(),
                pos: (100.0, 100.0),
                value: 1,
            }),
            Rc::new(Register {
                id: "r1".to_string(),
                pos: (100.0, 140.0),
                r_in: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: vec![(110.0, 100.0), (180.0, 100.0)],
                input: Input::new("c", "out"),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: vec![(110.0, 140.0), (180.0, 140.0)],
                input: Input::new("r1", "out"),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: vec![(220.0, 120.0), (260.0, 120.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: vec![(260.0, 120.0), (260.0, 180.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: vec![(60.0, 180.0), (260.0, 180.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "w6".to_string(),
                pos: vec![(60.0, 140.0), (60.0, 180.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "w7".to_string(),
                pos: vec![(60.0, 140.0), (90.0, 140.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Probe {
                id: "p_add".to_string(),
                pos: (280.0, 120.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Probe {
                id: "p_reg".to_string(),
                pos: (130.0, 120.0),
                input: Input::new("r1", "out"),
            }),
            Rc::new(Mux {
                id: "mux".to_string(),
                pos: (270.0, 300.0),
                select: Input::new("add", "out"),
                m_in: vec![
                    Input::new("mc1", "out"),
                    Input::new("mc2", "out"),
                    Input::new("mc3", "out"),
                    Input::new("mc4", "out"),
                ],
            }),
            Rc::new(Constant {
                id: "mc1".to_string(),
                pos: (190.0, 270.0),
                value: 0,
            }),
            Rc::new(Constant {
                id: "mc2".to_string(),
                pos: (190.0, 290.0),
                value: 1,
            }),
            Rc::new(Constant {
                id: "mc3".to_string(),
                pos: (190.0, 310.0),
                value: 2,
            }),
            Rc::new(Constant {
                id: "mc4".to_string(),
                pos: (190.0, 330.0),
                value: 3,
            }),
            Rc::new(Wire {
                id: "wm_sel".to_string(),
                pos: vec![(260.0, 180.0), (260.0, 250.0)],
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "wm1".to_string(),
                pos: vec![(200.0, 270.0), (250.0, 270.0)],
                input: Input::new("mc1", "out"),
            }),
            Rc::new(Wire {
                id: "wm2".to_string(),
                pos: vec![(200.0, 290.0), (250.0, 290.0)],
                input: Input::new("mc2", "out"),
            }),
            Rc::new(Wire {
                id: "wm3".to_string(),
                pos: vec![(200.0, 310.0), (250.0, 310.0)],
                input: Input::new("mc3", "out"),
            }),
            Rc::new(Wire {
                id: "wm4".to_string(),
                pos: vec![(200.0, 330.0), (250.0, 330.0)],
                input: Input::new("mc4", "out"),
            }),
            Rc::new(Wire {
                id: "wm_o0".to_string(),
                pos: vec![(290.0, 300.0), (340.0, 300.0)],
                input: Input::new("mux", "out"),
            }),
            Rc::new(Probe {
                id: "p_mux".to_string(),
                pos: (350.0, 300.0),
                input: Input::new("mux", "out"),
            }),
        ],
    };

    let path = PathBuf::from("add_mux.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
