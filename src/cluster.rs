use crate::pixels::{Pixel, PixelCieLab};
use crate::colors::{Color, ColorCieLab};
use cgmath::{MetricSpace, EuclideanSpace, Vector2, Point2, Point3};

pub struct PixelCluster<'a> {
    pixels: Vec<&'a PixelCieLab>,
    center_index: usize
}

impl PixelCluster<'_> {
    pub const fn new(pixels: Vec<&PixelCieLab>) -> PixelCluster {
        PixelCluster {
            pixels,
            center_index: 0
        }
    }

    // pub const fn new_empty(center: PixelCieLab) -> PixelCluster {
    //     PixelCluster {
    //         pixels: Vec::new(),
    //         center_index: 0
    //     }
    // }

    pub fn calculate_center(&self) -> Point2<f32> {
        
        EuclideanSpace::centroid(
            &self.pixels
            .iter()
            .map(|p| EuclideanSpace::from_vec(*p.values()))
            .collect::<Vec<Point2<f32>>>()
            [..]
            )
    }

    // pub fn update_center(&mut self) {
    //     self.center = self.calculate_center()
    // }
}