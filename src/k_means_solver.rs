use crate::pixels::{PixelCieLab, Grid};
use crate::cluster::PixelCluster;
use num::integer::Roots;

pub struct KMeansSuperPixelSolver {
    flat_pixels: Vec<PixelCieLab>,
    clusters: Vec<PixelCluster>,

    /// image height
    height: usize,
    /// image width
    width: usize,
    /// compactness value of super pixels, between 1 and 20, 10 is a good value
    m: u8,
    /// size (in pixels) of a super pixel
    s: u16,
    /// number of all pixels
    n: usize,
    /// number of clusters (in this case superpixels)
    k: usize
}

impl KMeansSuperPixelSolver {
    pub fn new(pixels: Vec<PixelCieLab>, compactness: u8, superpixel_count: usize,
        image_width: usize, image_height: usize) -> KMeansSuperPixelSolver {
        
        let pixel_count = image_width * image_height;
        let pixel_size = Self::calculate_superpixel_size(pixel_count, superpixel_count);

        let initial_centroids = Self::calculate_initial_centroids(&pixels, superpixel_count);
        let mut initial_clusters = 
        initial_centroids
            .into_iter()
            .map(move |c| PixelCluster::new_empty( c))
            .collect()
            ;

        Self::assign_pixels_to_superpixels(&mut initial_clusters, &pixels);

        KMeansSuperPixelSolver {
            flat_pixels: pixels,
            clusters: initial_clusters,
            height: image_height,
            width: image_width,
            m: compactness,
            s: pixel_size,
            n: pixel_count,
            k: superpixel_count
        }
    }

    fn calculate_superpixel_size(pixel_count: usize, superpixel_count: usize) -> u16 {
        let result = (pixel_count / superpixel_count).sqrt();
        match result % 2 == 1 {
            true => return result as u16,
            false => return (result + 1) as u16
        }
    }

    fn calculate_initial_centroids(pixels: &Vec<PixelCieLab>, superpixel_count: usize) -> Vec<PixelCieLab> {
        unimplemented!()
    }

    fn assign_pixels_to_superpixels(superpixels: &mut Vec<PixelCluster>, pixels: &Vec<PixelCieLab>) {
        unimplemented!()
    }
}