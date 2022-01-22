/// Used to represent a point in two-dimensional space.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// The number of all possible [`Point`]s that are smaller than or equal to
    /// `self`: `(x + 1)(y + 1)`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use gol::misc::Point;
    /// 
    /// assert_eq!(Point::new(0, 1).smaller(), 2); // (0, 0), (0, 1)
    /// assert_eq!(Point::new(1, 2).smaller(), 6); // (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)
    /// ```
    #[inline]
    pub fn smaller(&self) -> u32 {
        (self.x as u32 + 1) * (self.y as u32 + 1)
    }

    /// Invokes [`Self::order_around_x`] with `point`'s x-value incremented by one. 
    /// Used to order many points in respect to one.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use gol::misc::Point;
    /// 
    /// // (3, 2) is the 13th (counting from 0) point in respect to (4, 1):
    /// // (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (0, 1), (1, 1), (2, 1),
    /// // (3, 1), (4, 1), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (0, 3)...
    /// assert_eq!(Point::new(3, 2).order(Point::new(4, 1)), 13);
    /// ```
    #[inline]
    pub fn order(&self, point: Point) -> u32 {
        self.order_around_x(point.x + 1)
    }

    /// 'Wraps' this point around some x-value: `self.x + self.y * x`.
    #[inline]
    pub fn order_around_x(&self, x: u16) -> u32 {
        self.x as u32 + self.y as u32 * x as u32
    }
}
