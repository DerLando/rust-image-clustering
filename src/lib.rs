mod colors;
mod pixels;
mod k_means_solver;

use image::{DynamicImage, GenericImage, GenericImageView, Rgb, RgbImage};
use rand::Rng;
use crate::colors::{ColorRgb, ColorCieLab, ColorXyz};
use crate::pixels::PixelCieLab;
use crate::k_means_solver::KMeansSuperPixelSolver;

fn pixel_from_rgb(x: u32, y: u32, rgb: &Rgb<u8>) -> PixelCieLab {
    let c_rgb = ColorRgb::new(rgb.0[0], rgb.0[1], rgb.0[2]);
    let xyz = ColorXyz::new_from_rgb(&c_rgb);
    let cie = ColorCieLab::new_from_xyz(&xyz);

    PixelCieLab::new(x, y, cie)
}

fn test_generate_pixels(image: DynamicImage) -> RgbImage {
    // convert to nice pixels
    let rgb = image.as_rgb8().unwrap();
    let pixels: Vec<PixelCieLab> = rgb.enumerate_pixels().into_iter()
    .map(|(x, y, rgb)| pixel_from_rgb(x, y, rgb))
    .collect();

    let superpixel_count = 400;

    let mut solver = 
        KMeansSuperPixelSolver::new(pixels, 10, superpixel_count,
            image.width() as usize, image.height() as usize);

    for _ in 0..10 {
        solver.solve_tick();
    }

    let superpixels = solver.current_superpixels();

    let mut rng = rand::thread_rng();
    let mut img = RgbImage::new(image.width(), image.height());
    
    for i in 0..superpixel_count {
        let r = rng.gen_range(0, 256) as u8;
        let g = rng.gen_range(0, 256) as u8;
        let b = rng.gen_range(0, 256) as u8;
        let rgb = Rgb([r, g, b]);

        for pixel in &superpixels[i] {
            img.put_pixel(pixel.0, pixel.1, rgb)
        }
    }

    img
}

fn clustered_file_path(path: &String) -> String {
    let start = path.split(&String::from(".")).next().unwrap();
    let mut result = String::from(start);
    result.push_str("_clustered.jpg");

    result
}

pub fn generate_super_pixels(path: String) {
    match image::open(path.clone()) {
        Ok(img) => {
            let result = test_generate_pixels(img);
            result.save(clustered_file_path(&path));
        },
        Err(e) => println!("Error: {:?}", e)
    };


}