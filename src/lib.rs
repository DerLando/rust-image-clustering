mod colors;
mod pixels;
mod k_means_solver;

use image::{DynamicImage, GenericImage, GenericImageView, Rgb};
use crate::colors::{ColorRgb, ColorCieLab, ColorXyz};
use crate::pixels::PixelCieLab;
use crate::k_means_solver::KMeansSuperPixelSolver;

fn pixel_from_rgb(x: u32, y: u32, rgb: &Rgb<u8>) -> PixelCieLab {
    let c_rgb = ColorRgb::new(rgb.0[0], rgb.0[1], rgb.0[2]);
    let xyz = ColorXyz::new_from_rgb(&c_rgb);
    let cie = ColorCieLab::new_from_xyz(&xyz);

    PixelCieLab::new(x, y, cie)
}

fn test_generate_pixels(image: DynamicImage) {
    // convert to nice pixels
    let rgb = image.as_rgb8().unwrap();
    let pixels: Vec<PixelCieLab> = rgb.enumerate_pixels().into_iter()
    .map(|(x, y, rgb)| pixel_from_rgb(x, y, rgb))
    .collect();

    let solver = KMeansSuperPixelSolver::new(pixels, 10, 50, image.width() as usize, image.height() as usize);
}

pub fn generate_super_pixels(path: String) {
    match image::open(path) {
        Ok(img) => {
            test_generate_pixels(img);
        },
        Err(e) => println!("Error: {:?}", e)
    };


}