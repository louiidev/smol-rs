use glyph_brush::ab_glyph::Rect;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn add_padding(&mut self, expand_by: Vec2) {
        self.x -= expand_by.x;
        self.y -= expand_by.y;
        self.w += expand_by.x * 2.;
        self.h += expand_by.y * 2.;
    }
}

impl From<Rect> for Rectangle {
    fn from(item: Rect) -> Self {
        Rectangle {
            x: item.min.x,
            y: item.min.y,
            h: item.height(),
            w: item.width(),
        }
    }
}

pub fn clamp(min: f32, max: f32, value: f32) -> f32 {
    f32::min(f32::max(min, value), max)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::hash::Hash for Vec2 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.x as i32);
        state.finish();
    }
}

impl Eq for Vec2 {}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn length(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn dot(self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Returns a new `Vec2` with normalized components from the current vector.
    pub fn normalized(&self) -> Vec2 {
        *self / self.length()
    }

    pub fn distance(self, v: Vec2) -> f32 {
        let x = self.x - v.x;
        let y = self.y - v.y;

        f32::sqrt(x * x + y * y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, v: Vec2) -> Self {
        Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, value: f32) -> Self {
        Vec2 {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, v: Vec2) {
        *self = *self + v;
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, v: Vec2) -> Self {
        Vec2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, value: f32) -> Self {
        Vec2 {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, v: Vec2) {
        *self = *self - v;
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Self {
        Vec2 {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, value: f32) -> Self {
        Vec2 {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, v: Vec2) {
        *self = *self * v;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, v: Vec2) -> Self {
        Vec2 {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, value: f32) -> Self {
        Vec2 {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, v: Vec2) {
        *self = *self / v;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }
}

impl From<Vec3> for Vec4 {
    fn from(item: Vec3) -> Self {
        Vec4 {
            x: item.x,
            y: item.y,
            z: item.z,
            w: 0.,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec2Int {
    pub x: i32,
    pub y: i32,
}

impl Vec2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2Int { x, y }
    }

    pub const UP: Vec2Int = Vec2Int { x: 0, y: 1 };
    pub const RIGHT: Vec2Int = Vec2Int { x: 1, y: 0 };
    pub const LEFT: Vec2Int = Vec2Int { x: -1, y: 0 };
    pub const DOWN: Vec2Int = Vec2Int { x: 0, y: -1 };

    pub fn distance(self, v: Vec2Int) -> i32 {
        // Manhattan distance
        (self.x - v.x).abs() + (self.y - v.y).abs()
    }
}

impl From<Vec2> for Vec2Int {
    fn from(item: Vec2) -> Self {
        Vec2Int {
            x: item.x as _,
            y: item.y as _,
        }
    }
}

impl From<Vec2Int> for Vec2 {
    fn from(item: Vec2Int) -> Self {
        Vec2 {
            x: item.x as f32,
            y: item.y as f32,
        }
    }
}

impl From<Vec2> for Vec3 {
    fn from(item: Vec2) -> Self {
        Vec3 {
            x: item.x,
            y: item.y,
            z: 0.0,
        }
    }
}

impl AddAssign for Vec2Int {
    fn add_assign(&mut self, v: Vec2Int) {
        *self = *self + v;
    }
}

impl Add for Vec2Int {
    type Output = Vec2Int;
    fn add(self, v: Vec2Int) -> Self {
        Vec2Int {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<i32> for Vec2Int {
    type Output = Vec2Int;
    fn add(self, value: i32) -> Self {
        Vec2Int {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl Mul for Vec2Int {
    type Output = Vec2Int;
    fn mul(self, v: Vec2Int) -> Self {
        Vec2Int {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<i32> for Vec2Int {
    type Output = Vec2Int;
    fn mul(self, value: i32) -> Self {
        Vec2Int {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl MulAssign for Vec2Int {
    fn mul_assign(&mut self, v: Vec2Int) {
        *self = *self * v;
    }
}

impl MulAssign<i32> for Vec2Int {
    fn mul_assign(&mut self, value: i32) {
        *self = *self * value;
    }
}

impl Div for Vec2Int {
    type Output = Vec2Int;
    fn div(self, v: Vec2Int) -> Self {
        Vec2Int {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<i32> for Vec2Int {
    type Output = Vec2Int;
    fn div(self, value: i32) -> Self {
        Vec2Int {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl DivAssign for Vec2Int {
    fn div_assign(&mut self, v: Vec2Int) {
        *self = *self / v;
    }
}

impl DivAssign<i32> for Vec2Int {
    fn div_assign(&mut self, value: i32) {
        *self = *self / value;
    }
}

impl Sub for Vec2Int {
    type Output = Vec2Int;
    fn sub(self, v: Vec2Int) -> Self {
        Vec2Int {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<i32> for Vec2Int {
    type Output = Vec2Int;
    fn sub(self, value: i32) -> Self {
        Vec2Int {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl SubAssign for Vec2Int {
    fn sub_assign(&mut self, v: Vec2Int) {
        *self = *self - v;
    }
}

impl SubAssign<i32> for Vec2Int {
    fn sub_assign(&mut self, value: i32) {
        *self = *self - value;
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix
{
	m0: f32, m4: f32, m8: f32, m12: f32,
	m1: f32, m5: f32, m9: f32, m13: f32,
	m2: f32, m6: f32, m10: f32, m14: f32,
	m3: f32, m7: f32, m11: f32, m15: f32
}

#[rustfmt::skip]
impl From<Matrix> for [f32; 16] {
    fn from(item: Matrix) -> Self {
        [
            item.m0, item.m1, item.m2, item.m3,
            item.m4, item.m5, item.m6, item.m7,
            item.m8, item.m9, item.m10, item.m11,
            item.m12, item.m13, item.m14, item.m15
        ]
    }
}

#[rustfmt::skip]
impl Matrix {
    pub fn translate(base: Vec3) -> Self {
        Matrix {
            m0: 1.0, m4: 0.0, m8: 0.0, m12: base.x,
            m1: 0.0, m5: 1.0, m9: 0.0, m13: base.y,
            m2: 0.0, m6: 0.0, m10: 1.0, m14: base.z,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0
        }
    }

    pub fn scale(&mut self, scale: Vec2) {
        self.m0 *= scale.x;
		self.m5 *= scale.y;
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self
    {

        let rl = right - left;
        let tb = top - bottom;
        let f_n = far - near;

        let tx = -(right + left) / rl;
        let ty = -(top + bottom) / tb;
        let tz = -(far + near) / f_n;

        Matrix {
            m0: 2.0 / rl, m1: 0.0, m2: 0.0, m3: 0.0,
            m4: 0.0, m5: 2.0 / tb, m6: 0.0, m7: 0.0,
            m8: 0.0, m9: 0.0, m10: -2.0 / f_n, m11: 0.0,
            m12: tx, m13: ty,  m14: tz, m15: 1.0
        }
    }
}
