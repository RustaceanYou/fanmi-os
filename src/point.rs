#[derive(Copy, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point::new(0, 0)
    }
}