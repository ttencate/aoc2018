use std::cmp;
use std::iter::FromIterator;
use std::ops;

// A discrete point on a 2D integer grid.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn x_min(&self) -> i32 { self.x }
    pub fn x_max(&self) -> i32 { self.x + self.width as i32 - 1 }
    pub fn y_min(&self) -> i32 { self.y }
    pub fn y_max(&self) -> i32 { self.y + self.height as i32 - 1 }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
}

// Allows building a Rect as a bounding box of a series of Points, e.g. using collect().
impl<'a> FromIterator<&'a Point> for Rect {
    fn from_iter<T: IntoIterator<Item=&'a Point>>(points: T) -> Rect {
        let (x_min, x_max, y_min, y_max) = points
            .into_iter()
            .fold(
                (i32::max_value(), i32::min_value(), i32::max_value(), i32::min_value()),
                |(x_min, x_max, y_min, y_max), &point| {
                    (
                        cmp::min(x_min, point.x), cmp::max(x_max, point.x),
                        cmp::min(y_min, point.y), cmp::max(y_max, point.y)
                    )
                });
        let width = (x_max - x_min + 1) as u32;
        let height = (y_max - y_min + 1) as u32;
        Rect { x: x_min, y: y_min, width: width, height: height }
    }
}
