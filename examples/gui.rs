use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let r1 = Register {
        id: "r1".to_string(),
        pos: (100.0, 20.0),
        r_in: Input {
            id: "r2".to_string(),
            index: 0,
        },
    };
    let r1 = Rc::new(r1);

    let r2 = Register {
        id: "r2".to_string(),
        pos: (100.0, 40.0),
        r_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };
    let r2 = Rc::new(r2);

    let cs = ComponentStore {
        store: vec![r1, r2],
    };

    println!("--- store id:s");
    cs.to_();

    syncrim::gui::gui(&cs);
}
