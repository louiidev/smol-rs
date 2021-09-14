#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub f32);

impl Color {
    pub fn normalize(&self) -> [f32; 4] {
        [
            self.0 as f32 / 255.,
            self.1 as f32 / 255.,
            self.2 as f32 / 255.,
            self.3,
        ]
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b, 1.)
    }

    pub const WHITE: Color = Color(255, 255, 255, 1.);
    pub const BLUE: Color = Color(10, 10, 255, 1.);
    pub const RED: Color = Color(255, 10, 10, 1.);
    pub const GREEN: Color = Color(10, 255, 10, 1.);
    pub const BLACK: Color = Color(1, 1, 1, 1.);
}
