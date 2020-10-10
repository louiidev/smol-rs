pub mod render;
pub mod math;
use std::time::Instant;
use std::collections::HashSet;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::rect::Point;
use sdl2::EventPump;

pub struct Input {
    pub keys_pressed: HashSet<Keycode>,
    pub keys_down: HashSet<Keycode>,
    keys_released: HashSet<Keycode>,
    mouse_state: MouseState,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys_pressed: HashSet::new(),
            keys_down: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_state: MouseState::from_sdl_state(0),
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: Keycode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn set_mouse_state(&mut self, events: &EventPump) {
        let state = events.mouse_state();
        self.mouse_state = state;
    }

    pub fn set_keys(&mut self, events: &EventPump) {
        let keys = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        let new_keys = &keys - &self.keys_down;
        let old_keys = &self.keys_down - &keys;
        self.keys_down = keys;
        self.keys_pressed = new_keys;
        self.keys_released = old_keys;

  
    }

    pub fn get_mouse_pos(&mut self) -> Point {
        Point::new(self.mouse_state.x(), self.mouse_state.y())
    }
}

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

    static mut CONTEXT: Option<Smol> = None;


    fn get_context() -> &'static mut Smol {
        unsafe { CONTEXT.as_mut().unwrap_or_else(|| panic!()) }
    }

    fn get_render_context() -> std::sync::MutexGuard<'static, render::Renderer> {
        RENDER_CONTEXT.lock().unwrap()
    }


    pub fn clear() {
        Renderer::clear(Color (0.3 * 255., 0.3 * 255., 0.5 * 255., 255.));
    }

    pub fn draw_rectangle() {
        get_render_context().rect(
            Rectangle { x: 0., y: 0., width: 100., height: 100.},
            Color(255., 0.0, 0.0, 255.0)
        );
    }

    pub fn load_texture(src: &str) -> Texture {
        Texture::load_from_file(src)
    }

    pub fn draw_sprite(texture: &Texture, position: Vector2) {
        get_render_context().texture(texture, position);
    }

    pub fn draw_sprite_from_atlas(atlas: &Texture, position: Vector2, coords: Vector2Int, size: u32) {
        get_render_context().atlas_sub_s(&atlas, coords.x, coords.y, size, position, 1.0)
    }


    pub fn get_keys_pressed() {

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
        ctx.delta_time = ctx.time_step.delta() as f32;;
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
                            println!("w: {} h: {}", w, h);
                            Renderer::set_viewport(0.0, 0.0, w as u32, h as u32);
                            get_render_context().set_projection(w as u32, h as u32);
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
        let virtual_width= 320;
        let virtual_height= 180;

        let screen_width = virtual_width * 1;  
        let screen_height = virtual_height * 1; 
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
       



        
        
        // This is your target virtual resolution for the game, the size you built your game to
        let virtual_width= 320;
        let virtual_height= 180;
        
        let target_aspect_ratio = (virtual_width/virtual_height) as f32;
        
        // figure out the largest area that fits in this resolution at the desired aspect ratio
        let mut width = screen_width as f32;
        let mut height = (width / target_aspect_ratio as f32)  + 0.5;
        
        if height > screen_height as f32
        {
        //It doesn't fit our height, we must switch to pillarbox then
            height = screen_height as f32;
            width = (height * target_aspect_ratio)  + 0.5;
        }
        
        // set up the new viewport centered in the backbuffer
        let vp_x = (screen_width as f32  / 2.) - (width / 2.);
        let vp_y = (screen_height as f32 / 2.) - (height / 2.);
        
        Renderer::set_viewport(0.0, 0.0, screen_width as u32, screen_height as u32);
        get_render_context().set_projection(screen_width, screen_height);
        // get_render_context().set_scale(Vector2 {
        //     x: (screen_width / virtual_width) as f32,
        //     y: (screen_height / virtual_height) as f32
        // });

        


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

    struct Smol {
        pub running: bool,
        window: Window,
        event_pump: EventPump,
        _gl_context: GLContext,
        time_step: TimeStep,
        input: Input,
        delta_time: f32,
    }
}


