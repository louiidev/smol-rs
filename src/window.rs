use crate::{errors::SmolError, App};

impl App {
    pub fn set_title(&mut self, title: &str) -> Result<(), SmolError> {
        self.window.set_title(title)?;

        Ok(())
    }
    pub fn set_size(&mut self, width: u32, height: u32) -> Result<(), SmolError> {
        self.window.set_size(width, height)?;

        Ok(())
    }
}
