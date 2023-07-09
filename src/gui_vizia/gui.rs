use crate::common::{ComponentStore, Simulator};
use crate::gui_vizia::{grid::Grid, keymap::init_keymap, menu::Menu, transport::Transport};
use rfd::FileDialog;
use std::path::PathBuf;
use vizia::prelude::*;

#[derive(Lens, Clone)]
pub struct GuiData {
    pub path: PathBuf,
    pub clock: usize,
    pub simulator: Simulator,
    pub pause: bool,
    pub is_saved: bool,
    pub show_about: bool,
    pub selected_id: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum GuiEvent {
    Open,
    ReOpen,
    Clock,
    Reset,
    UnClock,
    Play,
    Pause,
    PlayToggle,
    Preferences,
    ShowAbout,
    HideAbout,
    // SelectComponent(usize),
}

impl Model for GuiData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        #[allow(clippy::single_match)]
        event.map(|window_event, meta| match window_event {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            WindowEvent::WindowClose => {
                if !self.is_saved {
                    // self.show_dialog = true;
                    meta.consume();
                    self.is_saved = true;
                }
            }
            _ => {}
        });

        event.map(|app_event, _meta| match app_event {
            GuiEvent::Open => {
                let files = FileDialog::new().add_filter("json", &["json"]).pick_file();
                println!("files {:?}", files);
                if let Some(path_buf) = files {
                    self.path = path_buf;
                    self.open();
                }
            }
            GuiEvent::ReOpen => self.open(),
            GuiEvent::Clock => self.simulator.clock(&mut self.clock),
            GuiEvent::UnClock => self.simulator.un_clock(&mut self.clock),
            GuiEvent::Reset => {
                self.simulator.reset(&mut self.clock);
                self.pause = true;
            }
            GuiEvent::Play => self.pause = false,
            GuiEvent::Pause => self.pause = true,
            GuiEvent::PlayToggle => self.pause = !self.pause,
            GuiEvent::Preferences => println!("Preferences"),
            GuiEvent::ShowAbout => self.show_about = true,
            GuiEvent::HideAbout => self.show_about = false,
            // GuiEvent::SelectComponent(index) => self.selected_id = *index,
        });
    }
}

impl GuiData {
    fn open(&mut self) {
        // Re-Open model
        println!("open path {:?}", self.path);
        let cs = Box::new(ComponentStore::load_file(&self.path));
        let simulator = Simulator::new(&cs, &mut self.clock);

        self.simulator = simulator;

        println!("opened");
    }
}

pub fn gui(cs: &ComponentStore, path: &PathBuf) {
    let mut clock = 0;
    let simulator = Simulator::new(cs, &mut clock);
    let path = path.to_owned();
    simulator.save_dot(&path);

    Application::new(move |cx| {
        cx.add_stylesheet(include_style!("src/gui_vizia/style.css"))
            .expect("Failed to add stylesheet");

        // Create keymap
        init_keymap(cx);

        GuiData {
            path,
            clock,
            simulator,
            pause: true,
            is_saved: false,
            show_about: false,
            selected_id: 0,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Menu
            Menu::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Transport::new(cx).size(Auto);
                    Label::new(cx, GuiData::clock.map(|clock| format!("Clock #{}", clock)))
                        .top(Stretch(1.0))
                        .bottom(Stretch(1.0))
                        .height(Auto);
                })
                .col_between(Pixels(10.0))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .size(Auto);
            })
            .background_color(Color::lightgray())
            .height(Auto);

            Grid::new(cx, |cx| {
                // (re-)bind all components when simulator changed
                Binding::new(
                    cx,
                    GuiData::simulator.then(Simulator::ordered_components),
                    |cx, wrapper_oc| {
                        let oc = wrapper_oc.get(cx);
                        for c in oc {
                            c.view(cx);
                        }
                    },
                )
            });

            //
            // HStack::new(cx, |cx| {
            // Component selector
            // PickList::new(cx, Gui::component_ids, Gui::selected_id, true)
            //     .on_select(|cx, index| cx.emit(GuiEvent::SelectComponent(index)))
            //     .width(Pixels(140.0));

            // About
            Popup::new(cx, GuiData::show_about, true, |cx| {
                Label::new(cx, "About").class("title");
                Label::new(cx, "SyncRim 0.1.0");
                Label::new(cx, "per.lindgren@ltu.se");

                Button::new(
                    cx,
                    |cx| cx.emit(GuiEvent::HideAbout),
                    |cx| Label::new(cx, "Ok"),
                )
                .class("accent");
            })
            .on_blur(|cx| cx.emit(GuiEvent::HideAbout))
            .class("modal");
        });
    })
    .title("SyncRim")
    .run();
}
