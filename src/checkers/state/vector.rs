use std::fmt;
use std::cmp::Ordering;
use crate::checkers::state::point::Point;

pub struct Vector {
    pub from: Point,
    pub to: Point
}

pub struct DirectionUnit {
    pub x: i8,
    pub y: i8
}

#[derive(PartialEq)]
pub enum Direction {
    Diagonal,
    Orthogonal,
    Other,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Direction::Diagonal => "Diagonal",
            Direction::Orthogonal => "Orthogonal",
            Direction::Other => "Other",
        };
        write!(f, "{:?}", string)
    }
}

impl Vector {
    pub fn direction(&self) -> Direction {
        if self.diagonal() {
            Direction::Diagonal
        } else if self.orthogonal() {
            Direction::Orthogonal
        } else {
            Direction::Other
        }
    }

    pub fn direction_unit(&self) -> DirectionUnit {
        let dx = self.to.x as i8 - self.from.x as i8;
        let dy = self.to.y as i8 - self.from.y as i8;
        let ux = if let Some(c) = dx.partial_cmp(&0) {
            match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0
            }
        } else {
            0
        };
        let uy = if let Some(c) = dy.partial_cmp(&0) {
            match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0
            }
        } else {
            0
        };

        DirectionUnit { x: ux , y: uy }
    }

    pub fn diagonal(&self) -> bool {
        let abs_dx = (self.to.x as i8 - self.from.x as i8).abs();
        abs_dx != 0 && abs_dx == (self.to.y as i8 - self.from.y as i8).abs()
    }

    pub fn orthogonal(&self) -> bool {
        (self.to.x == self.from.x) ^ (self.to.y == self.from.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_diagonal() {
        let from = Point { x: 4, y: 4 };
        let to = Point { x: 2, y: 6 };
        let vector = Vector { from, to };
        let result = vector.direction();
        assert_eq!(result, Direction::Diagonal);
    }

    #[test]
    fn direction_orthogonal() {
        let from = Point { x: 4, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        let result = vector.direction();
        assert_eq!(result, Direction::Orthogonal);
    }

    #[test]
    fn direction_other() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        let result = vector.direction();
        assert_eq!(result, Direction::Other);
    }

    #[test]
    fn direction_unit_test() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        let result = vector.direction_unit();
        assert_eq!(result.x, -1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn same_dx_and_dy() {
        let from = Point { x: 4, y: 4 };
        let to = Point { x: 2, y: 6 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), true);
        assert_eq!(vector.orthogonal(), false);
    }

    #[test]
    fn same_y_or_x() {
        let from = Point { x: 4, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
        assert_eq!(vector.orthogonal(), true);
    }

    #[test]
    fn different_dx_and_dy_and_x_and_y() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
        assert_eq!(vector.orthogonal(), false);
    }

    #[test]
    fn same_points() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 5, y: 4 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
        assert_eq!(vector.orthogonal(), false);
    }
}
