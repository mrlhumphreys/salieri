use crate::checkers::state::point::Point;
use crate::checkers::state::vector::Vector;
use crate::checkers::state::vector::Direction;
use crate::checkers::state::square::Square;

pub fn find_by_x_and_y(squares: &Vec<Square>, x: i8, y: i8) -> Option<&Square> {
    squares.iter().find(|s| { s.x == x && s.y == y })
}

pub fn between<'a>(squares: &'a Vec<Square>, from: Point, to: Point) -> Option<&'a Square> {
    let vector = Vector { from, to };

    if vector.direction() == Direction::Other {
       return None; 
    } else {
        let direction_unit = vector.direction_unit();
        let end = to;
        let mut counter = from + direction_unit;
        while counter != end {
            let square = find_by_x_and_y(squares, counter.x, counter.y);
            if square.is_some() {
                return square;
            }
            counter = counter + direction_unit;
        }
    };

    return None;
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
        let from_point = Point { x: 4, y: 4 };
        let between_square = Square { id: 2, x: 5, y: 3, player_number: 1, king: false };
        let to = Square { id: 3, x: 6, y: 2, player_number: 0, king: false };
        let to_point = Point { x: 6, y: 2 };
        let squares = vec![from, between_square, to];
        let result = between(&squares, from_point, to_point).unwrap();
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let from_point = Point { x: 4, y: 4 };
        let between_square = Square { id: 2, x: 5, y: 4, player_number: 0, king: false };
        let to = Square { id: 3, x: 6, y: 3, player_number: 0, king: false };
        let to_point = Point { x: 6, y: 3 };
        let squares = vec![from, between_square, to];
        let result = between(&squares, from_point, to_point);
        assert_eq!(result, None);
    }
}
