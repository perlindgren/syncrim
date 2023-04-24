use mips::*;
use syncrim::*;

fn main() {
    let c = Constant { v: 42 };
    let c = Box::new(c) as Box<dyn Component>;
    let r = Register { r_in: 0, r_out: 1 };
    let r = Box::new(r) as Box<dyn Component>;
    let m = MipsCtrl {};
    let m = Box::new(m) as Box<dyn Component>;

    let cs = ComponentStore {
        store: vec![c, r, m],
    };

    cs.to_();

    let json = serde_json::to_string(&cs).unwrap();
    println!("json: {}", json);

    let cs: ComponentStore = serde_json::from_str(&json).unwrap();
    cs.to_();
}
