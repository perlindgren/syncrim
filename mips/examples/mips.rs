// An example MIPS model

use mips::components::*;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    gui::gui,
};

use std::rc::Rc;

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
                value: 4,
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
                id: "mem".to_string(),
                pos: (110.0, 100.0),
                size: (70.0, 0.0),
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
                size: (130.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: (260.0, 120.0),
                size: (0.0, 60.0),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: (260.0, 180.0),
                size: (-200.0, 0.0),
            }),
            Rc::new(Wire {
                id: "w6".to_string(),
                pos: (60.0, 180.0),
                size: (0.0, -40.0),
            }),
            Rc::new(Wire {
                id: "w7".to_string(),
                pos: (60.0, 140.0),
                size: (30.0, 0.0),
            }),
            Rc::new(InstrMem {
                id: "instr_mem".to_string(),
                pos: (400.0, 150.0),
                pc: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
                instr: vec![0, 1, 2, 3, 5, 6, 7, 8, 9],
            }),
            Rc::new(Wire {
                id: "w8".to_string(),
                pos: (450.0, 120.0),
                size: (70.0, 0.0),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (280.0, 100.0),
                input: Input {
                    id: "add1".to_string(),
                    index: 0,
                },
            }),
            Rc::new(Probe {
                id: "p2".to_string(),
                pos: (500.0, 100.0),
                input: Input {
                    id: "instr_mem".to_string(),
                    index: 0,
                },
            }),
        ],
    };

    cs.save_file("mips.json");

    gui(&cs);
}
