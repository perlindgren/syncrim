use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    gui_vizia::gui,
};

fn main() {
    let cs = ComponentStore {
        path: "mux.json".to_string(),
        store: vec![
            Rc::new(Mux {
                id: "mux".to_string(),
                pos: (200.0, 200.0),
                select: Input {
                    id: "c0".to_string(),
                    index: 0,
                },
                m_in: vec![
                    Input {
                        id: "c1".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "c2".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "c3".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "c4".to_string(),
                        index: 0,
                    },
                ],
            }),
            Rc::new(Constant {
                id: "c0".to_string(),
                pos: (190.0, 100.0),
                value: 3,
            }),
            Rc::new(Wire {
                id: "w0".to_string(),
                pos: (190.0, 110.0),
                size: (0.0, 40.0),
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
                pos: (150.0, 170.0),
                size: (30.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (150.0, 190.0),
                size: (30.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: (150.0, 210.0),
                size: (30.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: (150.0, 230.0),
                size: (30.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: (220.0, 200.0),
                size: (30.0, 0.0),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (260.0, 200.0),
                input: Input {
                    id: "mux".to_string(),
                    index: 0,
                },
            }),
        ],
    };

    cs.save_file();

    gui(&cs);
}
