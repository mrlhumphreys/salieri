use std::ops;

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
    fn adding() {
        let left = Point { x: 4, y: 4 };
        let right = Point { x: 2, y: 6 };
        let result = left + right;
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 10);
    }
}
