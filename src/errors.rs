#[derive(Debug, Clone)]
pub struct SmolError {
    pub message: String,
}
impl SmolError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        SmolError {
            message: msg.into(),
        }
    }
}

impl std::convert::From<image::ImageError> for SmolError {
    fn from(e: image::ImageError) -> SmolError {
        SmolError {
            message: e.to_string(),
        }
    }
}

impl std::convert::From<std::io::Error> for SmolError {
    fn from(e: std::io::Error) -> SmolError {
        SmolError {
            message: e.to_string(),
        }
    }
}

impl std::convert::From<ron::de::Error> for SmolError {
    fn from(e: ron::Error) -> SmolError {
        SmolError {
            message: e.to_string(),
        }
    }
}

impl std::convert::From<std::ffi::NulError> for SmolError {
    fn from(e: std::ffi::NulError) -> SmolError {
        SmolError {
            message: e.to_string(),
        }
    }
}

impl std::convert::From<sdl2::IntegerOrSdlError> for SmolError {
    fn from(e: sdl2::IntegerOrSdlError) -> Self {
        SmolError {
            message: e.to_string(),
        }
    }
}

impl std::convert::From<glyph_brush::ab_glyph::InvalidFont> for SmolError {
    fn from(e: glyph_brush::ab_glyph::InvalidFont) -> Self {
        SmolError {
            message: e.to_string(),
        }
    }
}

impl std::convert::From<glyph_brush::BrushError> for SmolError {
    fn from(e: glyph_brush::BrushError) -> Self {
        SmolError {
            message: e.to_string(),
        }
    }
}
