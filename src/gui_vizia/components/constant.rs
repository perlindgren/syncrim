use crate::{
    components::Constant,
    gui_vizia::{ViziaComponent, V},
};
use vizia::prelude::*;

#[typetag::serde]
impl ViziaComponent for Constant {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| Label::new(cx, &format!("{}", self.value)))
            .left(Pixels(self.pos.0 - 10.0))
            .top(Pixels(self.pos.1 - 10.0))
            .background_color(Color::lightblue())
    }
}
