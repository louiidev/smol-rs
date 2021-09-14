pub(crate) const MAX_BATCH_SIZE: i32 = 10000;

pub mod batch;
pub mod core;
pub mod shader;
pub mod shapes;
pub mod text;
pub(crate) mod texture;

pub use self::core::*;
pub(crate) use texture::*;
