use std::path::PathBuf;
use syncrim::{
    common::{Component, ComponentStore, Input},
    components::*,
    fern::fern_setup,
    signal::SignalUnsigned,
};

fn main() {
    fern_setup();

    // let mut c = Constant::rc_new("constant", (100.0, 100.0), MemCtrl::Write as SignalUnsigned);
    // let out = c.get_id_ports();

    let cs = ComponentStore {
        store: vec![Constant::rc_new("probe_edit", (100.0, 100.0), 0)],
    };

    let path = PathBuf::from("probe_constant.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
