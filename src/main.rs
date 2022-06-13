#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn stuff() -> impl Widget {
    Stateful::new(|_ctx| {
        Box::new(Padding::from(
            PaddingBuilder::new((0.0, 0.0, 0.0, 0.0)).child(List::from(
                ListBuilder::new()
                    .child(Text::from(
                        TextBuilder::new("Yalum Demo").color(Color::BLACK),
                    ))
                    .child(Button::from(
                        ButtonBuilder::new()
                            .background(Color::RED)
                            .child(Expand::from(ExpandBuilder::new().child(Text::from(
                                TextBuilder::new("Expanded").color(Color::from_rgb(240, 240, 240)),
                            )))),
                    ))
                    .child(Button::from(
                        ButtonBuilder::new()
                            .background(Color::BLUE)
                            .child(Text::from(
                                TextBuilder::new("Fixed width and height")
                                    .color(Color::from_rgb(240, 240, 240)),
                            ))
                            .width(200.0)
                            .height(50.0),
                    ))
                    .child(Padding::from(
                        PaddingBuilder::new((0.0, 0.0, 0.0, 0.0)).child(Button::from(
                            ButtonBuilder::new()
                                .background(Color::BLACK)
                                .child(Padding::from(
                                    PaddingBuilder::new((50.0, 50.0, 25.0, 25.0)).child(
                                        Text::from(
                                            TextBuilder::new("Fixed paddings").color(Color::YELLOW),
                                        ),
                                    ),
                                )),
                        )),
                    ))
                    .child(Padding::from(
                        PaddingBuilder::new((10.0, 10.0, 10.0, 10.0)).child(Button::from(
                            ButtonBuilder::new()
                                .background(Color::GREEN)
                                .child(Expand::from(
                                    ExpandBuilder::new().child(Center::from(
                                        CenterBuilder::new().child(Text::from(
                                            TextBuilder::new(
                                                "Expanded horizontally + centered + paddings",
                                            )
                                            .color(Color::BLACK)
                                            .align(Align::Center),
                                        )),
                                    )),
                                )),
                        )),
                    ))
                    .child(Button::from(
                        ButtonBuilder::new()
                            .background(Color::MAGENTA)
                            .child(Expand::from(
                                ExpandBuilder::new()
                                    .child(Center::from(
                                        CenterBuilder::new()
                                            .child(List::from(
                                                ListBuilder::new()
                                                    .child(Text::from(
                                                        TextBuilder::new(
                                                            "Expanded both sides and centered",
                                                        )
                                                        .color(Color::BLACK)
                                                        .align(Align::Center),
                                                    ))
                                                    .child(Triangle::from(TriangleBuilder::new())),
                                            ))
                                            .direction(Direction::Both),
                                    ))
                                    .direction(Direction::Both),
                            )),
                    )),
            )),
        ))
    })
}

fn main() {
    yalem::run(
        App::new().with_window(
            Window::new()
                .with_title("Yalum Demo")
                .root(stuff()),
        ),
    )
}

use button::{Button, ButtonBuilder};
use center::{Center, CenterBuilder, Direction};
use expand::{Expand, ExpandBuilder};
use glutin::event_loop::EventLoopProxy;

use list::{List, ListBuilder};
use padding::{Padding, PaddingBuilder};
use skia_safe::{utils::text_utils::Align, Canvas, Color};
use stateful::Stateful;
use text::{Text, TextBuilder};
use triangle::{Triangle, TriangleBuilder};

mod button;
mod center;
mod expand;
mod list;
mod padding;
mod stateful;
mod text;
mod triangle;

#[derive(Clone, Debug)]
pub struct Context {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub trait Widget {
    fn draw(&mut self, canvas: &mut Canvas, context: Context);

    fn get_size(&self, ctx: Context) -> (f32, f32) {
        (ctx.width, ctx.height)
    }
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
    fn draw(&mut self, canvas: &mut Canvas, ctx: Context) {
        if let Some(root) = &mut self.root {
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
                let win_size = self
                    .windowed_context
                    .window()
                    .inner_size();
                let canvas = self.surface.canvas();
                canvas.clear(Color::WHITE);
                self.yalem_window.draw(
                    canvas,
                    Context {
                        x: 0.0,
                        y: 0.0,
                        height: win_size.height as f32,
                        width: win_size.width as f32,
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
                .set_inner_size(PhysicalSize::<u32>::new(300, 300));

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
                    WindowEvent::CloseRequested => {
                        // should only remove one window
                        *control_flow = ControlFlow::Exit
                    }
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
