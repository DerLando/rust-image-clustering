use cgmath::Vector3;


#[derive(Debug, PartialEq)]
pub struct ColorRgb {
    values: Vector3<u8>
}

impl ColorRgb {
    pub fn r(&self) -> u8 {self.values[0]}
    pub fn g(&self) -> u8 {self.values[1]}
    pub fn b(&self) -> u8 {self.values[2]}

    pub const fn new(r: u8, g: u8, b: u8) -> ColorRgb {
        ColorRgb {
            values: Vector3::new(r, g, b)
        }
    }
}

