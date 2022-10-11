#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector {
        Vector { x, y, z, w }
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from(other: (f32, f32, f32)) -> Self {
        Vector::new(other.0, other.1, other.2, 0.0)
    }
}

impl From<(f32, f32, f32, f32)> for Vector {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Vector::new(other.0, other.1, other.2, other.3)
    }
}