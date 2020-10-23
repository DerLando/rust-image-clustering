use cgmath::Point2;

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn sample_positions(&self, n: u32) -> Vec<Point2<u32>> {
        // https://github.com/DerLando/ImageClustering/blob/master/ImageClusteringLibrary/Algorithms/PositionHelper.cs

        let mut positions = Vec::with_capacity(n as usize);
        let n_squared = (n as f32).sqrt() as u32;
        let mut x_count = self.width * n_squared / self.height;
        let y_count = n / x_count;
        let cell_width = self.width / x_count;
        let cell_height = self.height / y_count;
        let mut x_position: u32;
        let mut y_position: u32;

        // test if formula adds up to k
        let is_nice = x_count * y_count == n;
        if !is_nice {
            x_count -= 1;
        }

        // sample grid positions
        for i in 0..x_count {
            x_position = (cell_width as f32 * (i as f32 + 0.5)) as u32;

            for j in 0..y_count {
                y_position = (cell_height as f32 * (j as f32 + 0.5)) as u32;
                positions.push(Point2::from((x_position, y_position)))
            }
        }

        // if we had a nice x-y configuration we can return here
        if is_nice {
            positions
        } else {
            x_position = cell_width * (x_count as f32 + 0.5) as u32;
            let remainder = n - ((x_count + 1) * y_count - y_count);
            let cell_height = self.height / remainder;
            for i in 0..remainder {
                y_position = (cell_height as f32 * (i as f32 + 0.5)) as u32;
                positions.push(Point2::from((x_position, y_position)))
            }

            positions
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pixels::Rectangle;

    #[test]
    fn test_rect_sampling() {
        // Arrange
        let rect = Rectangle::new(20, 30);

        // Act
        let actual = rect.sample_positions(10);
        println!("{:?}", actual);
        assert_eq!(10, actual.len())
    }
}
