use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let r = Register {
        id: "r1".to_string(),
        pos: (100.0, 20.0),
        r_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };
    let r = Rc::new(r);

    let cs = ComponentStore { store: vec![r] };

    println!("--- store id:s");
    cs.to_();

    syncrim::gui::gui(&cs);
}
