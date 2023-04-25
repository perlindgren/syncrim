use mips::*;
use syncrim::components::Component;
use syncrim::{components::*, *};

fn main() {
    let c = Constant {
        id: "c1".to_string(),
        value: 42,
    };
    let c = Box::new(c) as Box<dyn Component>;
    let r = Register {
        id: "r1".to_string(),
        r_in: Input {
            id: "c1".to_string(),
            index: 0,
        },
    };
    let r = Box::new(r) as Box<dyn Component>;
    let m = MipsCtrl {
        id: "mips_ctrl".to_string(),
    };
    let m = Box::new(m) as Box<dyn Component>;

    let cs = ComponentStore {
        store: vec![c, r, m],
    };

    cs.to_();

    let json = serde_json::to_string(&cs).unwrap();
    println!("json: {}", json);

    let cs = ComponentStore::load(&json);
    cs.to_();

    let hm = cs.to_id_ports();

    println!("hm {:?}", hm);
}
