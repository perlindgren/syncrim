#[allow(unused_imports)]
use mips::*;

use syncrim::common::{ComponentStore, Simulator};

fn main() {
    let cs = ComponentStore::load_file("mips.json");

    let (_simulator, sim_state) = Simulator::new(&cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
}
