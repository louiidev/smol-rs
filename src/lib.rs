pub mod asset_store;
pub mod camera;
pub mod collision;
pub mod errors;
pub mod gfx;
pub mod input;
pub mod math;
pub mod render;
pub mod render_batch;
pub mod renderer;
pub mod text_render;
pub mod ui;

#[cfg(feature = "vulkan")]
mod vulkan;

// use crate::input::Input;
use std::time::Instant;

#[derive(Debug)]
pub struct TimeStep {
    last_time: Instant,
    delta_time: f64,
    frame_count: u32,
    frame_time: f64,
    last_frame_count: u32,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
            last_frame_count: 0,
        }
    }

    pub fn delta(&mut self) -> f64 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros() as f64 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }

    // provides the framerate in FPS
    pub fn frame_rate(&mut self) -> u32 {
        self.frame_count += 1;
        self.frame_time += self.delta_time;
        let tmp;
        // per second
        if self.frame_time >= 1000.0 {
            tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            self.last_frame_count = tmp;
            return tmp;
        }
        self.last_frame_count
    }
}

pub mod core {
    use super::*;
    use crate::asset_store::AssetStore;
    use crate::camera::Camera;
    use crate::gfx::{build_window, GfxContext};
    use crate::math::*;
    use crate::render::*;
    use glyph_brush::ab_glyph::Rect;
    use lazy_static::lazy_static;
    use sdl2::event::Event;
    use sdl2::event::WindowEvent;
    use sdl2::video::Window;
    use sdl2::EventPump;
    use spin_sleep::LoopHelper;

    pub type Keycode = sdl2::keyboard::Keycode;
    pub type MouseButton = sdl2::mouse::MouseButton;

    pub const RENDER_SCALE: f32 = 2.;

    pub const RENDER_RES_W: i32 = 640;
    pub const RENDER_RES_H: i32 = 360;
    pub const BASE_RES_W: f32 = RENDER_RES_W as f32 * RENDER_SCALE;
    pub const BASE_RES_H: f32 = RENDER_RES_H as f32 * RENDER_SCALE;

    pub struct AppSettings {
        pub size: Vec2Int,
        pub target_fps: f32,
    }

    impl Default for AppSettings {
        fn default() -> Self {
            Self {
                size: Vec2Int::new(1280, 720),
                target_fps: 60.,
            }
        }
    }

    pub struct App {
        event_pump: EventPump,
        loop_helper: LoopHelper,
        window: Window,
        pub running: bool,
        pub gfx: GfxContext,
        pub asset_store: AssetStore,
    }

    impl App {
        pub fn new(settings: AppSettings) -> Self {
            let sdl_context = sdl2::init().unwrap();
            let window = build_window(&sdl_context, &settings);

            let event_pump = sdl_context.event_pump().unwrap();

            let loop_helper = LoopHelper::builder()
                .report_interval_s(0.5) // report every half a second
                .build_with_target_rate(settings.target_fps); // limit to 250 FPS if possible

            Self {
                running: true,
                event_pump,
                loop_helper,
                window,
                gfx: GfxContext::new(),
                asset_store: AssetStore::default(),
            }
        }

        pub fn end_render(&mut self) {
            self.window.gl_swap_window();
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
                        WindowEvent::Resized(w, h) => {}
                        _ => {}
                    },
                    _ => {}
                }
            }

            // ctx.input.set_keys(&ctx.event_pump);
            // ctx.input.set_mouse_state(&ctx.event_pump);
            self.loop_helper.loop_sleep();
        }
    }

    // pub struct Smol {
    //     pub running: bool,
    //     window: Window,
    //     event_pump: EventPump,
    //     _gl_context: GLContext,
    //     time_step: TimeStep,
    //     delta_time: f32,
    //     pub window_size: Vec2Int,
    //     loop_helper: LoopHelper,
    //     timer_state: TimerState,
    //     camera: Camera,
    // }
}
