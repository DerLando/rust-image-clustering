use cgmath::Vector3;
use image::Rgb;

#[derive(Debug, PartialEq)]
pub struct ColorRgb {
    values: Vector3<u8>,
}

impl ColorRgb {
    pub fn r(&self) -> u8 {
        self.values[0]
    }
    pub fn g(&self) -> u8 {
        self.values[1]
    }
    pub fn b(&self) -> u8 {
        self.values[2]
    }
    pub fn values(&self) -> (u8, u8, u8) {
        (self.values.x, self.values.y, self.values.z)
    }

    pub const fn new(r: u8, g: u8, b: u8) -> ColorRgb {
        ColorRgb {
            values: Vector3::new(r, g, b),
        }
    }

    pub fn as_image_rgb(&self) -> Rgb<u8> {
        Rgb::from([self.r(), self.g(), self.b()])
    }
}
