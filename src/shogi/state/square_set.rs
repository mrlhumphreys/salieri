use crate::shogi::state::point::add;
use crate::shogi::state::point::direction_unit;
use crate::shogi::state::point::length;
use crate::shogi::state::point::valid;
use crate::shogi::state::point::orthogonal_or_diagonal;
use crate::shogi::state::square::PieceKind;
use crate::shogi::state::square::Square;
use crate::shogi::state::square::destinations;
use crate::shogi::state::game_state::GameState;

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

pub fn between(from: (i8, i8), to: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];

    if orthogonal_or_diagonal(from, to) && length(from, to) > 1 {
        let direction_unit = direction_unit(from, to);
        let end = to;
        let mut counter = add(from, direction_unit);
        while counter != end {
            acc.push(counter);
            counter = add(counter, direction_unit);
        }
    }
    acc
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

pub fn find_ou_point_for_player(squares: &Vec<Vec<Square>>, player_number: i8) -> Option<(i8, i8)> {
    let mut ou_point = None;

    let moving_piece_kind = if player_number == 1 {
        PieceKind::Oushou
    } else {
        PieceKind::Gyokushou
    };

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if s.kind == moving_piece_kind {
                ou_point = Some((x as i8, y as i8));
            }
        }
    }

    ou_point
}

pub fn threats_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8, game_state: &GameState) -> Vec<(i8, i8)> {
    let opposing_player = if player_number == 2 {
        1
    } else {
        2
    };

    let mut acc = vec![];

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            // get opposing squares
            if s.player_number == opposing_player && s.kind != PieceKind::Oushou && s.kind != PieceKind::Gyokushou {
                // get opposing squares threatened points
                let threatened_points = destinations(s.kind, s.player_number, (x as i8, y as i8), game_state);
                // return the opposing point if a threatened point matches the specified point
                if threatened_points.iter().any(|t| { return *t == point; }) {
                    acc.push((x as i8, y as i8));
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shogi::state::game_state::parse;

    #[test]
    fn squares_set_find_by_x_and_y_mut_test() {
        let mut squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Fuhyou }
            ]
        ];

        let result = find_by_x_and_y_mut(&mut squares, (1, 1));
        match result {
            Some(s) => {
                assert_eq!(s.player_number, 1);
                assert_eq!(s.kind, PieceKind::Fuhyou);
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
                Square { player_number: 1, kind: PieceKind::Fuhyou }
            ]
        ];

        let result = find_by_x_and_y(&squares, (1, 1));
        match result {
            Some(s) => {
                assert_eq!(s.player_number, 1);
                assert_eq!(s.kind, PieceKind::Fuhyou);
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
                Square { player_number: 1, kind: PieceKind::Fuhyou }
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

    #[test]
    fn find_ou_point_for_player_test() {
        let squares = vec![
            vec![
                Square { player_number: 1, kind: PieceKind::Gyokushou },
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Oushou }
            ]
        ];
        let player_number = 1;
        let expected = Some((2, 2));
        let result = find_ou_point_for_player(&squares, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn threats_to_point_test() {
        let encoded = String::from("k8/9/9/9/9/9/7Bg/6g1p/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8,8);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let expected = vec![
            (8, 7)
        ];
        let result = threats_to_point(squares, point, player_number, &game_state);
        assert_eq!(result, expected);
    }
}
