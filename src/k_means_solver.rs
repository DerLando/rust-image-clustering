use crate::colors::{ColorCieLab, ColorRgb};
use crate::pixels::{Grid, PixelCieLab, Rectangle};
use cgmath::{EuclideanSpace, Point2};
use image::Rgb;
use num::integer::Roots;
use rand::thread_rng;
use rand_distr::{Alphanumeric, Distribution, Poisson, Standard, Uniform};
use rayon::prelude::*;
use std::collections::HashMap;

struct LabelPixel {
    pixel: PixelCieLab,
    centroid_distance: f32,
    centroid_index: usize,
}

impl LabelPixel {
    pub const fn new(pixel: PixelCieLab) -> LabelPixel {
        LabelPixel {
            pixel,
            centroid_distance: std::f32::MAX,
            centroid_index: std::usize::MAX, // this will panic if pixel is unassigned
        }
    }

    pub fn pixel(&self) -> &PixelCieLab {
        &self.pixel
    }

    pub fn centroid_distance_mut(&mut self) -> &mut f32 {
        &mut self.centroid_distance
    }
    pub fn centroid_index_mut(&mut self) -> &mut usize {
        &mut self.centroid_index
    }
}

pub struct KMeansSuperPixelSolver {
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
    k: usize,
}

impl KMeansSuperPixelSolver {
    pub fn m(&self) -> u8 {
        self.m
    }

    pub fn new(
        pixels: Vec<PixelCieLab>,
        compactness: u8,
        superpixel_count: usize,
        image_width: usize,
        image_height: usize,
    ) -> KMeansSuperPixelSolver {
        let label_pixels: Vec<LabelPixel> =
            pixels.into_iter().map(|p| LabelPixel::new(p)).collect();

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
            k: superpixel_count,
        };

        solver.calculate_initial_centroids();
        println!(
            "initial centroids: {:?}",
            solver
                .centroids()
                .iter()
                .map(|p| p.pixel.values())
                .collect::<Vec<_>>()
        );
        solver.assign_pixels_to_superpixels();

        solver
    }

    fn get_index(x: usize, y: usize, height: usize) -> usize {
        height * x + y
    }

    fn calculate_superpixel_size(pixel_count: usize, superpixel_count: usize) -> u16 {
        // TODO: Maybe we need to cast to f32 here bewfore calculation?
        let result = (pixel_count / superpixel_count).sqrt();
        match result % 2 == 1 {
            true => return result as u16,
            false => return (result + 1) as u16,
        }
    }

    fn calculate_initial_centroids(&mut self) {
        // let rng = rand::thread_rng();

        // let x_positions =
        //     Uniform::new_inclusive((self.s / 2) as usize,  self.width - (self.s / 2) as usize)
        //     .sample_iter(rng)
        //     .take(self.k);

        // let y_positions =
        //     Uniform::new_inclusive((self.s / 2) as usize,  self.height - (self.s / 2) as usize)
        //     .sample_iter(rng)
        //     .take(self.k);

        // self.centroid_indices =
        //     x_positions
        //     .zip(y_positions)
        //     .map(|(x, y)| Self::get_index(x, y, self.height))
        //     .collect();

        let rect = Rectangle::new(self.width as u32, self.height as u32);

        self.centroid_indices = rect
            .sample_positions(self.k as u32)
            .iter()
            .map(|p| Self::get_index(p.x as usize, p.y as usize, self.height))
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

    fn pixels(&self) -> &Vec<LabelPixel> {
        &self.flat_pixels
    }

    fn assign_pixels_to_superpixels(&mut self) {
        // copy fields, so we don't take an immutable reference to self :/
        let m = self.m;
        let s = self.s;
        let k = self.k;
        let centroids: Vec<PixelCieLab> = self
            .centroids()
            .iter()
            .map(|c| PixelCieLab::new(c.pixel().x(), c.pixel().y(), c.pixel().color().clone()))
            .collect();

        self.flat_pixels.par_iter_mut().for_each(|p| {
            // This could probably be a nested parallel for loop, but the compiler hates this idea
            for j in 0..k {
                let centroid = &centroids[j];

                let distance = PixelCieLab::distance(p.pixel(), &centroid, m, s as f32);

                if p.centroid_distance < distance {
                    continue;
                }

                p.centroid_distance = distance;
                p.centroid_index = j;
            }
        });

        // // Somethign is off here, many pixels are never found inside the grid, so probably a bug in grid...
        // for i in 0..self.k{
        //     let centroid = &centroids[i];
        //     //TODO: Don't see an obvious bug in grid...
        //     let grid =
        //         Grid::new((centroid.x() as u16, centroid.y() as u16),
        //         (self.s * 2) as u16, self.width as u16, self.height as u16);

        //     for point in grid.points() {
        //         //TODO: maybe the pixel does not get loaded correctly?
        //         let mut pixel = self.pixel_mut(point.x as usize, point.y as usize);

        //         //TODO: Either distance function is wrong or somethign else
        //         let distance = PixelCieLab::distance(pixel.pixel(), &centroid, m, s as f32);

        //         if pixel.centroid_distance < distance {continue;}

        //         pixel.centroid_distance = distance;
        //         pixel.centroid_index = i;
        //     }
        // }

        // let unassigned = self.flat_pixels.iter().filter(|p| p.centroid_index == std::usize::MAX).collect::<Vec<_>>();
        // println!("{:?} pixels unassigned!", unassigned.len());
    }

    fn position_centroid(positions: &Vec<Point2<u32>>) -> Point2<u32> {
        // Point2::centroid(&positions.iter().map(|p| p.cast().unwrap()).collect::<Vec<Point2<u32>>>()[..])
        match positions.len() == 0 {
            true => return Point2::from((0, 0)),
            false => return Point2::centroid(&positions[..]),
        }
    }

    fn position_clusters(&self) -> Vec<Vec<Point2<u32>>> {
        let mut clusters: Vec<Vec<Point2<u32>>> = (0..self.k).map(|_| Vec::new()).collect();
        // let mut clusters: Vec<Vec<Point2<u32>>> = Vec::with_capacity(self.k);
        // for _ in 0..self.k {clusters.push(Vec::new())}

        for pixel in self.pixels() {
            clusters[pixel.centroid_index].push(*pixel.pixel().values())
        }

        clusters
    }

    fn position_centroids(&self) -> Vec<Point2<u32>> {
        // println!("Calculating position centroids");
        self.clusters()
            .iter()
            .map(|(_, pixels)| {
                Self::position_centroid(&pixels.iter().map(|p| *p.pixel().values()).collect())
            })
            .collect()
    }

    fn clusters(&self) -> Vec<(&LabelPixel, Vec<&LabelPixel>)> {
        let mut clusters: Vec<Vec<&LabelPixel>> = (0..self.k).map(|_| Vec::new()).collect();
        for pixel in self.pixels() {
            clusters[pixel.centroid_index].push(pixel)
        }

        let centroids = self.centroids();

        centroids.into_iter().zip(clusters.into_iter()).collect()
    }

    fn update_centroids(&mut self) {
        self.centroid_indices = self
            .position_centroids()
            .iter()
            .map(|c| Self::get_index(c.x as usize, c.y as usize, self.height))
            .collect();
    }

    fn cluster_color_average(colors: &Vec<ColorCieLab>) -> ColorRgb {
        let len = colors.len();

        let mut l = 0.0;
        let mut a = 0.0;
        let mut b = 0.0;

        for color in colors {
            l += color.l();
            a += color.a();
            b += color.b();
        }

        l /= len as f32;
        a /= len as f32;
        b /= len as f32;

        ColorCieLab::new(l, a, b).as_xyz().as_rgb()
        // let mut r: f32 = 0.0;
        // let mut g: f32 = 0.0;
        // let mut b: f32 = 0.0;

        // for color in colors {
        //     let values = color.as_xyz().as_rgb().values();
        //     r += values.0 as f32;
        //     g += values.1 as f32;
        //     b += values.2 as f32;
        // }

        // r /= len as f32;
        // g /= len as f32;
        // b /= len as f32;

        // ColorRgb::new(r as u8, g as u8, b as u8)
    }

    fn cluster_color_normalized(&self, index: usize) -> ColorRgb {
        let centroid = self.centroids()[index];
        let grid = Grid::new(
            (centroid.pixel().x() as u16, centroid.pixel().y() as u16),
            2,
            self.width as u16,
            self.height as u16,
        );

        Self::cluster_color_average(
            &grid
                .points()
                .iter()
                .map(|p| self.color((p.x as u32, p.y as u32)))
                .collect(),
        )
    }

    pub fn solve_tick(&mut self) {
        // TODO: SOmehow ticks scrample ordering of super pixels and colors...
        println!("Solving tick...");
        // println!("Assigning pixels to superpixels...");
        self.assign_pixels_to_superpixels();
        // println!("Updating centroids...");
        self.update_centroids();
    }

    pub fn current_superpixels(&self) -> Vec<Vec<(u32, u32, Rgb<u8>)>> {
        let mut result: Vec<Vec<(u32, u32, Rgb<u8>)>> = Vec::with_capacity(self.k);

        for (centroid, cluster) in self.clusters() {
            // let color = Self::cluster_color_average(&cluster.iter().map(|c| self.color(*c)).collect()).as_image_rgb();
            let color = centroid.pixel().color().as_xyz().as_rgb().as_image_rgb();
            println!("Color for cluster: {:?}", color);
            result.push(
                cluster
                    .iter()
                    .map(|c| (c.pixel().values().x, c.pixel().values().y, color))
                    .collect(),
            );
        }

        // let counts: Vec<usize> = result.iter().map(|r| r.len()).collect();
        // println!("Generated superpixel counts: {:?}", counts);

        result
    }
}
