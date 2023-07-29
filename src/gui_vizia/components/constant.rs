use crate::{
    common::{Component, ViziaComponent, V},
    components::Constant,
};

use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for Constant {
    // create view
    fn view<'a>(&'a self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self.get_id_ports(), |cx| {
            Label::new(cx, &format!("{}", self.value))
        })
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 10.0))
        .background_color(Color::lightblue())
    }
}
