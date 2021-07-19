use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

use glyph_brush::ab_glyph::Rect;


#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Rectangle {
    pub fn add_padding(&mut self, expand_by: Vector2) {
        self.x-= expand_by.x;
        self.y-= expand_by.y;
        self.w+= expand_by.x * 2.;
        self.h+= expand_by.y * 2.;
    }
}


impl From<Rect> for Rectangle {
    fn from(item: Rect) -> Self {
        Rectangle {
            x: item.min.x,
            y: item.min.y,
            h: item.height(),
            w: item.width()
        }
    }
}


pub fn clamp(min: f32, max: f32, value: f32) -> f32 {
    f32::min(f32::max(min, value), max)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl std::hash::Hash for Vector2 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.x as i32);
        state.finish();
    }
}

impl Eq for Vector2 {}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x,
            y
        }
    }

    pub fn default() -> Self {
        Vector2 {
            x: 0.0,
            y: 0.0
        }
    }
    pub fn length(self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn dot(self, v: Vector2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Returns a new `Vector2` with normalized components from the current vector.
    pub fn normalized(&self) -> Vector2 {
        *self / self.length()
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, value: f32) -> Self {
        Vector2 {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, v: Vector2) {
        *self = *self + v;
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, value: f32) -> Self {
        Vector2 {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, v: Vector2) {
        *self = *self - v;
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, value: f32) -> Self {
        Vector2 {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, v: Vector2) {
        *self = *self * v;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, value: f32) -> Self {
        Vector2 {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, v: Vector2) {
        *self = *self / v;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}



#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn default() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 {
            x,
            y,
            z,
            w,
        }
    }

    pub fn default() -> Self {
        Vector4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
    }
}

impl From<Vector3> for Vector4 {
    fn from(item: Vector3) -> Self {
        Vector4 {
            x: item.x,
            y: item.y,
            z: item.z,
            w: 0.
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32
}

impl Vector2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2Int {
            x,
            y
        }
    }

    pub fn default() -> Self {
        Vector2Int {
            x: 0,
            y: 0
        }
    }
}

impl From<Vector2Int> for Vector2 {
    fn from(item: Vector2Int) -> Self {
        Vector2 {
            x: item.x as f32,
            y: item.y as f32,
        }
    }
}

impl From<Vector2> for Vector3 {
    fn from(item: Vector2) -> Self {
        Vector3 {
            x: item.x,
            y: item.y,
            z: 0.0
        }
    }
}

impl AddAssign for Vector2Int {
    fn add_assign(&mut self, v: Vector2Int) {
        *self = *self + v;
    }
}



impl Add for Vector2Int {
    type Output = Vector2Int;
    fn add(self, v: Vector2Int) -> Self {
        Vector2Int {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<i32> for Vector2Int {
    type Output = Vector2Int;
    fn add(self, value: i32) -> Self {
        Vector2Int {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl Mul for Vector2Int {
    type Output = Vector2Int;
    fn mul(self, v: Vector2Int) -> Self {
        Vector2Int {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<i32> for Vector2Int {
    type Output = Vector2Int;
    fn mul(self, value: i32) -> Self {
        Vector2Int {
            x: self.x * value,
            y: self.y * value,
        }
    }
}


impl MulAssign for Vector2Int {
    fn mul_assign(&mut self, v: Vector2Int) {
        *self = *self * v;
    }
}

impl MulAssign<i32> for Vector2Int {
    fn mul_assign(&mut self, value: i32) {
        *self = *self * value;
    }
}


impl Div for Vector2Int {
    type Output = Vector2Int;
    fn div(self, v: Vector2Int) -> Self {
        Vector2Int {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<i32> for Vector2Int {
    type Output = Vector2Int;
    fn div(self, value: i32) -> Self {
        Vector2Int {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl DivAssign for Vector2Int {
    fn div_assign(&mut self, v: Vector2Int) {
        *self = *self / v;
    }
}

impl DivAssign<i32> for Vector2Int {
    fn div_assign(&mut self, value: i32) {
        *self = *self / value;
    }
}


impl Sub for Vector2Int {
    type Output = Vector2Int;
    fn sub(self, v: Vector2Int) -> Self {
        Vector2Int {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<i32> for Vector2Int {
    type Output = Vector2Int;
    fn sub(self, value: i32) -> Self {
        Vector2Int {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl SubAssign for Vector2Int {
    fn sub_assign(&mut self, v: Vector2Int) {
        *self = *self - v;
    }
}

impl SubAssign<i32> for Vector2Int {
    fn sub_assign(&mut self, value: i32) {
        *self = *self - value;
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Matrix
{
	m0: f32, m4: f32, m8: f32, m12: f32,
	m1: f32, m5: f32, m9: f32, m13: f32,
	m2: f32, m6: f32, m10: f32, m14: f32,
	m3: f32, m7: f32, m11: f32, m15: f32
}


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


impl Matrix {
    pub fn translate(base: Vector3) -> Self {
        Matrix {
            m0: 1.0, m4: 0.0, m8: 0.0, m12: base.x,
            m1: 0.0, m5: 1.0, m9: 0.0, m13: base.y,
            m2: 0.0, m6: 0.0, m10: 1.0, m14: base.z,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0
        }
    }

    pub fn scale(&mut self, scale: Vector2) {
        self.m0 *= scale.x;
		self.m5 *= scale.y;
    }

    
    #[rustfmt::skip]
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
 