use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    gui_vizia::gui,
};

fn main() {
    let cs = ComponentStore {
        path: "add.json".to_string(),
        store: vec![
            Rc::new(Add {
                id: "add1".to_string(),
                pos: (200.0, 120.0),
                a_in: Input {
                    id: "c1".to_string(),
                    index: 0,
                },

                b_in: Input {
                    id: "c2".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Constant {
                id: "c1".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "c2".to_string(),
                pos: (100.0, 140.0),
                value: 4,
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (110.0, 100.0),
                size: (70.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (110.0, 140.0),
                size: (70.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: (220.0, 120.0),
                size: (40.0, 0.0),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (270.0, 120.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
        ],
    };

    cs.save_file();

    gui(&cs);
}
