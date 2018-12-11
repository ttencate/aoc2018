use std::cmp;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ops;
use std::ops::RangeInclusive;

// A discrete point on a 2D integer grid.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
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

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:},{:}", self.x, self.y);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Rect {
    pub fn from_inclusive_ranges(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Rect {
        Rect { x_range: x_range, y_range: y_range }
    }

    pub fn x_min(&self) -> i32 { *self.x_range.start() }
    pub fn x_max(&self) -> i32 { *self.x_range.end() }
    pub fn y_min(&self) -> i32 { *self.y_range.start() }
    pub fn y_max(&self) -> i32 { *self.y_range.end() }
    pub fn width(&self) -> u32 { cmp::max(0i32, self.x_range.end() - self.x_range.start() + 1) as u32 }
    pub fn height(&self) -> u32 { cmp::max(0i32, self.y_range.end() - self.y_range.start() + 1) as u32 }

    pub fn contains(&self, point: Point) -> bool {
        // Range::contains is nightly-only.
        self.x_min() <= point.x && point.x <= self.x_max() && self.y_min() <= point.y && point.y <= self.y_max()
    }

    pub fn iter(&self) -> RectIter {
        self.clone().into_iter()
    }
}

#[test]
fn rect_contains_test() {
    let rect = Rect::from_inclusive_ranges(1 ..= 3, 1 ..= 2);
    assert!(rect.contains(Point::new(1, 1)));
    assert!(rect.contains(Point::new(1, 1)));
    assert!(!rect.contains(Point::new(-1, -1)));
    assert!(!rect.contains(Point::new(0, 0)));
    assert!(!rect.contains(Point::new(0, 1)));
    assert!(!rect.contains(Point::new(1, 0)));
    assert!(!rect.contains(Point::new(4, 2)));
    assert!(!rect.contains(Point::new(3, 3)));
    assert!(!rect.contains(Point::new(3, 4)));
}

// Allows building a Rect as a bounding box of a series of Points, e.g. using collect().
impl<'a> FromIterator<&'a Point> for Rect {
    fn from_iter<T: IntoIterator<Item=&'a Point>>(points: T) -> Rect {
        points
            .into_iter()
            .fold(
                Rect::from_inclusive_ranges(i32::max_value() ..= i32::min_value(), i32::max_value() ..= i32::min_value()),
                |rect, &point| {
                    Rect::from_inclusive_ranges(
                        cmp::min(*rect.x_range.start(), point.x) ..= cmp::max(*rect.x_range.end(), point.x),
                        cmp::min(*rect.y_range.start(), point.y) ..= cmp::max(*rect.y_range.end(), point.y)
                    )
                })
    }
}

// Allows iterating over the Points contained in a Rect.
impl IntoIterator for Rect {
    type Item = Point;
    type IntoIter = RectIter;
    fn into_iter(self) -> Self::IntoIter {
        RectIter { rect: self.clone(), current_point: Point::new(self.x_max(), self.y_min() - 1) }
    }
}

pub struct RectIter {
    rect: Rect,
    current_point: Point,
}

impl Iterator for RectIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.current_point.x += 1;
        if self.current_point.x > self.rect.x_max() {
            self.current_point.x = self.rect.x_min();
            self.current_point.y += 1;
        }
        if self.rect.contains(self.current_point) {
            Some(self.current_point)
        } else {
            None
        }
    }
}

#[test]
fn rect_into_iter_test() {
    assert_eq!(Rect::from_inclusive_ranges(0..=-1, 0..=-1).into_iter().collect::<Vec<Point>>(), vec![]);
    assert_eq!(Rect::from_inclusive_ranges(0..=0, 0..=-1).into_iter().collect::<Vec<Point>>(), vec![]);
    assert_eq!(Rect::from_inclusive_ranges(0..=-1, 0..=0).into_iter().collect::<Vec<Point>>(), vec![]);
    assert_eq!(Rect::from_inclusive_ranges(0..=0, 0..=0).into_iter().collect::<Vec<Point>>(), vec![
        Point::new(0, 0),
    ]);
    assert_eq!(Rect::from_inclusive_ranges(2..=3, -2..=-2).into_iter().collect::<Vec<Point>>(), vec![
        Point::new(2, -2),
        Point::new(3, -2),
    ]);
    assert_eq!(Rect::from_inclusive_ranges(2..=2, -2..=-1).into_iter().collect::<Vec<Point>>(), vec![
        Point::new(2, -2),
        Point::new(2, -1),
    ]);
}

// A dense 2D rectangular array with customizable lower bound.
pub struct Matrix<T> {
    rect: Rect,
    values: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rect: &Rect, initial_value: T) -> Matrix<T>
        where T: Clone
    {
        Matrix { rect: rect.clone(), values: vec![initial_value; (rect.width() * rect.height()) as usize] }
    }

    pub fn coords(&self) -> impl Iterator<Item=Point> {
        self.rect.iter()
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if self.rect.contains(point) {
            Some(&self.values[self.index_of(point)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
        if self.rect.contains(point) {
            let index = self.index_of(point);
            Some(&mut self.values[index])
        } else {
            None
        }
    }

    fn index_of(&self, point: Point) -> usize {
        ((point.y - self.rect.y_min()) * self.rect.width() as i32 + point.x - self.rect.x_min()) as usize
    }
}

impl<T> ops::Index<Point> for Matrix<T> {
    type Output = T;
    fn index(&self, point: Point) -> &T {
        assert!(self.rect.contains(point));
        &self.values[self.index_of(point)]
    }
}

impl<T> ops::IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, point: Point) -> &mut T {
        assert!(self.rect.contains(point));
        let index = self.index_of(point);
        &mut self.values[index]
    }
}

#[test]
fn test_matrix_index() {
    let mut mat = Matrix::new(&Rect::from_inclusive_ranges(1 ..= 2, 1 ..= 2), 0);
    mat[Point::new(1, 1)] = 1;
    mat[Point::new(1, 2)] = 2;
    mat[Point::new(2, 1)] = 3;
    mat[Point::new(2, 2)] = 4;
    assert_eq!(mat.get(Point::new(1, 1)), Some(&1));
    assert_eq!(mat.get(Point::new(1, 2)), Some(&2));
    assert_eq!(mat.get(Point::new(2, 1)), Some(&3));
    assert_eq!(mat.get(Point::new(2, 2)), Some(&4));
    assert_eq!(mat.get(Point::new(0, 0)), None);
    assert_eq!(mat.get(Point::new(0, 1)), None);
    assert_eq!(mat.get(Point::new(1, 0)), None);
    assert_eq!(mat.get(Point::new(3, 2)), None);
    assert_eq!(mat.get(Point::new(2, 3)), None);
    assert_eq!(mat.get(Point::new(3, 3)), None);
}
