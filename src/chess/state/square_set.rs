use crate::chess::state::point::add;
use crate::chess::state::point::direction_unit;
use crate::chess::state::point::length;
use crate::chess::state::point::valid;
use crate::chess::state::point::orthogonal_or_diagonal;
use crate::chess::state::square::Square;

pub fn find_by_x_and_y_mut(squares: &mut Vec<Vec<Square>>, point: (i8, i8)) -> Option<&mut Square> {
    if valid(point) {
        Some(&mut squares[point.1 as usize][point.0 as usize])
    } else {
        None
    }
}

pub fn find_by_x_and_y(squares: &Vec<Vec<Square>>, point: (i8, i8)) -> Option<&Square> {
    if valid(point) {
        Some(&squares[point.1 as usize][point.0 as usize])
    } else {
        None
    }
}

pub fn between_unoccupied(squares: &Vec<Vec<Square>>, from: (i8, i8), to: (i8, i8)) -> bool {
    let mut result = true;

    if orthogonal_or_diagonal(from, to) && length(from, to) > 1 {
        let direction_unit = direction_unit(from, to);
        let end = to;
        let mut counter = add(from, direction_unit);
        while counter != end {
            if let Some(square) = find_by_x_and_y(squares, counter) {
               if square.occupied() {
                   result = false;
                   break;
               }
            }
            counter = add(counter, direction_unit);
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
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Pawn }
            ]
        ];

        let result = find_by_x_and_y_mut(&mut squares, (1, 1));
        match result {
            Some(s) => {
                assert_eq!(s.player_number, 1);
                assert_eq!(s.kind, PieceKind::Pawn);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn squares_set_find_by_x_and_y_test() {
        let squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Pawn }
            ]
        ];

        let result = find_by_x_and_y(&squares, (1, 1));
        match result {
            Some(s) => {
                assert_eq!(s.player_number, 1);
                assert_eq!(s.kind, PieceKind::Pawn);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn square_set_between_unoccupied_true_test() {
        let squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
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
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Pawn }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let origin_point: (i8, i8) = (1, 1);
        let destination_point: (i8, i8) = (1, 4);

        let result = between_unoccupied(&squares, origin_point, destination_point);
        assert_eq!(result, false);
    }
}
