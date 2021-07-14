pub mod render;
pub mod math;
pub mod events;
pub mod components;
pub mod input;
pub mod texture_packer;
pub mod render_batch;
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
    use lazy_static::lazy_static;
    use crate::render::*;
    use crate::math::*;
    use sdl2::video::SwapInterval;
    use sdl2::video::Window;
    use sdl2::EventPump;
    use sdl2::event::Event;
    use sdl2::video::GLContext;
    use sdl2::video::GLProfile;
    use std::sync::Mutex;
    use sdl2::event::WindowEvent;

    pub type Keycode = sdl2::keyboard::Keycode;

    lazy_static! {
        static ref RENDER_CONTEXT: Mutex<Renderer> = {
            Mutex::new(Renderer::default())
        };
    }

    pub static mut CONTEXT: Option<Smol> = None;


    pub fn get_context() -> &'static mut Smol {
        unsafe { CONTEXT.as_mut().unwrap_or_else(|| panic!()) }
    }

    fn get_render_context() -> std::sync::MutexGuard<'static, render::Renderer> {
        RENDER_CONTEXT.lock().unwrap()
    }


    pub fn clear(color: Color) {
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

    // pub fn render_texture_to_rect(texture: &Texture, position: Vector2, ) {
    //     get_render_context().texture_rect_scale(&texture, )
    // }

    pub fn get_screen_center() -> Vector2Int {
        let ctx = get_context();
        let (x, y) = ctx.window.size();
        Vector2Int::new(x as i32 / 2, y as i32 / 2)
    }


    pub fn is_key_down(key: Keycode) -> bool {
        let ctx = get_context();
        ctx.input.is_key_down(key)
    }

    pub fn is_key_released(key: Keycode) -> bool {
        let ctx = get_context();
        ctx.input.is_key_released(key)
    }

    pub fn is_key_pressed(key: Keycode) -> bool {
        let ctx = get_context();
        ctx.input.is_key_pressed(key)
    }


    pub fn is_running() -> bool {
        let ctx = get_context();
        ctx.running
    }

    pub fn end_render() {
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
                            Renderer::set_viewport(0.0, 0.0, w as u32, h as u32);
                            get_render_context().set_projection(w as f32, h as f32);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        ctx.input.set_keys(&ctx.event_pump);
    }

    pub fn delta_time() -> f32 {
        get_context().delta_time
    }


    pub fn init() {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
       
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(4, 1);
        let virtual_width= 640;
        let virtual_height= 480;

        let screen_width = virtual_width * 2;  
        let screen_height = virtual_height * 2; 
        let window = video_subsystem.window("Window", screen_width, screen_height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
    
        // Unlike the other example above, nobody created a context for your window, so you need to create one.
        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        let _ = video_subsystem.gl_set_swap_interval(SwapInterval::VSync);
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (4, 1));
        let event_pump = sdl_context.event_pump().unwrap();
       
        Renderer::set_viewport(0.0, 0.0, screen_width as u32, screen_height as u32);
        get_render_context().set_projection(screen_width as f32, screen_height as f32);
        
        unsafe {
            CONTEXT = Option::from(
                Smol {
                    running: true,
                    window,
                    event_pump,
                    _gl_context,
                    time_step: TimeStep::new(),
                    delta_time: 0.0,
                    input: Input::new()
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
    }
}


