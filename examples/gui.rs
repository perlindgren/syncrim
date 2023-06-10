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
            id: "c1".to_string(),
            index: 0,
        },
    };
    let r1 = Rc::new(r1);

    let c1 = Constant {
        id: "c1".to_string(),
        pos: (100.0, 40.0),
        value: 2,
    };
    let c1 = Rc::new(c1);

    let cs = ComponentStore {
        store: vec![r1, c1],
    };

    println!("--- store id:s");
    cs.to_();

    syncrim::gui::gui(&cs);
}
