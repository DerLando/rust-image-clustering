pub use self::rgb::ColorRgb;
mod rgb;

pub use self::xyz::ColorXyz;
mod xyz;

pub use self::cie_lab::ColorCieLab;
mod cie_lab;

pub enum Color {
    Rgb(ColorRgb),
    Xyz(ColorXyz),
    CieLab(ColorCieLab),
}
