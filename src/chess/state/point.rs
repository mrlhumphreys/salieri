use std::ops;

pub const MIN_N: i8 = 0;
pub const MAX_N: i8 = 7;

pub fn valid(point: (i8, i8)) -> bool {
    point.0 >= MIN_N && point.0 <= MAX_N && point.1 >= MIN_N && point.1 <= MAX_N
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_true_test() {
        let point = (4, 4);
        let result = valid(point);
        assert_eq!(result, true);
    }

    #[test]
    fn valid_false_test() {
        let point = (4, 8);
        let result = valid(point);
        assert_eq!(result, false);
    }

    #[test]
    fn adding() {
        let left = Point { x: 4, y: 4 };
        let right = Point { x: 2, y: 6 };
        let result = left + right;
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 10);
    }
}
