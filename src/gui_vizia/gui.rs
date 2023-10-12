use crate::{
    common::{ComponentStore, Simulator},
    gui_vizia::{
        grid::Grid, keymap::init_keymap, menu::Menu, tooltip::new_component_tooltip,
        transport::Transport,
    },
};
use rfd::FileDialog;
use std::collections::HashSet;
use std::path::PathBuf;
use vizia::{icons, prelude::*};

use log::*;

#[derive(Lens, Clone)]
pub struct GuiData {
    pub path: PathBuf,
    pub simulator: Simulator,
    pub is_saved: bool,
    pub show_about: bool,
    pub selected_id: usize,
    pub visible: HashSet<usize>,
    pub expanded: HashSet<usize>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GuiEvent {
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
    ShowLeftPanel(usize),
    HideLeftPanel(usize),
    ToggleExpandLeftPanel(usize),
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
                trace!("files {:?}", files);
                if let Some(path_buf) = files {
                    self.path = path_buf;
                    self.open();
                }
            }
            GuiEvent::ReOpen => self.open(),
            GuiEvent::Clock => self.simulator.clock(),
            GuiEvent::UnClock => self.simulator.un_clock(),
            GuiEvent::Reset => {
                self.simulator.reset();
            }
            GuiEvent::Play => {
                self.simulator.running = true;
                self.simulator.clock();
            }
            GuiEvent::Pause => {
                self.simulator.running = false;
            }
            GuiEvent::PlayToggle => {
                self.simulator.running = !self.simulator.running;
                if self.simulator.running {
                    self.simulator.clock();
                }
            }
            GuiEvent::Preferences => trace!("Preferences"),
            GuiEvent::ShowAbout => self.show_about = true,
            GuiEvent::HideAbout => self.show_about = false,
            GuiEvent::ShowLeftPanel(i) => {
                trace!("Show Left Panel {:?}", i);
                self.visible.insert(*i);
                trace!("visible {:?}", self.visible);
            }
            GuiEvent::HideLeftPanel(i) => {
                trace!("Hide Left Panel {:?}", i);
                self.visible.remove(i);
            }
            GuiEvent::ToggleExpandLeftPanel(i) => {
                trace!("Toggle Expand Left Panel {:?}", i);
                trace!("expanded {:?}", self.visible);
                if self.expanded.contains(i) {
                    self.expanded.remove(i);
                } else {
                    self.expanded.insert(*i);
                }
            }
        });
    }
}

impl GuiData {
    fn open(&mut self) {
        // Re-Open model
        trace!("open path {:?}", self.path);
        let cs = ComponentStore::load_file(&self.path);
        let simulator = Simulator::new(cs);
        match simulator {
            Ok(s) => {
                self.simulator = s;
                trace!("opened");
            }
            Err(e) => trace!("File failed to open due to errors with simulator {}", e),
        }
    }
}

pub fn gui(cs: ComponentStore, path: &PathBuf) {
    let simulator = Simulator::new(cs).unwrap();
    let path = path.to_owned();
    simulator.save_dot(&path);

    Application::new(move |cx| {
        cx.add_stylesheet(include_style!("src/gui_vizia/style.css"))
            .expect("Failed to add stylesheet");

        // Create keymap
        init_keymap(cx);

        GuiData {
            path,
            simulator,
            is_saved: false,
            show_about: false,
            selected_id: 0,
            visible: HashSet::new(),
            expanded: HashSet::new(),
        }
        .build(cx);

        // bind a clock change to allow free-running mode
        Binding::new(cx, GuiData::simulator.then(Simulator::cycle), |cx, lens| {
            // Retrieve the data from context
            let cycle = lens.get(cx);
            trace!("cycle changed {}", cycle);

            let simulator = GuiData::simulator.get(cx);
            if simulator.running {
                trace!("send clock event");
                cx.emit(GuiEvent::Clock);
            };
        });

        VStack::new(cx, |cx| {
            // Menu
            Menu::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Transport::new(cx).size(Auto);
                    Label::new(
                        cx,
                        GuiData::simulator.map(|simulator| format!("Cycle #{:?}", simulator.cycle)),
                    )
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

            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    // Left panel
                    Binding::new(
                        cx,
                        GuiData::simulator.then(Simulator::ordered_components),
                        |cx, wrapper_oc| {
                            VStack::new(cx, |cx| {
                                Label::new(cx, "Left").top(Pixels(0.0));
                                let oc = wrapper_oc.get(cx);
                                for (i, c) in oc.iter().enumerate() {
                                    VStack::new(cx, |cx| {
                                        // left pane bar
                                        HStack::new(cx, move |cx| {
                                            Button::new(
                                                cx,
                                                move |cx| {
                                                    cx.emit(GuiEvent::ToggleExpandLeftPanel(i))
                                                },
                                                |cx| {
                                                    Label::new(
                                                        cx,
                                                        GuiData::expanded.map(move |expanded| {
                                                            if expanded.contains(&i) {
                                                                // expanded
                                                                icons::ICON_CHEVRON_DOWN
                                                            } else {
                                                                // folded
                                                                icons::ICON_CHEVRON_RIGHT
                                                            }
                                                        }),
                                                    )
                                                    .class("icon")
                                                },
                                            )
                                            .left(Pixels(5.0))
                                            .top(Stretch(1.0))
                                            .bottom(Stretch(1.0))
                                            .right(Stretch(1.0))
                                            .size(Auto);
                                            let (id, _) = c.get_id_ports();

                                            Label::new(cx, &format!("Instance: {}", &id))
                                                .left(Pixels(5.0))
                                                .top(Stretch(1.0))
                                                .bottom(Stretch(1.0))
                                                .right(Stretch(1.0))
                                                .size(Auto);

                                            Button::new(
                                                cx,
                                                move |cx| cx.emit(GuiEvent::HideLeftPanel(i)),
                                                |cx| Label::new(cx, icons::ICON_X).class("icon"),
                                            )
                                            .right(Pixels(1.0))
                                            .top(Pixels(1.0))
                                            .bottom(Pixels(1.0));
                                        })
                                        .background_color(Color::lightgrey())
                                        .height(Auto)
                                        .border_color(Color::darkgray())
                                        .border_width(Pixels(1.0));
                                        // left view expanded or folded
                                        VStack::new(cx, |cx| c.left_view(cx)).display(
                                            GuiData::expanded.map(move |hs_expanded| {
                                                if hs_expanded.contains(&i) {
                                                    Display::Flex
                                                } else {
                                                    Display::None
                                                }
                                            }),
                                        );
                                    })
                                    .display(
                                        GuiData::visible.map(move |hs_visible| {
                                            if hs_visible.contains(&i) {
                                                Display::Flex
                                            } else {
                                                Display::None
                                            }
                                        }),
                                    );
                                }
                            })
                            .border_color(Color::black())
                            .border_width(Pixels(1.0));
                        },
                    );
                })
                .width(Pixels(400.0));

                // Mid panel
                ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                    // Grid area
                    Grid::new(cx, |cx| {
                        // (re-)bind all components when simulator changed
                        Binding::new(
                            cx,
                            GuiData::simulator.then(Simulator::ordered_components),
                            |cx, wrapper_oc| {
                                VStack::new(cx, |cx| {
                                    let oc = wrapper_oc.get(cx);
                                    for (i, c) in oc.into_iter().enumerate() {
                                        trace!("build view comp id {}", i);
                                        c.view(cx)
                                            .tooltip(|cx| new_component_tooltip(cx, &*c))
                                            .hoverable(true)
                                            .position_type(PositionType::SelfDirected)
                                            .on_mouse_down(move |ex, button| {
                                                trace!("on_mouse_down");
                                                match button {
                                                    MouseButton::Left => {
                                                        trace!("LEFT");
                                                        ex.emit(PopupEvent::Switch)
                                                    }
                                                    MouseButton::Middle => {
                                                        trace!("MIDDLE ")
                                                    }
                                                    MouseButton::Right => {
                                                        trace!("RIGHT {:?}", i);
                                                        ex.emit(GuiEvent::ShowLeftPanel(i))
                                                    }
                                                    _ => {}
                                                }
                                            });
                                    }
                                })
                                .border_color(Color::black())
                                .border_width(Pixels(1.0))
                                .overflow(Overflow::Hidden);
                            },
                        )
                    })
                    .height(Pixels(1080.0))
                    .width(Pixels(1920.0));
                })
                // .size(Units::Pixels(300.0))
                .class("bg-default");

                // Right pane
                Label::new(cx, "Right").top(Pixels(0.0));
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
