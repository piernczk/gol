use crate::misc;
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    size: misc::Point,
    data: BitVec,
    generation: u64,
    wrap: bool
}

impl Board {
    pub fn new(size: misc::Point, wrap: bool) -> Self {
        let mut bv = BitVec::new();
        // fill with false
        for _ in 0..size.smaller() {
            bv.push(false);
        }

        Self { size, data: bv, generation: 0, wrap }
    }

    pub fn set(&mut self, p: misc::Point, value: bool) {
        if self.wrap {
            let wrapped = misc::Point::new(p.x % self.size.x, p.y % self.size.y);
            self.data.set(wrapped.order(self.size) as usize, value)
        } else {
            self.data.set(p.order(self.size) as usize, value)
        }
    }

    pub fn get(&self, p: misc::Point) -> bool {
        if self.wrap {
            let wrapped = misc::Point::new(p.x % self.size.x, p.y % self.size.y);
            self.data.get(wrapped.order(self.size) as usize).map(|v| *v).unwrap()
        } else {
            // return false if out of bounds
            self.data.get(p.order(self.size) as usize).map(|v| *v).unwrap_or(false)
        }
    }

    pub fn next(&self) -> Board {
        let mut board = self.clone();

        for (i, val) in self.data.iter().enumerate() {
            let point = self.size.from_order(i as u32);

            // check for living neighbors
            let mut living_neighbors = 0;
            if self.get(misc::Point::new(point.x + 1, point.y + 1)) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(point.x + 1, point.y)) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(point.x + 1, abs_diff(point.y, 1))) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(point.x, abs_diff(point.y, 1))) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(abs_diff(point.x, 1), abs_diff(point.y, 1))) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(abs_diff(point.x, 1), point.y)) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(abs_diff(point.x, 1), point.y + 1)) {
                living_neighbors += 1
            }
            if self.get(misc::Point::new(point.x, point.y + 1)) {
                living_neighbors += 1
            }

            if *val {
                if living_neighbors < 2 || living_neighbors > 3 {
                    board.set(point, false);
                }
            } else if living_neighbors == 3 {
                board.set(point, true);
            }
        }

        board.generation += 1;
        board
    }

    pub fn cont(&mut self) {
        self.data = self.next().data;
        self.generation += 1;
    }

    pub fn get_size(&self) -> misc::Point {
        self.size
    }

    pub fn get_generation(&self) -> u64 {
        self.generation
    }

    pub fn is_wrapped(&self) -> bool {
        self.wrap
    }
}

fn abs_diff<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y { y - x }
    else { x - y }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(stages: Vec<Vec<(u16, u16)>>, size: misc::Point) {
        assert!(stages.len() >= 1);

        let mut board = Board::new(size, true);

        let stages: Vec<Vec<misc::Point>> = stages.into_iter().map(|points| points.into_iter().map(|t| t.into()).collect()).collect();

        for point in stages[0].clone() {
            board.set(point, true);
        }

        for x in 0..size.x {
            for y in 0..size.y {
                let point = misc::Point::new(x, y);
                assert_eq!(board.get(point), stages[0].contains(&point));
            }
        }

        board.cont();

        for i in 1..stages.len() {
            for x in 0..size.x {
                for y in 0..size.y {
                    let point = misc::Point::new(x, y);
                    assert_eq!(board.get(point), stages[i].contains(&point));
                }
            }
        }
    }

    fn test_still(points: Vec<(u16, u16)>, size: misc::Point) {
        test(vec![points.clone(), points], size)
    }

    #[test]
    fn block() {
        test_still(vec![(4, 4), (4, 5), (5, 4), (5, 5)], misc::Point::new(10, 10));
    }

    #[test]
    fn beehive() {
        test_still(vec![(2, 1), (3, 1), (1, 2), (4, 2), (2, 3), (3, 3)], misc::Point::new(6, 6));
    }

    #[test]
    fn blinker() {
        test(vec![
            vec![(2, 1), (2, 2), (2, 3)],
            vec![(1, 2), (2, 2), (3, 2)]
        ], misc::Point::new(5, 5));
    }
}
