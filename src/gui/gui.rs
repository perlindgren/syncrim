use crate::common::{ComponentStore, SimState, Simulator};
use crate::gui::grid::GridView;
use crate::gui::menu::Menu;
use std::rc::Rc;
use vizia::{
    icons,
    prelude::*,
    //vg::{Paint, Path},
};

pub enum Mode {
    Pause,
    Play,
}

#[derive(Lens)]
pub struct Gui {
    pub simulator: Rc<Simulator>,
    pub state: SimState,
    // History, acts like a stack
    pub history: Vec<Vec<u32>>,
    pub mode: Mode,
    pub is_saved: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum GuiEvent {
    Clock,
    Reset,
    UnClock,
    Play,
    Pause,
    PlayToggle,
    Preferences,
}

impl Model for Gui {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
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
            GuiEvent::Clock => {
                // push current state
                self.history.push(self.state.lens_values.clone());
                self.simulator.clock(&mut self.state);
            }
            GuiEvent::Reset => {
                self.simulator.reset(&mut self.state);
                // clear history
                self.history = vec![];
                // make sure its in paused mode
                self.mode = Mode::Pause;
            }
            GuiEvent::UnClock => {
                if let Some(state) = self.history.pop() {
                    // set old state
                    self.state.lens_values = state;
                }
            }
            GuiEvent::Play => self.mode = Mode::Play,
            GuiEvent::Pause => self.mode = Mode::Pause,
            GuiEvent::PlayToggle => {
                self.mode = match self.mode {
                    Mode::Play => Mode::Pause,
                    _ => Mode::Play,
                }
            }
            GuiEvent::Preferences => println!("Preferences"),
        });
    }
}

// const STYLE: &str = r#"

//     .modal {
//         space: 1s;
//         background-color: white;
//         border-radius: 3px;
//         border-width: 1px;
//         border-color: #999999;
//         outer-shadow: 0 3 10 #00000055;
//         overflow: visible;
//         child-space: 10px;
//     }

//     .modal>vstack>label {
//         width: auto;
//         height: auto;
//         space: 5px;
//         child-space: 1s;
//     }

//     .modal button {
//         border-radius: 3px;
//         child-space: 1s;
//     }

//     .modal hstack {
//         child-space: 1s;
//         col-between: 20px;
//     }
// "#;

const STYLE: &str = r#"
    .tt_shortcut {
        color: #c4c4c4;
    }

    submenu.file_menu > popop {
        width: 600px;
    }
"#;

pub fn gui(cs: &ComponentStore) {
    let (simulator, mut sim_state) = Simulator::new(cs);
    let simulator = Rc::new(simulator);
    // Initial clock to propagate constants
    simulator.clock(&mut sim_state);

    Application::new(move |cx| {
        // Styling
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // Create keymap
        crate::gui::keymap::new(cx);

        Gui {
            simulator: simulator.clone(),
            state: sim_state,
            history: vec![],
            mode: Mode::Pause,
            is_saved: false,
        }
        .build(cx);

        Menu::new(cx).background_color(Color::beige()).size(Auto);

        // Grid
        GridView::new(cx, |cx| {
            for c in &simulator.ordered_components {
                c.view(cx, simulator.clone());
            }
        })
        .top(Stretch(1.0))
        .bottom(Stretch(1.0));

        // a label to display the raw state for debugging purpose
        Label::new(
            cx,
            Gui::state
                .then(SimState::lens_values)
                .map(|v| format!("Raw state {:?}", v)),
        );

        HStack::new(cx, |cx| {
            // Reset
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Reset),
                |cx| Label::new(cx, icons::ICON_PLAYER_SKIP_BACK),
            )
            .tooltip(|cx| {
                Label::new(cx, "Reset Shift + Ctrl + F5");
            });

            // UnClock (step back)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::UnClock),
                |cx| Label::new(cx, icons::ICON_CHEVRON_LEFT),
            )
            .tooltip(|cx| {
                Label::new(cx, "UnClock Shift + F10");
            });

            // Clock (step forward)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Clock),
                |cx| Label::new(cx, icons::ICON_CHEVRON_RIGHT),
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Clock");
                    Label::new(cx, " F10").class("tt_shortcut");
                })
                .size(Auto);
            });

            // Play (continuous mode)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Play),
                |cx| {
                    Label::new(
                        cx,
                        Gui::mode.map(|mode| match mode {
                            Mode::Pause => icons::ICON_PLAYER_PLAY,
                            Mode::Play => icons::ICON_PLAYER_PLAY_FILLED,
                        }),
                    )
                },
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Play");
                    Label::new(cx, " F5 (Toggle)").class("tt_shortcut");
                })
                .size(Auto);
            });

            // Pause (step mode)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Pause),
                |cx| {
                    Label::new(
                        cx,
                        Gui::mode.map(|mode| match mode {
                            Mode::Pause => icons::ICON_PLAYER_PAUSE_FILLED,
                            Mode::Play => icons::ICON_PLAYER_PAUSE,
                        }),
                    )
                },
            )
            .tooltip(|cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Pause");
                    Label::new(cx, " F5 (Toggle)").class("tt_shortcut");
                })
                .size(Auto);
            });
        });
    })
    .title("SyncRim")
    .run();
}
