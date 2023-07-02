use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    gui_vizia::gui,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Add {
                id: "add1".to_string(),
                pos: (200.0, 120.0),
                a_in: Input {
                    id: "c1".to_string(),
                    index: 0,
                },

                b_in: Input {
                    id: "r1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Constant {
                id: "c1".to_string(),
                pos: (100.0, 100.0),
                value: 1,
            }),
            Rc::new(Register {
                id: "r1".to_string(),
                pos: (100.0, 140.0),
                r_in: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (110.0, 100.0),
                delta: (70.0, 0.0),
                input: Input {
                    id: "c1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (110.0, 140.0),
                delta: (70.0, 0.0),
                input: Input {
                    id: "r1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: (220.0, 120.0),
                delta: (40.0, 0.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: (260.0, 120.0),
                delta: (0.0, 60.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: (60.0, 180.0),
                delta: (200.0, 0.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w6".to_string(),
                pos: (60.0, 140.0),
                delta: (0.0, 40.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "w7".to_string(),
                pos: (60.0, 140.0),
                delta: (30.0, 0.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Probe {
                id: "p_add".to_string(),
                pos: (280.0, 120.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Probe {
                id: "p_reg".to_string(),
                pos: (130.0, 120.0),
                input: Input {
                    id: "r1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Mux {
                id: "mux".to_string(),
                pos: (270.0, 300.0),
                select: Input {
                    id: "r1".to_string(),
                    index: 0,
                },
                m_in: vec![
                    Input {
                        id: "mc1".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "mc2".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "mc3".to_string(),
                        index: 0,
                    },
                    Input {
                        id: "mc4".to_string(),
                        index: 0,
                    },
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
                pos: (260.0, 180.0),
                delta: (0.0, 70.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "wm1".to_string(),
                pos: (200.0, 270.0),
                delta: (50.0, 0.0),
                input: Input {
                    id: "mc1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "wm2".to_string(),
                pos: (200.0, 290.0),
                delta: (50.0, 0.0),
                input: Input {
                    id: "mc2".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "wm3".to_string(),
                pos: (200.0, 310.0),
                delta: (50.0, 0.0),
                input: Input {
                    id: "mc3".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "wm4".to_string(),
                pos: (200.0, 330.0),
                delta: (50.0, 0.0),
                input: Input {
                    id: "mc4".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Wire {
                id: "wm_o0".to_string(),
                pos: (290.0, 300.0),
                delta: (50.0, 0.0),
                input: Input {
                    id: "mux".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Probe {
                id: "p_mux".to_string(),
                pos: (350.0, 300.0),
                input: Input {
                    id: "mux".to_string(),
                    index: 0,
                },
            }),
        ],
    };

    let path = PathBuf::from("add_mux.json");
    cs.save_file(&path);
    gui(&cs, &path);
}
