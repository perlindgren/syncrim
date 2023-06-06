#[allow(unused_imports)]
use mips::*;

use syncrim::common::{ComponentStore, SimState};

fn main() {
    let cs = ComponentStore::load_file("mips.json");

    let sim_state = SimState::new(&cs);
    println!("SimState {:#?}", sim_state.lens_values);
}
