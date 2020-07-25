use cgmath::Point2;
use num::clamp;

const MIN_WIDTH: u8 = 0;
const MIN_HEIGHT: u8 = 0;

pub struct Grid {
    points: Vec<Point2<u16>>
}

impl Grid {
    pub fn new(center: impl Into<Point2<u16>>, ring_count: u8, max_width: u16, max_height: u16) -> Grid {
        let center_point: Point2<u16> = center.into();

        let min_x: u16 = num::clamp(center_point.x as i16 - ring_count as i16, MIN_WIDTH as i16, max_width as i16) as u16;
        let min_y: u16 = num::clamp(center_point.y as i16 - ring_count as i16, MIN_HEIGHT as i16, max_height as i16) as u16;
        
        let max_x: u16 = num::clamp(center_point.x as u16 + ring_count as u16, MIN_WIDTH as u16, max_width);
        let max_y: u16 = num::clamp(center_point.y as u16 + ring_count as u16, MIN_HEIGHT as u16, max_height);

        let mut points:Vec<Point2<u16>> = Vec::with_capacity(max_x as usize * max_y as usize);

        for x in min_x..max_x {
            for y in min_y..max_y {
                points.push(Point2::new(x, y));
            }
        }

        Grid{
            points
        }
    }

    pub fn points(&self) -> &Vec<Point2<u16>> {&self.points}
}

#[cfg(test)]
mod test {
    use crate::pixels::Grid;

    #[test]
    fn simple_grid_should_work() {
        // Arrange
        let grid = Grid::new((0, 0), 2, 4, 4);
        
        assert_eq!(4, grid.points().len())
    }
}