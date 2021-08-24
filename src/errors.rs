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
