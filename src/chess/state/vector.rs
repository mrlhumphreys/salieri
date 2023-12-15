use std::cmp::Ordering;
use crate::chess::state::point::Point;
use crate::chess::state::castle_move::Side;

pub struct Vector {
    pub from: Point,
    pub to: Point,
}

// #[derive(PartialEq)]
// pub enum Direction {
//     Diagonal,
//     Orthogonal,
//     Other,
// }

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

    pub fn direction_unit(&self) -> Point {
        let dx = self.to.x - self.from.x;
        let dy = self.to.y - self.from.y;
        let ux = match dx.partial_cmp(&0) {
            Some(c) => {
                match c {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                }
            },
            None => 0,
        };
        let uy = match dy.partial_cmp(&0) {
            Some(c) => {
                match c {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                }
            },
            None => 0,
        };

        Point { x: ux, y: uy }
    }

    pub fn diagonal(&self) -> bool {
        let abs_dx = (self.to.x - self.from.x).abs();
        let abs_dy = (self.to.y - self.from.y).abs();
        self.from != self.to && abs_dx == abs_dy
    }

    pub fn orthogonal(&self) -> bool {
        let same_x = self.to.x == self.from.x;
        let same_y = self.to.y == self.from.y;
        (self.from != self.to) && (same_x || same_y)
    }

    pub fn orthogonal_or_diagonal(&self) -> bool {
        self.orthogonal() || self.diagonal()
    }

    pub fn not_orthogonal_or_diagonal(&self) -> bool {
        !self.orthogonal() && !self.diagonal()
    }

    pub fn length(&self) -> i8 {
        if self.diagonal() {
            (self.to.x - self.from.x).abs()
        } else if self.orthogonal() {
            if self.to.x == self.from.x {
                 (self.to.y - self.from.y).abs()
            } else {
                 (self.to.x - self.from.x).abs()
            }
        } else {
            let dx = (self.to.x - self.from.x).abs();
            let dy = (self.to.y - self.from.y).abs();
            if dx > dy {
                dx
            } else {
                dy
            }
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

    // #[test]
    // fn direction_diagonal() {
    //     let from = Point { x: 4, y: 4 };
    //     let to = Point { x: 2, y: 6 };
    //     let vector = Vector { from, to };
    //     let result = vector.direction();
    //     assert_eq!(result, Direction::Diagonal);
    // }

    // #[test]
    // fn direction_orthogonal() {
    //     let from = Point { x: 4, y: 4 };
    //     let to = Point { x: 4, y: 6 };
    //     let vector = Vector { from, to };
    //     let result = vector.direction();
    //     assert_eq!(result, Direction::Orthogonal);
    // }

    // #[test]
    // fn direction_other() {
    //     let from = Point { x: 5, y: 4 };
    //     let to = Point { x: 4, y: 6 };
    //     let vector = Vector { from, to };
    //     let result = vector.direction();
    // }

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
