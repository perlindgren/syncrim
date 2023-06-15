use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let add1 = Add {
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
    };
    let add1 = Rc::new(add1);

    let c1 = Constant {
        id: "c1".to_string(),
        pos: (100.0, 100.0),
        value: 3,
    };
    let c1 = Rc::new(c1);

    let r1 = Register {
        id: "r1".to_string(),
        pos: (100.0, 140.0),
        r_in: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let r1 = Rc::new(r1);

    let w1 = Wire {
        id: "w1".to_string(),
        pos: (110.0, 100.0),
        size: (70.0, 0.0),
    };
    let w1 = Rc::new(w1);

    let w2 = Wire {
        id: "w2".to_string(),
        pos: (110.0, 140.0),
        size: (70.0, 0.0),
    };
    let w2 = Rc::new(w2);

    let w3 = Wire {
        id: "w3".to_string(),
        pos: (220.0, 120.0),
        size: (40.0, 0.0),
    };
    let w3 = Rc::new(w3);

    let w4 = Wire {
        id: "w4".to_string(),
        pos: (260.0, 120.0),
        size: (0.0, 60.0),
    };
    let w4 = Rc::new(w4);

    let w5 = Wire {
        id: "w5".to_string(),
        pos: (260.0, 180.0),
        size: (-200.0, 0.0),
    };
    let w5 = Rc::new(w5);

    let w6 = Wire {
        id: "w6".to_string(),
        pos: (60.0, 180.0),
        size: (0.0, -40.0),
    };
    let w6 = Rc::new(w6);

    let w7 = Wire {
        id: "w7".to_string(),
        pos: (60.0, 140.0),
        size: (30.0, 0.0),
    };
    let w7 = Rc::new(w7);

    let p1 = Probe {
        id: "p1".to_string(),
        pos: (280.0, 120.0),
        input: Input {
            id: "add1".to_string(),
            index: 0,
        },
    };
    let p1 = Rc::new(p1);

    let cs = ComponentStore {
        store: vec![add1, c1, r1, w1, w2, w3, w4, w5, w6, w7, p1],
    };

    println!("--- store id:s");
    cs.to_();

    syncrim::gui::gui(&cs);
}
