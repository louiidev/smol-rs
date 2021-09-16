pub mod asset_store;
mod camera;
pub mod collision;
pub mod color;
pub mod errors;
pub mod gfx;
pub mod input;
pub mod renderer;
pub mod transform;
pub mod window;

#[cfg(feature = "vulkan")]
mod vulkan;

pub type Keycode = sdl2::keyboard::Keycode;
pub type MouseButton = sdl2::mouse::MouseButton;
use crate::asset_store::AssetStore;
pub use crate::color::*;
use crate::gfx::build_window;
use crate::input::Input;
pub use crate::renderer::shapes::*;
use crate::renderer::Renderer;
pub use crate::transform::*;
use math::Vector;
use math::Vector2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::video::Window;
use sdl2::EventPump;
use spin_sleep::LoopHelper;

pub use nalgebra_glm as glm;

pub use nalgebra as math;

pub const RENDER_SCALE: f32 = 2.;

pub const RENDER_RES_W: i32 = 640;
pub const RENDER_RES_H: i32 = 360;
pub const BASE_RES_W: f32 = RENDER_RES_W as f32 * RENDER_SCALE;
pub const BASE_RES_H: f32 = RENDER_RES_H as f32 * RENDER_SCALE;

pub struct AppSettings {
    pub size: Vector2<i32>,
    pub target_fps: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            size: Vector::from([1280, 720]),
            target_fps: 60.,
        }
    }
}

pub struct App {
    event_pump: EventPump,
    loop_helper: LoopHelper,
    window: Window,
    running: bool,
    pub renderer: Renderer,
    pub asset_store: AssetStore,
    pub input: Input,
    pub delta: f32,
    pub frame_rate: f32,

    #[cfg(feature = "opengl")]
    _gl_context: sdl2::video::GLContext,
}

impl App {
    pub fn new(settings: AppSettings) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let (window, _gl_context) = build_window(&sdl_context, &settings);
        let renderer = Renderer::new(settings.size);
        let event_pump = sdl_context.event_pump().unwrap();

        let loop_helper = LoopHelper::builder()
            .report_interval_s(0.5) // report every half a second
            .build_with_target_rate(settings.target_fps); // limit to 250 FPS if possible

        Self {
            running: true,
            event_pump,
            loop_helper,
            window,
            renderer,
            asset_store: AssetStore::default(),
            delta: 1. / 60.,
            frame_rate: 60.,
            _gl_context,
            input: Input::new(),
        }
    }

    pub fn is_running(&mut self) -> bool {
        let delta = self.loop_helper.loop_start().as_secs_f32();
        self.delta = delta;
        if let Some(fps) = self.loop_helper.report_rate() {
            self.frame_rate = fps.round() as f32;
        }

        self.running
    }

    pub fn end_scene(&mut self) {
        self.renderer.render(); // render batch
        self.renderer.swap_buffer(&self.window);
        let mut mouse_scroll_direction = 0;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        self.renderer.context.resize_window(width, height)
                    }
                    _ => {}
                },
                Event::MouseWheel { y, .. } => mouse_scroll_direction = y,
                _ => {}
            }
        }

        self.input.set_keys(&self.event_pump);
        self.input
            .set_mouse_state(&self.event_pump, mouse_scroll_direction);
        self.loop_helper.loop_sleep();
    }
}
