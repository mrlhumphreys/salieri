use std::cmp::Ordering;
use crate::chess::state::point::Point;
use crate::chess::state::castle_move::Side;

pub struct Vector {
    pub from: Point,
    pub to: Point,
}

pub fn length(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> i8 {
    let dx = (to_x - from_x).abs();
    let dy = (to_y - from_y).abs();
    if dx > dy {
        dx
    } else {
        dy
    }
}

pub fn direction_unit_n(from_n: i8, to_n: i8) -> i8 {
    let dn = to_n - from_n;
    return match dn.partial_cmp(&0) {
        Some(c) => {
            match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            }
        },
        None => 0,
    };
}

pub fn orthogonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let same_x = to_x == from_x;
    let same_y = to_y == from_y;
    same_x ^ same_y
}

pub fn diagonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let abs_dx = (to_x - from_x).abs();
    let abs_dy = (to_y - from_y).abs();
    abs_dx != 0 && abs_dx == abs_dy
}

pub fn orthogonal_or_diagonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let abs_dx = (to_x - from_x).abs();
    let abs_dy = (to_y - from_y).abs();
    let same_x = to_x == from_x;
    let same_y = to_y == from_y;
    (same_x ^ same_y) || (abs_dx != 0 && abs_dx == abs_dy)
}

pub fn knight_jump(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let abs_dx = (to_x - from_x).abs();
    let abs_dy = (to_y - from_y).abs();
    (abs_dx == 2 && abs_dy == 1) || (abs_dx == 1 && abs_dy == 2)
}

impl Vector {
    // pub fn direction(&self) -> Direction {
    //     if self.diagonal() {
    //         Direction::Diagonal
    //     } else if self.orthogonal() {
    //         Direction::Orthogonal
    //     } else {
    //         Direction::Other
    //     }
    // }
    pub fn direction_unit_x(&self) -> i8 {
        let dx = self.to.x - self.from.x;
        return match dx.partial_cmp(&0) {
            Some(c) => {
                match c {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                }
            },
            None => 0,
        };
    }

    pub fn direction_unit_y(&self) -> i8 {
        let dy = self.to.y - self.from.y;
        return match dy.partial_cmp(&0) {
            Some(c) => {
                match c {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                }
            },
            None => 0,
        };
    }

    pub fn direction_unit(&self) -> Point {
        Point { x: self.direction_unit_x(), y: self.direction_unit_y() }
    }

    pub fn diagonal(&self) -> bool {
        let abs_dx = (self.to.x - self.from.x).abs();
        let abs_dy = (self.to.y - self.from.y).abs();
        abs_dx != 0 && abs_dx == abs_dy
    }

    pub fn length(&self) -> i8 {
        let dx = (self.to.x - self.from.x).abs();
        let dy = (self.to.y - self.from.y).abs();
        if dx > dy {
            dx
        } else {
            dy
        }
    }

    pub fn side(&self) -> Side {
        let dx = self.to.x - self.from.x;
        if dx > 0 {
            Side::King
        } else {
            Side::Queen
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_length_test() {
        let from_x = 1;
        let from_y = 1;
        let to_x = 2;
        let to_y = 4;
        let result = length(from_x, from_y, to_x, to_y);
        assert_eq!(result, 3);
    }

    #[test]
    fn vector_direction_unit_y_test() {
        let from_y = 4;
        let to_y = 6;
        let result = direction_unit_n(from_y, to_y);
        assert_eq!(result, 1);
    }

    #[test]
    fn vector_orthogonal_true_test() {
        let from_x = 4;
        let from_y = 6;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_orthogonal_false_test() {
        let from_x = 4;
        let from_y = 5;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_diagonal_true_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 2;
        let to_y = 6;
        let result = diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_diagonal_false_test() {
        let from_x = 4;
        let from_y = 6;
        let to_x = 2;
        let to_y = 6;
        let result = diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_orthogonal_or_diagonal_diagonal_true_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal_or_diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_orthogonal_or_diagonal_orthogonal_true_test() {
        let from_x = 4;
        let from_y = 6;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal_or_diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_orthogonal_or_diagonal_false_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 5;
        let to_y = 6;
        let result = orthogonal_or_diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_knight_jump_true_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 5;
        let to_y = 6;
        let result = knight_jump(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_knight_jump_false_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 6;
        let to_y = 6;
        let result = knight_jump(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn direction_unit_x_test() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        let result = vector.direction_unit_x();
        assert_eq!(result, -1);
    }

    #[test]
    fn direction_unit_y_test() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        let result = vector.direction_unit_y();
        assert_eq!(result, 1);
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
    }

    #[test]
    fn same_y_or_x() {
        let from = Point { x: 4, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
    }

    #[test]
    fn different_dx_and_dy_and_x_and_y() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 4, y: 6 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
    }

    #[test]
    fn same_points() {
        let from = Point { x: 5, y: 4 };
        let to = Point { x: 5, y: 4 };
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
    }
}
