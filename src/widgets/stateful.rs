use skia_safe::Canvas;

use crate::{Context, Widget, YalemEvent};

pub struct StateContext {}

impl StateContext {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Stateful {
    render: fn(&StateContext) -> Box<dyn Widget>,
    state_ctx: StateContext,
}

impl Stateful {
    pub fn new(render: fn(&StateContext) -> Box<dyn Widget>) -> Self {
        Self {
            render,
            state_ctx: StateContext::new(),
        }
    }
}

impl Widget for Stateful {
    fn send_event(&mut self, event: &YalemEvent) {
        let mut child = (self.render)(&self.state_ctx);
        child.send_event(&event);
    }

    fn get_size(&self, ctx: Context) -> (f32, f32) {
        (self.render)(&self.state_ctx).get_size(ctx)
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        (self.render)(&self.state_ctx).draw(canvas, ctx)
    }
}
