use super::{ColorXyz};

// d65 CIE 1931 reference values for rgb color space conversion
const REFERENCE_X: f32 = 95.047;
const REFERENCE_Y: f32 = 100.0;
const REFERENCE_Z: f32 = 108.883;

fn convert_to_pre_lab(value: f32) -> f32 {
    const TRESHHOLD: f32 = 0.008856;

    match value > TRESHHOLD {
        true => return value.powf(1.0 / 3.0),
        false => return 7.787 * value + 16.0 / 116.0
    }
}

fn convert_to_pre_xyz(value: f32) -> f32 {
    const TRESHHOLD: f32 = 0.008856;
    
    let pow = value.powi(3);

    match pow > TRESHHOLD {
        true => return pow,
        false => return (value - 16.0 / 116.0) / 7.787
    }
}

#[derive(Debug)]
pub struct ColorCieLab {
    pub L: f32,
    pub a: f32,
    pub b: f32,
}

impl ColorCieLab {
    pub fn new_from_xyz(xyz: &ColorXyz) -> ColorCieLab {
        let x = convert_to_pre_lab(xyz.x / REFERENCE_X);
        let y = convert_to_pre_lab(xyz.y / REFERENCE_Y);
        let z = convert_to_pre_lab(xyz.z / REFERENCE_Z);

        ColorCieLab {
            L: 116.0 * y - 16.0,
            a: 500.0 * (x - y),
            b: 200.0 * (y - z)
        }
    }

    pub fn as_xyz(&self) -> ColorXyz {
        let y = (self.L as f32 + 16.0) / 116.0;
        let x = self.a as f32 / 500.0 + y;
        let z = y - self.b as f32 / 200.0;

        ColorXyz{
            x: convert_to_pre_xyz(x) * REFERENCE_X,
            y: convert_to_pre_xyz(y) * REFERENCE_Y,
            z: convert_to_pre_xyz(z) * REFERENCE_Z
        }
    }
}

#[cfg(test)]
mod test {

    use crate::colors::{ColorRgb, ColorXyz, ColorCieLab};
    use rand::Rng;

    #[test]
    fn convert_from_xyz_to_cielab_and_back_should_be_identity() {
        let mut rng = rand::thread_rng();

        for i in 0..100 {
            let r = rng.gen_range(0, 256) as u8;
            let g = rng.gen_range(0, 256) as u8;
            let b = rng.gen_range(0, 256) as u8;

            // Arrange
            let rgb = ColorRgb{r, g, b};
            let xyz = ColorXyz::new_from_rgb(&rgb);
            let cie = ColorCieLab::new_from_xyz(&xyz);

            // Act
            let converted = cie.as_xyz().as_rgb();

            // Assert
            assert_eq!(rgb, converted);
        }        
    }
}