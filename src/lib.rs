pub mod render;
pub mod math;
pub mod events;
pub mod components;
pub mod input;
pub mod texture_packer;
pub mod render_batch;
pub mod systems;
pub mod ai;
pub mod world_setup;
pub mod map;
pub mod text_render;
pub mod ui;
pub mod collision;
pub mod camera;
pub mod pathfinding;
pub mod queries;
pub mod logging;

use std::time::Instant;
use crate::input::Input;

#[derive(Debug)]
pub struct TimeStep {
    last_time:   Instant,
    delta_time:  f64,
    frame_count: u32,
    frame_time:  f64,
    last_frame_count: u32,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time:   Instant::now(),
            delta_time:  0.0,
            frame_count: 0,
            frame_time:  0.0,
            last_frame_count: 0,
        }
    }

    pub fn delta(&mut self) -> f64 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros()
            as f64
            * 0.001;
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
    use glyph_brush::ab_glyph::Rect;
    use lazy_static::lazy_static;
    use spin_sleep::LoopHelper;
    use crate::camera::Camera;
    use crate::render::*;
    use crate::math::*;
    use crate::render::Color;
    use crate::text_render::TextQueueConfig;
    use crate::text_render::TextRenderer;
    use sdl2::video::SwapInterval;
    use sdl2::video::Window;
    use sdl2::EventPump;
    use sdl2::event::Event;
    use sdl2::video::GLContext;
    use sdl2::video::GLProfile;
    use std::sync::Mutex;
    use sdl2::event::WindowEvent;

    pub type Keycode = sdl2::keyboard::Keycode;
    pub type MouseButton = sdl2::mouse::MouseButton;

    pub const RENDER_SCALE: f32 = 2.;

    pub const RENDER_RES_W: i32 = 640;
    pub const RENDER_RES_H: i32 = 360;
    pub const BASE_RES_W: f32 = RENDER_RES_W as f32 * RENDER_SCALE;
    pub const BASE_RES_H: f32 = RENDER_RES_H as f32 * RENDER_SCALE;


    pub fn get_window_scale() -> Vector2 {
        let ctx = get_context();

        Vector2 {
            x: ctx.window_size.x as f32 / RENDER_RES_W as f32,
            y: ctx.window_size.y as f32 / RENDER_RES_H as f32
        }
    }

    
    pub fn get_window_scale_clamped() -> f32 {
        let pixel_size = Vector2 { x: RENDER_RES_W as f32, y: RENDER_RES_H as f32 };
        let window_size: Vector2 = get_context().window_size.into();
        let value = (window_size / pixel_size).x;
        clamp(2., 5., value)
    }
 

    

    lazy_static! {
        static ref RENDER_CONTEXT: Mutex<Renderer> = {
            Mutex::new(Renderer::default(BASE_RES_W as _, BASE_RES_H as _))
        };
    }

    lazy_static! {
        static ref TEXT_RENDER_CONTEXT: Mutex<TextRenderer> = {
            Mutex::new(TextRenderer::new(BASE_RES_W as _, BASE_RES_H as _))
        };
    }

    pub static mut CONTEXT: Option<Smol> = None;


    pub fn get_context() -> &'static mut Smol {
        unsafe { CONTEXT.as_mut().unwrap_or_else(|| panic!()) }
    }

    pub fn get_render_context() -> std::sync::MutexGuard<'static, render::Renderer> {
        RENDER_CONTEXT.lock().unwrap()
    }

    pub fn get_text_render_context() -> std::sync::MutexGuard<'static, TextRenderer> {
        TEXT_RENDER_CONTEXT.lock().unwrap()
    }

    pub fn get_text_bounds(text: &str, position: Vector2, font_size: f32) -> Option<Rect> {
        get_text_render_context().get_text_bounds(text, TextQueueConfig {
            position,
            font_size,
            ..Default::default()
        })
    }

    pub fn get_text_bounds_ex(text: &str, text_config: TextQueueConfig) -> Option<Rect> {
        get_text_render_context().get_text_bounds(text, text_config)
    }

    pub fn queue_text(text: &str, position: Vector2, font_size: f32, color: Color) -> Option<Rect> {
        queue_text_ex(text, TextQueueConfig {
            position,
            font_size,
            color,
            ..Default::default()
        })
    }

    pub fn queue_text_ex(text: &str, text_config: TextQueueConfig) -> Option<Rect> {
        get_text_render_context().queue_text_ex(text, text_config)
    }

    pub fn queue_multiple_text(text: Vec<(String, Color)>, position: Vector2, font_size: f32) -> Option<Rect> {
        get_text_render_context().queue_multiple(text, position, font_size)
    }

    pub fn render_text_queue() {
        get_text_render_context().render_queue();
    }

    pub fn clear(color: Color) {
        let ctx = get_context();
        if let Some(fps) = ctx.loop_helper.report_rate() {
            ctx.timer_state.fps = Some(fps);
        }

        Renderer::clear(color);
    }

    pub fn render_framebuffer_scale(texture: &Texture, position: Vector2, scale: Vector2) {
        get_render_context().framebuffer_texture_scale(texture, position, scale);
    }

    pub fn render_rect(x: f32, y: f32, width: f32, height: f32, color: Color) {
        get_render_context().rect(
           width, height, x, y, color
        );
    }

    pub fn load_texture(src: &str) -> Texture {
        Texture::load_from_file(src)
    }

    pub fn load_texture_from_bytes(bytes: &[u8]) -> Texture {
        Texture::load_from_bytes(bytes)
    }

    pub fn render_texture(texture: &Texture, position: Vector2) {
        get_render_context().texture(texture, position);
    }

    pub fn render_texture_scale(texture: &Texture, position: Vector2, scale: f32) {
        get_render_context().texture_scale(texture, position, scale);
    }

    
    pub fn render_texture_partial(texture: &PartialTexture, position: Vector2) {
        get_render_context().render_texture_partial(&texture, position);
    }

    pub fn start_scissor(x: i32, y: i32, width: i32, height: i32) {
        Renderer::start_scissor(x, y, width, height);
    }

    pub fn end_scissor() {
        Renderer::end_scissor();
    }

    // pub fn render_texture_to_rect(texture: &Texture, position: Vector2, ) {
    //     get_render_context().texture_rect_scale(&texture, )
    // }

    pub fn capture_framebuffer() {
        get_render_context().frame_buffer.bind();
    }

    pub fn stop_capture_framebuffer() {
        get_render_context().frame_buffer.unbind();
    }

    pub fn render_framebuffer(position: Vector2, scale: f32) {
        let render_context = get_render_context();
        let texture = &render_context.frame_buffer.texture;
        
        render_context.texture_scale(texture, position, scale);
    }


    pub fn get_window_size() -> Vector2Int {
        get_context().window_size
    }


    pub fn get_screen_center() -> Vector2Int {
        let ctx = get_context();
        let (x, y) = ctx.window.size();
        Vector2Int::new(x as i32 / 2, y as i32 / 2)
    }

    pub fn is_running() -> bool {
        let ctx = get_context();
        let delta = ctx.loop_helper.loop_start().as_secs_f32();
        ctx.timer_state.delta = delta;
        ctx.running
    }

    pub fn set_offset(offset: Vector2) {
        get_render_context().set_offset(offset);
        get_text_render_context().text_pipe.set_offset(offset);
    }
    
    pub fn reset_offset() {
        get_render_context().reset_offset();
        get_text_render_context().text_pipe.reset_offset();
    }

    pub fn end_render() {
        render_text_queue();
        let ctx = get_context();
        ctx.delta_time = ctx.time_step.delta() as f32;
        ctx.window.gl_swap_window();
        for event in ctx.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    ctx.running = false;
                },
                Event::Window {
                    win_event,
                    ..
                } => {
                    match win_event {
                        WindowEvent::Resized(w, h) => {
                            get_render_context().set_viewport(0.0, 0.0, w as u32, h as u32);
                            get_render_context().set_projection(w as f32, h as f32);
                            get_text_render_context().on_resize_window(Vector2Int { x: w, y: h });
                            ctx.window_size.x = w;
                            ctx.window_size.y = h;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        ctx.input.set_keys(&ctx.event_pump);
        ctx.input.set_mouse_state(&ctx.event_pump);
        ctx.loop_helper.loop_sleep();
    }

    pub fn delta_time() -> f32 {
        get_context().timer_state.delta
    }

    pub fn fps() -> f64 {
        get_context().timer_state.fps.unwrap_or(60.).round()
    }


    pub fn init() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let screen_width = BASE_RES_W as i32;  
        let screen_height = BASE_RES_H as i32; 
        let window = video_subsystem.window("Window", screen_width as _, screen_height as _)
            .opengl()
            .resizable()
            .build()
            .unwrap();
    
        // Unlike the other example above, nobody created a context for your window, so you need to create one.
        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        let _ = video_subsystem.gl_set_swap_interval(SwapInterval::VSync);
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        let event_pump = sdl_context.event_pump().unwrap();
       
        get_render_context().set_viewport(0.0, 0.0, screen_width as u32, screen_height as u32);
        get_render_context().set_projection(screen_width as f32, screen_height as f32);
        set_offset(Vector2::default());
        let loop_helper = LoopHelper::builder()
            .report_interval_s(0.5) // report every half a second
            .build_with_target_rate(60.0); // limit to 250 FPS if possible
        
        unsafe {
            CONTEXT = Option::from(
                Smol { 
                    running: true,
                    window,
                    event_pump,
                    _gl_context,
                    time_step: TimeStep::new(),
                    delta_time: 1. / 60.,
                    input: Input::new(),
                    window_size: Vector2Int {
                        x: screen_width as i32,
                        y: screen_height as i32
                    },
                    loop_helper,
                    timer_state: TimerState {
                        fps: None,
                        delta: 1. / 60.
                    },
                    camera: Camera {
                        zoom: 1.0
                    }
                }
            )
        };
    }

    pub struct Smol {
        pub running: bool,
        window: Window,
        event_pump: EventPump,
        _gl_context: GLContext,
        time_step: TimeStep,
        pub input: Input,
        delta_time: f32,
        pub window_size: Vector2Int,
        loop_helper: LoopHelper,
        timer_state: TimerState,
        camera: Camera
    }

    pub struct TimerState {
        delta: f32,
        fps: Option<f64>
    }
}


