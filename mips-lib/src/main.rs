use clap::Parser;
// The trait impls from here are used dynamically when json is loaded, so this is actually used
#[allow(unused_imports)]
use mips_lib::*;
use std::{path::PathBuf, rc::Rc};
#[cfg(feature = "gui-egui")]
use std::str::FromStr;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{common::ComponentStore, fern::fern_setup};
// this is actually loaded dynamically, so it is used.
#[allow(unused_imports)]
use mips_lib::components::*;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long)]
    model: Option<PathBuf>,
    #[arg(short, long)]
    inbuilt: Option<InBuilt>,
    #[arg(short, long)]
    elf_file: Option<PathBuf>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum InBuilt {
    SingleCycle,
    Pipe,
    Extended,
}

const BUILT_IN_MODELS: [(&str, &str); 3] = [
    (
        "single cycle mips",
        include_str!("../mips_single_cycle.json"),
    ),
    ("piped mips", include_str!("../mips_pipe.json")),
    ("extended mips", include_str!("../mips_pipe_ex.json")),
];

fn main() {
    fern_setup();
    let args = Args::parse();
    let cs;
    if let Some(path) = args.model {
        cs = ComponentStore::load_file(&path).unwrap_or(ComponentStore { store: vec![] });
    } else if let Some(model) = args.inbuilt {

        let mut cs_tmp;
        match model {
            InBuilt::SingleCycle => {
                cs_tmp = ComponentStore::load(BUILT_IN_MODELS[0].1).unwrap()
            },
            InBuilt::Pipe => {
                cs_tmp = ComponentStore::load(BUILT_IN_MODELS[1].1).unwrap()
            },
            InBuilt::Extended => {
                cs_tmp = ComponentStore::load(BUILT_IN_MODELS[2].1).unwrap()
            },
        }

        // get phys mems position
        let pos = cs_tmp.store
            .iter()
            .find(|c| c.get_id_ports().0 == "phys_mem")
            .unwrap().get_pos();

        // replace the default mem with our own
        let phys_mem = Rc::new(PhysicalMem::new("phys_mem", pos));
        *cs_tmp.store
            .iter_mut()
            .find(|c| c.get_id_ports().0 == "phys_mem")
            .unwrap() = phys_mem.clone();

        if let Some(elf_path) = args.elf_file {
            phys_mem.load_file(&elf_path).unwrap();
        }

        cs = cs_tmp;
    } else {
        cs = ComponentStore { store: vec![] }
    }

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::Gui::new(cs, &PathBuf::from_str("./new_file.json").unwrap(), Library::default()).unwrap().with_inbuilt(&BUILT_IN_MODELS).run().unwrap();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);

    #[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
    syncrim::common::Simulator::new(cs).unwrap();
}
