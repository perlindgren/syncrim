use crate::common::{ComponentStore, SimState, Simulator};
use crate::gui::menu::Menu;
use std::rc::Rc;
use vizia::{
    icons,
    prelude::*,
    vg::{Paint, Path},
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
enum GuiEvent {
    Clock,
    Reset,
    UnClock,
    Play,
    Pause,
    PlayToggle,
}

// The actions that are associated with the key chords.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    Play,
    Pause,
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
        });
    }
}

pub fn gui(cs: &ComponentStore) {
    let (simulator, mut sim_state) = Simulator::new(cs);
    let simulator = Rc::new(simulator);
    // Initial clock to propagate constants
    simulator.clock(&mut sim_state);

    Application::new(move |cx| {
        // Build the keymap.
        Keymap::from(vec![
            (
                KeyChord::new(Modifiers::empty(), Code::F5),
                KeymapEntry::new(Action::Play, |ex| {
                    println!("Action F5");
                    ex.emit(GuiEvent::PlayToggle);
                }),
            ),
            (
                KeyChord::new(Modifiers::SHIFT, Code::F5),
                KeymapEntry::new(Action::Play, |ex| {
                    println!("Action Shift F5");
                    ex.emit(GuiEvent::Pause);
                }),
            ),
            (
                KeyChord::new(Modifiers::SHIFT | Modifiers::CTRL, Code::F5),
                KeymapEntry::new(Action::Play, |ex| {
                    println!("Action Shift Ctrl F5");
                    ex.emit(GuiEvent::Reset);
                }),
            ),
            (
                KeyChord::new(Modifiers::empty(), Code::F10),
                KeymapEntry::new(Action::Play, |ex| {
                    println!("Action F10");
                    ex.emit(GuiEvent::Clock);
                }),
            ),
            (
                KeyChord::new(Modifiers::SHIFT, Code::F10),
                KeymapEntry::new(Action::Play, |ex| {
                    println!("Action Shift F10");
                    ex.emit(GuiEvent::UnClock);
                }),
            ),
        ])
        .build(cx);

        Gui {
            simulator: simulator.clone(),
            state: sim_state,
            history: vec![],
            mode: Mode::Pause,
            is_saved: false,
        }
        .build(cx);

        Menu::new(cx);

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
                Label::new(cx, "Reset");
            });

            // UnClock (step back)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::UnClock),
                |cx| Label::new(cx, icons::ICON_CHEVRON_LEFT),
            )
            .tooltip(|cx| {
                Label::new(cx, "UnClock");
            });

            // Clock (step forward)
            Button::new(
                cx,
                |ex| ex.emit(GuiEvent::Clock),
                |cx| Label::new(cx, icons::ICON_CHEVRON_RIGHT),
            )
            .tooltip(|cx| {
                Label::new(cx, "Clock");
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
                Label::new(cx, "Play");
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
                Label::new(cx, "Pause");
            });
        });
    })
    .title("SyncRim")
    .run();
}

struct GridView {}

impl GridView {
    fn new<F>(cx: &mut Context, content: F) -> Handle<'_, Self>
    where
        F: FnOnce(&mut Context),
    {
        View::build(GridView {}, cx, |cx| content(cx))
    }
}

impl View for GridView {
    fn element(&self) -> Option<&'static str> {
        Some("Grid")
    }

    // draw operates on native pixels
    // bounds is given in scaled format
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();
        // println!("Grid draw {:?}, {}", bounds, cx.scale_factor());

        let unscaled_height = bounds.height() / scale;
        let unscaled_width = bounds.width() / scale;

        let rows: usize = (unscaled_height / 20.0).round() as usize;
        let columns: usize = (unscaled_width / 20.0).round() as usize;

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbaf(0.0, 0.0, 1.0, 0.1));

        paint.set_line_width(cx.logical_to_physical(1.0));

        for r in 0..rows {
            path.move_to(
                bounds.left() + 0.5,
                bounds.top() + r as f32 * 20.0 * scale + 0.5,
            );
            path.line_to(
                bounds.right() + 0.5,
                bounds.top() + r as f32 * 20.0 * scale + 0.5,
            );
        }

        for c in 0..columns {
            path.move_to(
                bounds.left() + c as f32 * 20.0 * scale + 0.5,
                bounds.top() + 0.5,
            );
            path.line_to(
                bounds.left() + c as f32 * 20.0 * scale + 0.5,
                bounds.bottom() + 0.5,
            );
        }

        canvas.stroke_path(&path, &paint);
    }
}
