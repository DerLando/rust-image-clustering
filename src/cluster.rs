use crate::image::Image;
use crate::pixels::{Pixel, PixelCieLab};
use crate::colors::{Color, ColorCieLab};
use cgmath::{MetricSpace, EuclideanSpace, Vector2, Point2, Point3};

fn cielab_distance(a: &ColorCieLab, b: &ColorCieLab) -> f32 {
    a.values().distance2(*b.values())
}

fn pixel_distance(a: Pixel, b: Pixel, m: u8, s: f32) -> f32 {
    let mut result: f32 = 0.0;

    match (a.color(), b.color()) {
        (Color::CieLab(a_cielab), Color::CieLab(b_cielab)) => {
            let cie_dist = cielab_distance(a_cielab, b_cielab);
            let pixel_dist = a.values().distance2(*b.values());

            result = cie_dist + (m as f32 / s) * pixel_dist;
        }
        _ => {}
    }

    result
}

pub struct PixelCluster {
    pixels: Vec<PixelCieLab>,
    center: PixelCieLab
}

impl PixelCluster {
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
}