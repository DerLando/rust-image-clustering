use crate::pixels::{PixelCieLab, Grid};
use crate::colors::ColorCieLab;
use num::integer::Roots;
use rand::thread_rng;
use rand_distr::{Distribution, Alphanumeric, Uniform, Standard, Poisson};
use std::collections::HashMap;
use cgmath::{Point2, EuclideanSpace};
use image::Rgb;

struct LabelPixel {
    pixel: PixelCieLab,
    centroid_distance: f32,
    centroid_index: usize
}

impl LabelPixel {
    pub const fn new(pixel: PixelCieLab) -> LabelPixel {
        LabelPixel {
            pixel,
            centroid_distance: std::f32::MAX,
            centroid_index: 0
        }
    }

    pub fn pixel(&self) -> &PixelCieLab {&self.pixel}

    pub fn centroid_distance_mut(&mut self) -> &mut f32 {&mut self.centroid_distance}
    pub fn centroid_index_mut(&mut self) -> &mut usize {&mut self.centroid_index}
}

pub struct KMeansSuperPixelSolver{
    flat_pixels: Vec<LabelPixel>,
    centroid_indices: Vec<usize>,
    pixel_relation: HashMap<usize, usize>,

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
    pub fn m(&self) -> u8 {self.m}

    pub fn new(pixels: Vec<PixelCieLab>, compactness: u8, superpixel_count: usize,
        image_width: usize, image_height: usize) -> KMeansSuperPixelSolver {
        
        let label_pixels: Vec<LabelPixel> = 
            pixels.into_iter()
            .map(|p| LabelPixel::new(p))
            .collect();

        let pixel_count = image_width * image_height;
        let pixel_size = Self::calculate_superpixel_size(pixel_count, superpixel_count);

        let mut solver = KMeansSuperPixelSolver {
            flat_pixels: label_pixels,
            centroid_indices: Vec::new(),
            pixel_relation: HashMap::new(),
            height: image_height,
            width: image_width,
            m: compactness,
            s: pixel_size,
            n: pixel_count,
            k: superpixel_count
        };

        solver.calculate_initial_centroids();
        println!("initial centroids: {:?}", solver.centroid_indices);
        solver.assign_pixels_to_superpixels();

        solver
    }

    fn get_index(x: usize, y: usize, height: usize) -> usize {
        height * x + y
    }

    fn calculate_superpixel_size(pixel_count: usize, superpixel_count: usize) -> u16 {
        let result = (pixel_count / superpixel_count).sqrt();
        match result % 2 == 1 {
            true => return result as u16,
            false => return (result + 1) as u16
        }
    }

    fn calculate_initial_centroids(&mut self) {
        let rng = rand::thread_rng();

        let x_positions = 
            Uniform::new_inclusive((self.s / 2) as usize,  self.width - (self.s / 2) as usize)
            .sample_iter(rng)
            .take(self.k);
            
        let y_positions = 
            Uniform::new_inclusive((self.s / 2) as usize,  self.height - (self.s / 2) as usize)
            .sample_iter(rng)
            .take(self.k);

        self.centroid_indices =
            x_positions
            .zip(y_positions)
            .map(|(x, y)| Self::get_index(x, y, self.height))
            .collect();
    }

    fn centroids(&self) -> Vec<&LabelPixel> {
        self.centroid_indices
            .iter()
            .map(|i| &self.flat_pixels[*i])
            .collect()
    }

    fn pixel(&self, position: impl Into<Point2<u32>>) -> &LabelPixel {
        let point: Point2<u32> = position.into();
        &self.flat_pixels[Self::get_index(point.x as usize, point.y as usize, self.height)]
    }

    fn pixel_mut(&mut self, x: usize, y: usize) -> &mut LabelPixel {
        &mut self.flat_pixels[Self::get_index(x, y, self.height)]
    }

    fn color(&self, position: impl Into<Point2<u32>>) -> ColorCieLab {
        self.pixel(position).pixel().color().clone()
    }

    fn pixels(&self) -> &Vec<LabelPixel> {&self.flat_pixels}

    fn assign_pixels_to_superpixels(&mut self) {

        // copy fields, so we don't take an immutable reference to self :/
        let m = self.m;
        let s = self.s;
        let centroids: Vec<PixelCieLab> = 
            self.centroids().iter()
            .map(|c| PixelCieLab::new(c.pixel().x(), c.pixel().y(), c.pixel().color().clone()))
            .collect();

        
        for i in 0..self.k{
            let centroid = &centroids[i];
            let centroid_x = centroid.values().x;
            let centroid_y = centroid.values().y;

            let grid = 
                Grid::new((centroid_x as u16, centroid_y as u16),
                self.s as u8, self.width as u16, self.height as u16);

            for point in grid.points() {
                let mut pixel = self.pixel_mut(point.x as usize, point.y as usize);

                let distance = PixelCieLab::distance(pixel.pixel(), &centroid, m, s as f32);

                if pixel.centroid_distance < distance {continue;}

                pixel.centroid_distance = distance;
                pixel.centroid_index = i;
            }
        }
        
    }

    fn position_centroid(positions: &Vec<Point2<u32>>) -> Point2<u32> {
        // Point2::centroid(&positions.iter().map(|p| p.cast().unwrap()).collect::<Vec<Point2<u32>>>()[..])
        match positions.len() == 0 {
            true => return Point2::from((0, 0)),
            false => return Point2::centroid(&positions[..])
        }
    }

    fn position_clusters(&self) -> Vec<Vec<Point2<u32>>> {
        let mut clusters: Vec<Vec<Point2<u32>>> = Vec::with_capacity(self.k);
        for _ in 0..self.k {clusters.push(Vec::new())}

        for pixel in self.pixels() {
            clusters[pixel.centroid_index].push(*pixel.pixel().values())
        }

        clusters
    }

    fn position_centroids(&self) -> Vec<Point2<u32>> {
        println!("Calculating position centroids");
        self.position_clusters().iter()
            .map(|c| Self::position_centroid(c))
            .collect()
    }

    fn update_centroids(&mut self) {
        println!("Updating position centroids");
        self.centroid_indices = 
            self.position_centroids().iter()
            .map(|c| Self::get_index(c.x as usize, c.y as usize, self.height)).collect();
    }

    pub fn solve_tick(&mut self) {
        println!("Solving tick...");
        println!("Assigning pixels to superpixels...");
        self.assign_pixels_to_superpixels();
        println!("Updating centroids...");
        self.update_centroids();
    }

    pub fn current_superpixels(&self) -> Vec<Vec<(u32, u32, Rgb<u8>)>> {
        let mut result: Vec<Vec<(u32, u32, Rgb<u8>)>> = Vec::with_capacity(self.k);

        for cluster in self.position_clusters() {
            result.push(
                cluster.iter()
                .map(|c|
                    (c.x, c.y, self.color(*c).as_xyz().as_rgb().as_image_rgb())).collect());
        }

        let counts: Vec<usize> = result.iter().map(|r| r.len()).collect();
        println!("Generated superpixel counts: {:?}", counts);

        result
    }
}