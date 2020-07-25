use crate::colors::ColorCieLab;
use cgmath::{Vector2, MetricSpace};

pub struct PixelCieLab {
    values: Vector2<f32>,
    color: ColorCieLab
}

impl PixelCieLab {
    pub fn x(&self) -> f32 {self.values[0]}
    pub fn y(&self) -> f32 {self.values[1]}
    pub fn color(&self) -> &ColorCieLab {&self.color}
    pub fn values(&self) -> &Vector2<f32> {&self.values}

    pub const fn new(x: f32, y: f32, color: ColorCieLab) -> PixelCieLab {
        let values = Vector2::new(x, y);

        PixelCieLab {
            values,
            color
        }
    }

    pub fn distance(a: &PixelCieLab, b: &PixelCieLab, m: u8, s: f32) -> f32 {
        let cie_dist = a.color().values().distance2(*b.color().values());
        let pixel_dist = a.values().distance2(*b.values());

        cie_dist + (m as f32 / s) * pixel_dist
    }
}