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
            id: "c2".to_string(),
            index: 0,
        },
    };
    let add1 = Rc::new(add1);

    let c1 = Constant {
        id: "c1".to_string(),
        pos: (100.0, 100.0),
        value: 1,
    };
    let c1 = Rc::new(c1);

    let c2 = Constant {
        id: "c2".to_string(),
        pos: (100.0, 140.0),
        value: 2,
    };
    let c2 = Rc::new(c2);

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

        let cs = ComponentStore {
        store: vec![add1, c1, c2, w1, w2],
    };

    println!("--- store id:s");
    cs.to_();

    syncrim::gui::gui(&cs);
}
