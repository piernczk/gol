#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn smaller(&self) -> u32 {
        (self.x as u32 + 1) * (self.y as u32 + 1)
    }

    #[inline]
    pub fn order(&self, point: Point) -> u32 {
        self.order_around_x(point.x + 1)
    }

    #[inline]
    pub fn order_around_x(&self, x: u16) -> u32 {
        self.x as u32 + self.y as u32 * x as u32
    }
}
