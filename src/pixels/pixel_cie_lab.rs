use crate::colors::ColorCieLab;
use cgmath::{MetricSpace, Point2};

pub struct PixelCieLab {
    values: Point2<u32>,
    color: ColorCieLab,
}

impl PixelCieLab {
    pub fn x(&self) -> u32 {
        self.values.x
    }
    pub fn y(&self) -> u32 {
        self.values.y
    }
    pub fn color(&self) -> &ColorCieLab {
        &self.color
    }
    pub fn values(&self) -> &Point2<u32> {
        &self.values
    }
    pub fn values_as_float(&self) -> Point2<f32> {
        Point2::new(self.x() as f32, self.y() as f32)
    }

    pub const fn new(x: u32, y: u32, color: ColorCieLab) -> PixelCieLab {
        let values = Point2::new(x, y);

        PixelCieLab { values, color }
    }

    pub fn distance(a: &PixelCieLab, b: &PixelCieLab, m: u8, s: f32) -> f32 {
        let cie_dist = a.color().values().distance2(*b.color().values());
        let pixel_dist = a.values_as_float().distance2(b.values_as_float());

        cie_dist + (m as f32 / s) * pixel_dist
    }
}

#[cfg(test)]
mod test {
    use crate::colors::ColorCieLab;
    use crate::pixels::PixelCieLab;

    #[test]
    fn distance_between_equals_should_be_zero() {
        // Arrange
        let cie = ColorCieLab::new(100.0, 50.0, 40.0);
        let a = PixelCieLab::new(1, 5, cie.clone());
        let b = PixelCieLab::new(1, 5, cie.clone());

        // Act
        let dist = PixelCieLab::distance(&a, &b, 10, 30.0);

        // Assert
        assert_eq!(dist, 0.0);
    }
}
