use std::ops;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point { x: self * rhs.x, y: self * rhs.y }
    }
}

pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn bounding_box(points: &Vec<Point>) -> Rect {
        let x_min = points.iter().map(|p| p.x).min().unwrap();
        let x_max = points.iter().map(|p| p.x).max().unwrap();
        let y_min = points.iter().map(|p| p.y).min().unwrap();
        let y_max = points.iter().map(|p| p.y).max().unwrap();
        let width = (x_max - x_min + 1) as u32;
        let height = (y_max - y_min + 1) as u32;
        Rect { x: x_min, y: y_min, width: width, height: height }
    }

    pub fn x_min(&self) -> i32 { self.x }
    pub fn y_min(&self) -> i32 { self.y }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
}
