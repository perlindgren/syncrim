use mips::*;
use std::fs::File;
use std::io::prelude::*;
use syncrim::components::Component;
use syncrim::{components::*, *};

fn main() {
    let c1 = Constant {
        id: "c1".to_string(),
        value: 0,
    };

    let c1 = Box::new(c1) as Box<dyn Component>;

    let c2 = Constant {
        id: "c2".to_string(),
        value: 4,
    };

    let c2 = Box::new(c2) as Box<dyn Component>;

    let a = Add {
        id: "add1".to_string(),
        a_in: Input {
            id: "c2".to_string(),
            index: 0,
        },

        b_in: Input {
            id: "r1".to_string(),
            index: 0,
        },
    };

    let a = Box::new(a) as Box<dyn Component>;

    let m = Mux {
        id: "m1".to_string(),
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

    let m = Box::new(m) as Box<dyn Component>;

    let r = Register {
        id: "r1".to_string(),
        r_in: Input {
            id: "m1".to_string(),
            index: 0,
        },
    };
    let r = Box::new(r) as Box<dyn Component>;
    let mips = MipsCtrl {
        id: "mips_ctrl".to_string(),
    };

    let mips = Box::new(mips) as Box<dyn Component>;

    let cs = ComponentStore {
        store: vec![c1, c2, a, r, m, mips],
    };

    cs.to_();

    let json = serde_json::to_string(&cs).unwrap();
    println!("json: {}", json);

    let mut file = File::create("mips.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
