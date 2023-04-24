use syncrim::*;

fn main() {
    let c = Constant { v: 0 };
    let c = Box::new(c) as Box<dyn NewTrait>;
    let r = Register { r_in: 0, r_out: 1 };
    let r = Box::new(r) as Box<dyn NewTrait>;

    // let cs = Components {
    //     components: vec![c, r],
    // };

    let v = vec![c, r];
}
