#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use sdl2::video::Window;

use crate::glm::*;
use crate::AppSettings;

pub struct GfxContext {}

impl GfxContext {
    pub fn new() -> Self {
        GfxContext {}
    }
}

pub fn build_window(sdl_context: &sdl2::Sdl, settings: &AppSettings) -> Window {
    todo!()
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("Window", 800, 600)
    //     .vulkan()
    //     .build()
    //     .unwrap();

    // let instance_extensions = window.vulkan_instance_extensions().unwrap();
    // let raw_instance_extensions = RawInstanceExtensions::new(
    //     instance_extensions
    //         .iter()
    //         .map(|&v| CString::new(v).unwrap()),
    // );
    // let instance = Instance::new(None, raw_instance_extensions, None).unwrap();
    // let surface_handle = window
    //     .vulkan_create_surface(instance.internal_object())
    //     .unwrap();
    // let surface = unsafe { Surface::from_raw_surface(instance, surface_handle, window.context()) };

    // window
}
