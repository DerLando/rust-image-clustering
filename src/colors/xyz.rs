use super::{ColorRgb};

fn normalize_rgb(rgb: u8) -> f32 {
    rgb as f32 / 255.0
}

fn convert_to_pre_xyz(value: f32) -> f32 {
    const TRESHHOLD: f32 = 0.04045;

    let result: f32;

    match value > TRESHHOLD {
        true => result = ((value + 0.055) / 1.055).powf(2.4),
        false => result = value / 12.92
    };

    result * 100.0
}

fn convert_to_pre_rgb(value: f32) -> f32 {
    const TRESHHOLD: f32 = 0.0031308;

    let result: f32;

    match value > TRESHHOLD {
        true => result = 1.055 * value.powf(1.0 / 2.4) - 0.055,
        false => result = 12.92 * value
    }

    (result * 255.0).round()
}

#[derive(Debug, PartialEq)]
pub struct ColorXyz{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ColorXyz {
    pub fn new_from_rgb(rgb: &ColorRgb) -> ColorXyz {
        let r = convert_to_pre_xyz(normalize_rgb(rgb.r()));
        let g = convert_to_pre_xyz(normalize_rgb(rgb.g()));
        let b = convert_to_pre_xyz(normalize_rgb(rgb.b()));
        
        ColorXyz{
            x: r * 0.4124 + g * 0.3576 + b * 0.1805,
            y: r * 0.2126 + g * 0.7152 + b * 0.0722,
            z: r * 0.0193 + g * 0.1192 + b * 0.9505
        }
    }

    pub fn as_rgb(&self) -> ColorRgb {

        let x = self.x / 100.0;
        let y = self.y / 100.0;
        let z = self.z / 100.0;

        let r = x * 3.2406 + y * -1.5372 + z * -0.4986;
        let g = x * -0.9689 + y * 1.8758 + z * 0.0415;
        let b = x * 0.0557 + y * -0.2040 + z * 1.0570;

        ColorRgb::new(
            convert_to_pre_rgb(r) as u8,
            convert_to_pre_rgb(g) as u8,
            convert_to_pre_rgb(b) as u8
        )
    }

}

#[cfg(test)]
mod test {

    use crate::colors::{ColorRgb, ColorXyz};
    use rand::Rng;

    #[test]
    fn converting_from_rgb_to_xyz_and_back_should_be_identity() {

        let mut rng = rand::thread_rng();

        for i in 0..100 {
            let r = rng.gen_range(0, 256) as u8;
            let g = rng.gen_range(0, 256) as u8;
            let b = rng.gen_range(0, 256) as u8;

            // Arrange
            let rgb = ColorRgb::new(r, g, b);
            let xyz = ColorXyz::new_from_rgb(&rgb);

            // Act
            let converted = xyz.as_rgb();

            // Assert
            assert_eq!(rgb, converted);
        }
    }
}