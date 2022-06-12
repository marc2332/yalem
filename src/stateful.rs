use skia_safe::{Canvas};

use crate::{Context, Widget};

pub struct StateContext {

}

impl StateContext {
    pub fn new() -> Self {
        Self {  }
    }
}

pub struct Stateful {
    render: fn(&StateContext) -> Box<dyn Widget>,
    state_ctx: StateContext
}

impl Stateful {
    pub fn new(render: fn(&StateContext) -> Box<dyn Widget> ) -> Self {
        Self { render, state_ctx: StateContext::new() }
    }
}

impl Widget for Stateful {
    fn get_size(&self, ctx: Context) -> (f32, f32) {
        (self.render)(&self.state_ctx).get_size(ctx)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        (self.render)(&self.state_ctx).draw(canvas, ctx)
    }
}

