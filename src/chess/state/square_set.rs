use crate::chess::state::vector::direction_unit_n;
use crate::chess::state::vector::length;
use crate::chess::state::point::valid;
use crate::chess::state::vector::orthogonal_or_diagonal;
use crate::chess::state::square::Square;

pub fn find_by_x_and_y_mut(squares: &mut Vec<Vec<Square>>, x: i8, y: i8) -> Option<&mut Square> {
    if valid((x, y)) {
        Some(&mut squares[y as usize][x as usize])
    } else {
        None
    }
}

pub fn find_by_x_and_y(squares: &Vec<Vec<Square>>, x: i8, y: i8) -> Option<&Square> {
    if valid((x, y)) {
        Some(&squares[y as usize][x as usize])
    } else {
        None
    }
}

pub fn between_unoccupied(squares: &Vec<Vec<Square>>, from: (i8, i8), to: (i8, i8)) -> bool {
    let mut result = true;

    if orthogonal_or_diagonal(from.0, from.1, to.0, to.1) && length(from.0, from.1, to.0, to.1) > 1 {
        let direction_unit_x = direction_unit_n(from.0, to.0);
        let direction_unit_y = direction_unit_n(from.1, to.1);
        let end_x = to.0;
        let end_y = to.1;
        let mut counter_x = from.0 + direction_unit_x;
        let mut counter_y = from.1 + direction_unit_y;
        while counter_x != end_x || counter_y != end_y {
            if let Some(square) = find_by_x_and_y(squares, counter_x, counter_y) {
               if square.occupied() {
                   result = false;
                   break;
               }
            }
            counter_x = counter_x + direction_unit_x;
            counter_y = counter_y + direction_unit_y;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::state::square::PieceKind;

    #[test]
    fn squares_set_find_by_x_and_y_mut_test() {
        let mut squares = vec![
            vec![
                Square { x: 0, y: 0, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 0, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 1, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 1, player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let result = find_by_x_and_y_mut(&mut squares, 1, 1);
        match result {
            Some(s) => {
                assert_eq!(s.x, 1);
                assert_eq!(s.y, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn squares_set_find_by_x_and_y_test() {
        let squares = vec![
            vec![
                Square { x: 0, y: 0, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 0, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 1, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 1, player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let result = find_by_x_and_y(&squares, 1, 1);
        match result {
            Some(s) => {
                assert_eq!(s.x, 1);
                assert_eq!(s.y, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn square_set_between_unoccupied_true_test() {
        let squares = vec![
            vec![
                Square { x: 0, y: 0, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 0, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 1, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 1, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 2, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 2, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 3, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 3, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 4, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 4, player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let origin_point: (i8, i8) = (1, 1);
        let destination_point: (i8, i8) = (1, 3);

        let result = between_unoccupied(&squares, origin_point, destination_point);
        assert_eq!(result, true);
    }

    #[test]
    fn square_set_between_unoccupied_false_test() {
        let squares = vec![
            vec![
                Square { x: 0, y: 0, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 0, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 1, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 1, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 2, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 2, player_number: 1, kind: PieceKind::Pawn }
            ],
            vec![
                Square { x: 0, y: 3, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 3, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 4, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 4, player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { x: 0, y: 4, player_number: 0, kind: PieceKind::Empty },
                Square { x: 1, y: 5, player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let origin_point: (i8, i8) = (1, 1);
        let destination_point: (i8, i8) = (1, 4);

        let result = between_unoccupied(&squares, origin_point, destination_point);
        assert_eq!(result, false);
    }
}
