// An example MIPS model

use mips::components::*;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    gui_vizia::gui,
};

use std::rc::Rc;

fn main() {
    let cs = ComponentStore {
        path: "mips.json".to_string(),
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
                id: "c1_to_add1_a".to_string(),
                pos: (110.0, 100.0),
                size: (70.0, 0.0),
            }),
            Rc::new(Wire {
                id: "reg_to_add1_b".to_string(),
                pos: (110.0, 140.0),
                size: (70.0, 0.0),
            }),
            Rc::new(Wire {
                id: "reg_to_right".to_string(),
                pos: (220.0, 120.0),
                size: (40.0, 0.0),
            }),
            Rc::new(Wire {
                id: "reg_to_up".to_string(),
                pos: (260.0, 120.0),
                size: (0.0, -60.0),
            }),
            Rc::new(Wire {
                id: "reg_to_left".to_string(),
                pos: (260.0, 60.0),
                size: (-200.0, 0.0),
            }),
            Rc::new(Wire {
                id: "reg_to_down".to_string(),
                pos: (60.0, 60.0),
                size: (0.0, 80.0),
            }),
            Rc::new(Wire {
                id: "reg_in".to_string(),
                pos: (60.0, 140.0),
                size: (30.0, 0.0),
            }),
            Rc::new(Wire {
                id: "pc_to_down".to_string(),
                pos: (140.0, 140.0),
                size: (0.0, 40.0),
            }),
            Rc::new(Wire {
                id: "pc_to_right".to_string(),
                pos: (140.0, 180.0),
                size: (210.0, 0.0),
            }),
            Rc::new(InstrMem {
                id: "instr_mem".to_string(),
                pos: (400.0, 150.0),
                pc: Input {
                    id: "r1".to_string(),
                    index: 0,
                },
                // fake instructions just to show the relation between input address and instruction
                instr: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            Rc::new(Wire {
                id: "w8".to_string(),
                pos: (450.0, 120.0),
                size: (70.0, 0.0),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (280.0, 160.0),
                input: Input {
                    id: "r1".to_string(),
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

    cs.save_file();

    gui(&cs);
}
