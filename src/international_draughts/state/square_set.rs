use crate::international_draughts::state::point::add;
use crate::international_draughts::state::vector::Vector;
use crate::international_draughts::state::square::Square;

const MIN_N: i8 = 0;
const MAX_N: i8 = 9;

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

pub fn between_points(from: (i8, i8), to: (i8, i8)) -> Vec<(i8, i8)> {
    let vector = Vector { from, to };

    if vector.diagonal() {
        let direction_unit = vector.direction_unit();
        let end = to;
        let mut counter = add(from, direction_unit);
        let mut result = vec![];

        while counter != end {
            result.push(counter.clone());
            counter = add(counter, direction_unit);
        }

        result
    } else {
        vec![]
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
        let to_point = (3, 3);
        let result = between_points(from_point, to_point);
        assert_eq!(result, vec![(1, 1), (2, 2)]);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from_point = (0, 0);
        let to_point = (2, 1);
        let result = between_points(from_point, to_point);
        assert_eq!(result, vec![]);
    }
}
