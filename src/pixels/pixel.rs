use cgmath::Vector2;
use crate::colors::{Color, ColorCieLab};

pub struct Pixel {
    values: Vector2<f32>,
    color: Color,
}

impl Pixel {
    pub fn x(&self) -> f32 {self.values[0]}
    pub fn y(&self) -> f32 {self.values[1]}
    pub fn color(&self) -> &Color {&self.color}
    pub fn values(&self) -> &Vector2<f32> {&self.values}

    pub fn new (x: f32, y: f32, color: Color) -> Pixel {
        let values = Vector2::new(x, y);

        Pixel {
            values,
            color
        }
    }
}