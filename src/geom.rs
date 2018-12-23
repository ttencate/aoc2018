use std::cmp;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ops;
use std::ops::{Range, RangeInclusive};

// A discrete point on a 2D integer grid.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn origin() -> Point { Point::new(0, 0) }

    pub fn left() -> Point { Point::new(-1, 0) }
    pub fn right() -> Point { Point::new(1, 0) }
    pub fn up() -> Point { Point::new(0, -1) }
    pub fn down() -> Point { Point::new(0, 1) }

    pub fn neighbors(self) -> [Point; 4] {
        [self + Point::up(), self + Point::left(), self + Point::right(), self + Point::down()]
    }

    pub fn neighbors_diagonal(self) -> [Point; 8] {
        [
            self + Point::up(),
            self + Point::up() + Point::right(),
            self + Point::right(),
            self + Point::right() + Point::down(),
            self + Point::down(),
            self + Point::down() + Point::left(),
            self + Point::left(),
            self + Point::left() + Point::up(),
        ]
    }

    pub fn distance_to(self, other: Point) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl cmp::PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Point) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl cmp::Ord for Point {
    fn cmp(&self, rhs: &Point) -> cmp::Ordering {
        self.y.cmp(&rhs.y)
            .then(self.x.cmp(&rhs.x))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:},{:}", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn new(x: i32, y: i32, z: i32) -> Point3 {
        Point3 { x: x, y: y, z: z }
    }

    pub fn origin() -> Point3 {
        Point3::default()
    }

    pub fn distance_to(&self, other: &Point3) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32 + (self.z - other.z).abs() as u32
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:},{:},{:}", self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn as_point(self) -> Point {
        match self {
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    Straight,
    Left,
    Right,
}

impl ops::Add<Turn> for Direction {
    type Output = Direction;
    fn add(self, turn: Turn) -> Direction {
        match turn {
            Turn::Straight => self,
            Turn::Left => match self {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            }
            Turn::Right => match self {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

fn bounding_range(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    // RangeInclusive::is_empty is nightly-only; ExactSizeIterator::len is only implemented up to
    // RangeInclusive<i16> (for compatibility with systems where usize is only 16 bits?).
    if a.size_hint().0 == 0 {
        b.clone()
    } else if b.size_hint().0 == 0 {
        a.clone()
    } else {
        cmp::min(*a.start(), *b.start()) ..= cmp::max(*a.end(), *b.end())
    }
}

impl Rect {
    pub fn from_inclusive_ranges(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Rect {
        Rect { x_range: x_range, y_range: y_range }
    }

    pub fn empty() -> Rect {
        Self::from_inclusive_ranges(0 ..= -1, 0 ..= -1)
    }

    pub fn from_exclusive_ranges(x_range: Range<i32>, y_range: Range<i32>) -> Rect {
        Self::from_inclusive_ranges(
            x_range.start ..= (x_range.end - 1),
            y_range.start ..= (y_range.end - 1))
    }

    pub fn bounding_rects(a: &Rect, b: &Rect) -> Rect {
        Self::from_inclusive_ranges(
            bounding_range(&a.x_range, &b.x_range),
            bounding_range(&a.y_range, &b.y_range))
    }

    pub fn x_range(&self) -> RangeInclusive<i32> { self.x_range.clone() }
    pub fn y_range(&self) -> RangeInclusive<i32> { self.y_range.clone() }
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

    pub fn padded(&self, left: i32, right: i32, top: i32, bottom: i32) -> Rect {
        Self::from_inclusive_ranges(
            self.x_min() - left ..= self.x_max() + right,
            self.y_min() - top ..= self.y_max() + bottom)
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

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}..={}, {}..={})", self.x_min(), self.x_max(), self.y_min(), self.y_max())
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
#[derive(PartialEq, Eq, Clone)]
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

    pub fn rect(&self) -> &Rect {
        &self.rect
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

    pub fn row(&self, y: i32) -> &[T] {
        let start = self.index_of(Point::new(self.rect.x_min(), y));
        let end = start + self.rect.width() as usize;
        &self.values[start..end]
    }

    pub fn map<U, F>(&self, f: F) -> Matrix<U>
        where F: Fn(&T) -> U, U: Clone + Default
    {
        let mut out = Matrix::new(&self.rect, Default::default());
        for i in 0..self.values.len() {
            out.values[i] = f(&self.values[i]);
        }
        out
    }

    pub fn as_slice(&self) -> &[T] {
        self.values.as_slice()
    }

    pub fn fill_rect(&mut self, rect: &Rect, value: T)
        where T: Clone
    {
        for point in rect.iter() {
            self[point] = value.clone();
        }
    }

    fn index_of(&self, point: Point) -> usize {
        ((point.y - self.rect.y_min()) * self.rect.width() as i32 + point.x - self.rect.x_min()) as usize
    }
}

impl<T> ops::Index<Point> for Matrix<T> {
    type Output = T;
    fn index(&self, point: Point) -> &T {
        assert!(self.rect.contains(point), "{} not inside bounds {}", point, self.rect);
        &self.values[self.index_of(point)]
    }
}

impl<T> ops::IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, point: Point) -> &mut T {
        assert!(self.rect.contains(point), "{} not inside bounds {}", point, self.rect);
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

// Creates a Matrix from lines of text. All lines must be equal in length. The top left cell will
// be (0, 0).
impl<'a> FromIterator<&'a str> for Matrix<u8> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let lines: Vec<&str> = iter.into_iter().collect();
        let height = lines.len();
        if height == 0 {
            return Matrix { rect: Rect::from_exclusive_ranges(0..0, 0..0), values: vec![] };
        }
        let width = lines[0].len();
        assert!(lines.iter().all(|line| line.len() == width));
        Matrix {
            rect: Rect::from_exclusive_ranges(0..width as i32, 0..height as i32),
            values: lines.iter().flat_map(|line| line.bytes()).collect(),
        }
    }
}

impl Display for Matrix<u8> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for y in self.rect().y_range() {
            write!(f, "{}", String::from_utf8_lossy(self.row(y)))?;
            if y < self.rect().y_max() {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
