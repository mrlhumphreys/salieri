use crate::checkers::state::vector::Vector;
use crate::checkers::state::square::Square;

const MIN_N: i8 = 0;
const MAX_N: i8 = 7;

pub fn find_by_x_and_y(squares: &Vec<Vec<Square>>, x: i8, y: i8) -> Option<&Square> {
    if x < MIN_N || x > MAX_N || y < MIN_N || y > MAX_N {
        None
    } else {
        Some(&squares[y as usize][x as usize])
    }
}

pub fn find_by_x_and_y_mut(squares: &mut Vec<Vec<Square>>, x: i8, y: i8) -> Option<&mut Square> {
    if x < MIN_N || x > MAX_N || y < MIN_N || y > MAX_N {
        None
    } else {
        Some(&mut squares[y as usize][x as usize])
    }
}

pub fn between_point(from: (i8, i8), to: (i8, i8)) -> Option<(i8, i8)> {
    let vector = Vector { from, to };

    if vector.diagonal() {
        let direction_unit = vector.direction_unit();
        let between_x = from.0 + direction_unit.0;
        let between_y = from.1 + direction_unit.1;
        if between_x != to.0 && between_y != to.1 {
            Some((between_x, between_y))
        } else {
            None
        }
    } else {
       None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_by_x_and_y_test() {
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false }
            ]
        ];
        match find_by_x_and_y(&squares, 1, 1) {
            Some(result) => {
                assert_eq!(result.player_number, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn find_by_x_and_y_mut_test() {
        let mut squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false }
            ]
        ];
        match find_by_x_and_y_mut(&mut squares, 1, 1) {
            Some(result) => {
                assert_eq!(result.player_number, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn fetching_between_diagonal() {
        let from_point = (0, 0);
        let to_point = (2, 2);
        let result = between_point(from_point, to_point).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, 1);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from_point = (0, 0);
        let to_point = (2, 1);
        let result = between_point(from_point, to_point);
        assert_eq!(result, None);
    }
}
