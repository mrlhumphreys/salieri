use std::fmt;

use crate::checkers::state::vector::Vector;
use crate::checkers::state::vector::Direction;
use crate::checkers::state::square::Square;

#[derive(PartialEq, Debug)]
pub struct SquareSet {
    pub squares: Vec<Square>,
}

impl Clone for SquareSet {
    fn clone(&self) -> SquareSet {
        SquareSet {
            squares: self.squares.clone(),
        }
    }
}

impl fmt::Display for SquareSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let characters = self.squares.iter().map(|s| {
            match s.player_number {
                1 => "b",
                2 => "w",
                _ => "-"
            }
        }).collect::<String>();
        write!(f, "{}", characters)
    }
}

pub fn find_by_x_and_y(squares: &Vec<Square>, x: i8, y: i8) -> Option<&Square> {
    squares.iter().find(|s| { s.x == x && s.y == y })
}

pub fn between(squares: &Vec<Square>, from: &Square, to: &Square) -> Vec<Square> {
    let vector = Vector { from: from.point(), to: to.point() };

    let squares = if vector.direction() == Direction::Other {
        Vec::new()
    } else {
        let direction_unit = vector.direction_unit();
        let end = to.point();
        let mut counter = from.point() + direction_unit;
        let mut acc = Vec::new();
        while counter != end {
            let square = find_by_x_and_y(squares, counter.x, counter.y);
            if let Some(s) = square {
                acc.push(*s)
            }
            counter = counter + direction_unit;
        }
        acc
    };

    squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetching_by_x_and_y() {
        let first = Square { id: 1, x: 1, y: 1, player_number: 0, king: false };
        let second = Square { id: 2, x: 2, y: 2, player_number: 0, king: false };
        let squares = vec![first, second];
        let result = find_by_x_and_y(&squares, 1, 1);
        match result {
            Some(square) => {
                assert_eq!(square.x, 1);
                assert_eq!(square.y, 1);
            },
            None => assert!(false, "Expected Square"),
        }
    }

    #[test]
    fn fetching_between_diagonal() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between_square = Square { id: 2, x: 5, y: 3, player_number: 1, king: false };
        let to = Square { id: 3, x: 6, y: 2, player_number: 0, king: false };
        let squares = vec![from, between_square, to];
        let result = between(&squares, &from, &to);
        assert_eq!(result.len(), 1);
        let square = &result[0];
        assert_eq!(square.x, 5);
        assert_eq!(square.y, 3);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between_square = Square { id: 2, x: 5, y: 4, player_number: 0, king: false };
        let to = Square { id: 3, x: 6, y: 3, player_number: 0, king: false };
        let squares = vec![from, between_square, to];
        let result = between(&squares, &from, &to);
        assert_eq!(result.len(), 0);
    }
}
