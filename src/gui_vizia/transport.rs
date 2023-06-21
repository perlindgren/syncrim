use crate::gui_vizia::{Gui, GuiEvent};
use vizia::{icons, prelude::*};

pub(crate) struct Transport {}

impl View for Transport {}

impl Transport {
    pub(crate) fn new(cx: &mut Context) -> Handle<Self> {
        View::build(Transport {}, cx, |cx| {
            HStack::new(cx, |cx| {
                // Reset
                Button::new(
                    cx,
                    |ex| ex.emit(GuiEvent::Reset),
                    |cx| Label::new(cx, icons::ICON_PLAYER_SKIP_BACK),
                )
                .tooltip(|cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Reset");
                        Label::new(cx, " Shift + Ctrl + F5").class("tt_shortcut");
                    })
                    .size(Auto);
                });

                // UnClock (step back)
                Button::new(
                    cx,
                    |ex| ex.emit(GuiEvent::UnClock),
                    |cx| Label::new(cx, icons::ICON_CHEVRON_LEFT),
                )
                .tooltip(|cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "UnClock");
                        Label::new(cx, " Shift + F10").class("tt_shortcut");
                    })
                    .size(Auto);
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
                            Gui::pause.map(|pause| {
                                if *pause {
                                    icons::ICON_PLAYER_PLAY
                                } else {
                                    icons::ICON_PLAYER_PLAY_FILLED
                                }
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
                            Gui::pause.map(|pause| {
                                if *pause {
                                    icons::ICON_PLAYER_PAUSE_FILLED
                                } else {
                                    icons::ICON_PLAYER_PAUSE
                                }
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
            })
            .col_between(Pixels(5.0))
            .size(Auto);
        })
    }
}
