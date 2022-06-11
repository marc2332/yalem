fn main() {
    yalem::run(
        App::new().with_window(
            Window::new()
                .with_title("A")
                .root(Padding::from(
                    PaddingBuilder::new((10.0, 10.0, 10.0, 10.0)).child(List::from(
                        ListBuilder::new()
                            .child(Button::from(
                                ButtonBuilder::new()
                                    .background(Color::RED)
                                    .width(80.0)
                                    .height(20.0)
                                    .child(Padding::from(
                                        PaddingBuilder::new((10.0, 10.0, 10.0, 10.0)).child(
                                            Text::from(
                                                TextBuilder::new("Hello World")
                                                    .color(Color::from_rgb(240, 240, 240)),
                                            ),
                                        ),
                                    )),
                            ))
                            .child(Padding::from(
                                PaddingBuilder::new((10.0, 10.0, 10.0, 10.0)).child(Button::from(
                                    ButtonBuilder::new()
                                        .background(Color::BLACK)
                                        .width(100.0)
                                        .height(30.0)
                                        .child(Padding::from(
                                            PaddingBuilder::new((20.0, 20.0, 15.0, 15.0)).child(
                                                Text::from(
                                                    TextBuilder::new("Hello Earth")
                                                        .color(Color::YELLOW),
                                                ),
                                            ),
                                        )),
                                )),
                            )),
                    )),
                )),
        ),
    )
}

use button::{Button, ButtonBuilder};
use glutin::event_loop::EventLoopProxy;

use list::{List, ListBuilder};
use padding::{Padding, PaddingBuilder};
use skia_safe::{Canvas, Color};
use text::{Text, TextBuilder};

mod button;
mod list;
mod padding;
mod renderer;
mod text;
mod triangle;

#[derive(Clone)]
struct Context {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

trait Widget {
    fn draw(&self, canvas: &mut Canvas, context: Context);
}

trait StyledWidget {
    fn background(self, color: Color) -> Self;

    fn color(self, color: Color) -> Self;
}

trait AppWindow {
    fn run(&mut self, proxy: EventLoopProxy<()>);
}

pub struct App {
    windows: Vec<Window>,
}

impl App {
    pub fn new() -> Self {
        Self { windows: vec![] }
    }

    fn with_window(mut self, window: Window) -> Self {
        self.windows.push(window);
        self
    }
}

struct Window {
    title: String,
    root: Option<Box<dyn Widget>>,
}

impl Widget for Window {
    fn draw(&self, canvas: &mut Canvas, ctx: Context) {
        if let Some(root) = &self.root {
            root.draw(canvas, ctx.clone());
        }
    }
}

impl AppWindow for Window {
    fn run(&mut self, proxy: EventLoopProxy<()>) {
        // WIP -  let wb = WindowBuilder::new().with_title(self.title.to_string());
    }
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "Test".to_string(),
            root: None,
        }
    }

    fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    fn root(mut self, child: impl Widget + 'static) -> Self {
        self.root = Some(Box::new(child));
        self
    }
}

mod yalem {

    use std::sync::{Arc, Mutex};

    use gl::types::*;
    use glutin::dpi::PhysicalSize;
    use glutin::window::WindowId;
    use glutin::{
        event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        GlProfile,
    };
    use skia_safe::Color;
    use skia_safe::{
        gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
        ColorType, Surface,
    };

    use crate::{App, Context, Widget, Window as AppWindow};

    pub fn run(app: App) {
        type WindowedContext =
            glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

        let el = EventLoop::new();

        // Guarantee the drop order inside the FnMut closure. `WindowedContext` _must_ be dropped after
        // `DirectContext`.
        //
        // https://github.com/rust-skia/rust-skia/issues/476
        struct Env {
            surface: Surface,
            gr_context: skia_safe::gpu::DirectContext,
            windowed_context: WindowedContext,
            yalem_window: AppWindow,
            fb_info: FramebufferInfo,
        }

        impl Env {
            pub fn redraw(&mut self) {
                let canvas = self.surface.canvas();
                canvas.clear(Color::WHITE);
                self.yalem_window.draw(
                    canvas,
                    Context {
                        x: 0.0,
                        y: 0.0,
                        height: 0.0,
                        width: 0.0,
                    },
                );
                self.surface.canvas();
                self.gr_context.flush(None);
                self.windowed_context
                    .swap_buffers()
                    .unwrap();
            }
        }

        let wins = Arc::new(Mutex::new(vec![]));

        for win in app.windows {
            let wb = WindowBuilder::new().with_title(win.title.clone());

            let cb = glutin::ContextBuilder::new()
                .with_depth_buffer(0)
                .with_stencil_buffer(8)
                .with_pixel_format(24, 8)
                .with_gl_profile(GlProfile::Core);

            #[cfg(not(feature = "wayland"))]
            let cb = cb.with_double_buffer(Some(true));

            let windowed_context = cb.build_windowed(wb, &el).unwrap();

            let windowed_context = unsafe { windowed_context.make_current().unwrap() };

            let pixel_format = windowed_context.get_pixel_format();

            println!(
                "Pixel format of the window's GL context: {:?}",
                pixel_format
            );

            gl::load_with(|s| windowed_context.get_proc_address(s));

            let fb_info = {
                let mut fboid: GLint = 0;
                unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

                FramebufferInfo {
                    fboid: fboid.try_into().unwrap(),
                    format: skia_safe::gpu::gl::Format::RGBA8.into(),
                }
            };

            let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

            windowed_context
                .window()
                .set_inner_size(PhysicalSize::<u32>::new(200, 200));

            let surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
            // let sf = windowed_context.window().scale_factor() as f32;
            // surface.canvas().scale((sf, sf));

            let env = Env {
                surface,
                gr_context,
                windowed_context,
                fb_info,
                yalem_window: win,
            };

            wins.lock()
                .unwrap()
                .push(Arc::new(Mutex::new(env)))
        }

        fn create_surface(
            windowed_context: &WindowedContext,
            fb_info: &FramebufferInfo,
            gr_context: &mut skia_safe::gpu::DirectContext,
        ) -> skia_safe::Surface {
            let pixel_format = windowed_context.get_pixel_format();
            let size = windowed_context.window().inner_size();
            let backend_render_target = BackendRenderTarget::new_gl(
                (
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                ),
                pixel_format
                    .multisampling
                    .map(|s| s.try_into().unwrap()),
                pixel_format
                    .stencil_bits
                    .try_into()
                    .unwrap(),
                *fb_info,
            );
            Surface::from_backend_render_target(
                gr_context,
                &backend_render_target,
                SurfaceOrigin::BottomLeft,
                ColorType::RGBA8888,
                None,
                None,
            )
            .unwrap()
        }

        let get_window_context = move |window_id: WindowId| -> Option<Arc<Mutex<Env>>> {
            let mut win = None;
            for env in &*wins.lock().unwrap() {
                if env
                    .lock()
                    .unwrap()
                    .windowed_context
                    .window()
                    .id()
                    == window_id
                {
                    win = Some(env.clone())
                }
            }

            win
        };

        el.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            #[allow(deprecated)]
            match event {
                Event::LoopDestroyed => {}
                Event::WindowEvent { event, window_id } => match event {
                    WindowEvent::Resized(physical_size) => {
                        let result = get_window_context(window_id);
                        if let Some(env) = result {
                            let mut env = env.lock().unwrap();
                            let mut context = env.gr_context.clone();
                            env.surface =
                                create_surface(&env.windowed_context, &env.fb_info, &mut context);
                            env.windowed_context
                                .resize(physical_size)
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        if modifiers.logo() {
                            if let Some(VirtualKeyCode::Q) = virtual_keycode {
                                *control_flow = ControlFlow::Exit;
                            }
                        }

                        let result = get_window_context(window_id);
                        if let Some(env) = result {
                            let env = env.lock().unwrap();
                            env.windowed_context
                                .window()
                                .request_redraw();
                        }
                    }
                    _ => (),
                },
                Event::RedrawRequested(window_id) => {
                    let result = get_window_context(window_id);
                    if let Some(env) = result {
                        let mut env = env.lock().unwrap();
                        env.redraw();
                    }
                }
                _ => (),
            }
        });
    }
}
