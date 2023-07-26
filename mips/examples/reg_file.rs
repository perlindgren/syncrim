use mips::components::*;
use std::path::PathBuf;
use syncrim::{
    common::{ComponentStore, Input, SignalFmt, SignalSize},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Constant::rc_new(
                "c_read_reg_1",
                (100.0, 100.0),
                // Format as binary
                (3, SignalFmt::Binary(5)),
            ),
            Constant::rc_new(
                "c_write_addr",
                (100.0, 160.0),
                // Format as hex with padding
                (4, SignalFmt::Binary(5)),
            ),
            Constant::rc_new("c_write_enable", (100.0, 180.0), true),
            Constant::rc_new(
                "c_write_data",
                (100.0, 220.0),
                // Format as hex with padding
                (42, SignalFmt::Hex(SignalSize::_32, true)),
            ),
            Constant::rc_new(
                "c_read_reg_2",
                (100.0, 300.0),
                // Format as binary
                (4, SignalFmt::Binary(5)),
            ),
            // regfile
            RegFile::rc_new(
                "reg_file",
                (300.0, 200.0),
                200.0,
                300.0,
                // ports
                Input::new("c_read_reg_1", "out"),
                Input::new("c_read_reg_2", "out"),
                Input::new("c_write_data", "out"),
                Input::new("c_write_addr", "out"),
                Input::new("c_write_enable", "out"),
            ),
            Probe::rc_new("p_reg_a", (500.0, 100.0), Input::new("reg_file", "reg_a")),
            Probe::rc_new("p_reg_b", (500.0, 300.0), Input::new("reg_file", "reg_b")),
            Wire::rc_new(
                "w_read_reg_1",
                vec![(180.0, 100.0), (200.0, 100.0)],
                Input::new("c_read_reg_1", "out"),
            ),
            Wire::rc_new(
                "w_read_reg_2",
                vec![(180.0, 300.0), (200.0, 300.0)],
                Input::new("c_read_reg_2", "out"),
            ),
            Wire::rc_new(
                "w_write_addr",
                vec![(180.0, 160.0), (200.0, 160.0)],
                Input::new("c_write_addr", "out"),
            ),
            Wire::rc_new(
                "w_write_enable",
                vec![(180.0, 180.0), (200.0, 180.0)],
                Input::new("c_write_enable", "out"),
            ),
            Wire::rc_new(
                "w_write_data",
                vec![(180.0, 220.0), (200.0, 220.0)],
                Input::new("c_write_data", "out"),
            ),
            Wire::rc_new(
                "w_reg_a",
                vec![(400.0, 100.0), (490.0, 100.0)],
                Input::new("reg_file", "reg_a"),
            ),
            Wire::rc_new(
                "w_reg_b",
                vec![(400.0, 300.0), (490.0, 300.0)],
                Input::new("reg_file", "reg_b"),
            ),
        ],
    };

    let path = PathBuf::from("reg_file.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
