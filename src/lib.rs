pub mod render;

pub mod prelude {
    use lazy_static::lazy_static;
    use crate::render::{Color, Rectangle, Renderer, Texture, Vector2};
    use sdl2::video::GLProfile;
    use sdl2::video::Window;
    use sdl2::EventPump;
    use sdl2::keyboard::Keycode;
    use sdl2::event::Event;
    use sdl2::video::GLContext;
    use cgmath::prelude::*;

    lazy_static! {
        static ref RENDER_CONTEXT: Renderer = {
            let renderer = Renderer::default();
            Renderer::set_viewport(800, 600);
            renderer
        };
    }

    static mut CONTEXT: Option<Smol> = None;


    fn get_context() -> &'static mut Smol {
        unsafe { CONTEXT.as_mut().unwrap_or_else(|| panic!()) }
    }


    pub fn clear() {
        Renderer::clear(Color (0.3 * 255., 0.3 * 255., 0.5 * 255., 255.));
    }

    pub fn draw_rectangle() {
        RENDER_CONTEXT.rect(
            Rectangle { x: 0., y: 0., width: 100., height: 100.},
            Color(255., 0.0, 0.0, 255.0)
        );
    }

    pub fn load_texture(src: &str) -> Texture {
        Texture::load_from_file(src)
    }

    pub fn draw_sprite(texture: &Texture) {
        RENDER_CONTEXT.texture(texture, Vector2 {
            x: 100., y: 100.
        })
    }


    pub fn is_running() -> bool {
        let ctx = get_context();
        ctx.running
    }

    pub fn end_render() {
        let ctx = get_context();
        ctx.window.gl_swap_window();
        for event in ctx.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    ctx.running = false;
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    


    pub struct Smol {
        pub running: bool,
        window: Window,
        event_pump: EventPump,
        gl_context: GLContext
    }

    
    impl Smol {
        pub fn init() {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
            
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(GLProfile::Core);
            gl_attr.set_context_version(4, 1);
        
            let window = video_subsystem.window("Window", 800, 600)
                .opengl()
                .build()
                .unwrap();
        
            // Unlike the other example above, nobody created a context for your window, so you need to create one.
            let gl_context = window.gl_create_context().unwrap();
            gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
            
            debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
            debug_assert_eq!(gl_attr.context_version(), (4, 1));
            let event_pump = sdl_context.event_pump().unwrap();
            Renderer::set_viewport(800, 600);
            unsafe {
                CONTEXT = Option::from(
                    Smol {
                        running: true,
                        window,
                        event_pump,
                        gl_context
                    }
                )
            };
        
        }

        
    }
}


