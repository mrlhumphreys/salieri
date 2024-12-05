use crate::checkers::state::point::Point;
use crate::checkers::state::vector::Vector;
use crate::checkers::state::vector::Direction;
use crate::checkers::state::square::Square;

const MIN_N: usize = 0;
const MAX_N: usize = 7;

pub fn find_by_x_and_y(squares: &Vec<Vec<Square>>, x: usize, y: usize) -> Option<&Square> {
    if x < MIN_N || x > MAX_N || y < MIN_N || y > MAX_N {
        None
    } else {
        Some(&squares[y][x])
    }
}

pub fn find_by_x_and_y_mut(squares: &mut Vec<Vec<Square>>, x: usize, y: usize) -> Option<&mut Square> {
    if x < MIN_N || x > MAX_N || y < MIN_N || y > MAX_N {
        None
    } else {
        Some(&mut squares[y][x])
    }
}

pub fn between_point(from: Point, to: Point) -> Option<(usize, usize)> {
    let vector = Vector { from, to };

    if vector.direction() == Direction::Other {
       return None;
    } else {
        let direction_unit = vector.direction_unit();
        let between_x = from.x as i8 + direction_unit.x;
        let between_y = from.y as i8 + direction_unit.y;
        if between_x != to.x && between_y != to.y {
            Some((between_x as usize, between_y as usize))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_by_x_and_y_test() {
        let squares = vec![
            vec![
                Square { id: 1, x: 0, y: 0, player_number: 0, king: false },
                Square { id: 2, x: 1, y: 0, player_number: 0, king: false }
            ],
            vec![
                Square { id: 3, x: 0, y: 1, player_number: 0, king: false },
                Square { id: 4, x: 1, y: 1, player_number: 0, king: false }
            ]
        ];
        match find_by_x_and_y(&squares, 1, 1) {
            Some(result) => {
                assert_eq!(result.x, 1);
                assert_eq!(result.y, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn find_by_x_and_y_mut_test() {
        let mut squares = vec![
            vec![
                Square { id: 1, x: 0, y: 0, player_number: 0, king: false },
                Square { id: 2, x: 1, y: 0, player_number: 0, king: false }
            ],
            vec![
                Square { id: 3, x: 0, y: 1, player_number: 0, king: false },
                Square { id: 4, x: 1, y: 1, player_number: 0, king: false }
            ]
        ];
        match find_by_x_and_y_mut(&mut squares, 1, 1) {
            Some(result) => {
                assert_eq!(result.x, 1);
                assert_eq!(result.y, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn fetching_between_diagonal() {
        let from_point = Point { x: 0, y: 0 };
        let to_point = Point { x: 2, y: 2 };
        let result = between_point(from_point, to_point).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 1);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from_point = Point { x: 0, y: 0 };
        let to_point = Point { x: 2, y: 1 };
        let result = between_point(from_point, to_point);
        assert_eq!(result, None);
    }
}
