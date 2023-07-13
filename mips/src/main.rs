#[allow(unused_imports)]
use mips::components::*;
use std::path::PathBuf;
use syncrim::common::ComponentStore;

fn main() {
    let path = PathBuf::from("mips_single_cycle.json");
    let _cs = ComponentStore::load_file(&path);

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&_cs, &path);
}
