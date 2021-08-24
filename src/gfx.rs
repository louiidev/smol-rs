mod opengl;
mod vulkan;

#[cfg(feature = "opengl")]
pub use crate::gfx::opengl::GfxContext;

#[cfg(feature = "vulkan")]
pub use crate::gfx::vulkan::GfxContext;
