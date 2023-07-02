use clap::Parser;
use std::path::PathBuf;
use syncrim::{common::ComponentStore, gui_vizia::gui};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long)]
    model: String,
}

fn main() {
    let args = Args::parse();
    let path = PathBuf::from(args.model);
    let cs = ComponentStore::load_file(&path);
    gui(&cs, &path);
}
