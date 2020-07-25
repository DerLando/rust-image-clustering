use crate::image::Image;
use crate::pixels::{Pixel, PixelCieLab};
use crate::colors::{Color, ColorCieLab};
use cgmath::{MetricSpace, EuclideanSpace, Vector2, Point2, Point3};

pub struct PixelCluster {
    pixels: Vec<PixelCieLab>,
    center: PixelCieLab
}

impl PixelCluster {
    pub fn new(pixels: Vec<PixelCieLab>) -> PixelCluster {
        let mut cluster = PixelCluster {
            pixels,
            center: PixelCieLab::new(0.0, 0.0, ColorCieLab::new(0.0, 0.0, 0.0))
        };

        cluster.center = cluster.calculate_center();

        cluster
    }

    pub const fn new_empty(center: PixelCieLab) -> PixelCluster {
        PixelCluster {
            pixels: Vec::new(),
            center: center
        }
    }

    pub fn calculate_center(&self) -> PixelCieLab {
        
        let position_center = 
        EuclideanSpace::centroid(
            &self.pixels
            .iter()
            .map(|p| EuclideanSpace::from_vec(*p.values()))
            .collect::<Vec<Point2<f32>>>()
            [..]
            );
        
        let color_center =
        EuclideanSpace::centroid(
            &self.pixels
            .iter()
            .map(|p| EuclideanSpace::from_vec(*p.color().values()))
            .collect::<Vec<Point3<f32>>>()
            [..]
        );

        PixelCieLab::new(
            position_center.x, position_center.y,
            ColorCieLab::new(color_center.x, color_center.y, color_center.z))
    }

    pub fn update_center(&mut self) {
        self.center = self.calculate_center()
    }
}