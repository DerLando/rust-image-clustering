use cgmath::Vector2;
use crate::colors::Color;

pub struct Pixel {
    values: Vector2<u16>,
    color: Color,
}

impl Pixel {
    pub fn x(&self) -> u16 {self.values[0]}
    pub fn y(&self) -> u16 {self.values[1]}

    pub fn new (x: u16, y: u16, color: Color) -> Pixel {
        let values = Vector2::new(x, y);

        Pixel {
            values,
            color
        }
    }
}