use mips::*;

use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

use std::rc::Rc;

fn main() {
    let c1 = Constant {
        id: "c1".to_string(),
        pos: (10.0, 10.0),
        value: 0,
    };

    let c1 = Rc::new(c1);

    let c2 = Constant {
        id: "c2".to_string(),
        pos: (10.0, 30.0),
        value: 4,
    };

    let c2 = Rc::new(c2);

    let a = Add {
        id: "add1".to_string(),
        pos: (10.0, 50.0),
        a_in: Input {
            id: "c2".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };

    let a = Rc::new(a);

    let m = Mux {
        id: "m1".to_string(),
        pos: (10.0, 70.0),
        select: Input {
            id: "c1".to_string(),
            index: 0,
        },
        m_in: vec![
            Input {
                id: "add1".to_string(),
                index: 0,
            },
            Input {
                id: "r1".to_string(),
                index: 0,
            },
        ],
    };

    let m = Rc::new(m);

    let r = Register {
        id: "r1".to_string(),
        pos: (10.0, 90.0),
        r_in: Input {
            id: "m1".to_string(),
            index: 0,
        },
    };

    let r = Rc::new(r);

    let mips = MipsCtrl {
        id: "mips_ctrl".to_string(),
    };

    let mips = Rc::new(mips);

    let _cs = ComponentStore {
        store: vec![c1, c2, a, r, m, mips],
    };
}
