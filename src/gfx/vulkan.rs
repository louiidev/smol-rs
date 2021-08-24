use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use crate::math::*;

pub struct GfxContext {}

impl GfxContext {
    pub fn new() -> Self {
        GfxContext {}
    }
}
