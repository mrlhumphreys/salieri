use crate::shogi::state::point::add;
use crate::shogi::state::point::direction_unit;
use crate::shogi::state::point::length;
use crate::shogi::state::point::valid;
use crate::shogi::state::point::between;
use crate::shogi::state::point::orthogonal;
use crate::shogi::state::point::diagonal;
use crate::shogi::state::point::forwards_for_player;
use crate::shogi::state::point::orthogonal_or_diagonal;
use crate::shogi::state::point::points_in_line;
use crate::shogi::state::square::ou_kind;
use crate::shogi::state::square::opposing_player;
use crate::shogi::state::square::ranging;
use crate::shogi::state::square::PieceKind;
use crate::shogi::state::square::Square;
use crate::shogi::state::square::destinations;
use crate::shogi::state::square::threats_matches_point;
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

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if s.kind == ou_kind(player_number) {
                ou_point = Some((x as i8, y as i8));
                break;
            }
        }
        if ou_point.is_some() {
            break;
        }
    }

    ou_point
}

// Returns all opposing points that threaten the subject point.
pub fn threats_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let mut acc = vec![];

    let opposing_player_number = opposing_player(player_number);

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            // get opposing squares
            if s.player_number == opposing_player_number && s.kind != PieceKind::Oushou && s.kind != PieceKind::Gyokushou {
                // get opposing squares threatened points
                let threatened_matches_point = threats_matches_point(s.kind, s.player_number, (x as i8, y as i8), squares, point);
                // return the opposing square's point if a threatened point matches the specified point
                // i.e. the opposing square is threatening the target
                if threatened_matches_point {
                    acc.push((x as i8, y as i8));
                }
            }
        }
    }

    acc
}

// Returns true if any oppsoing points threatens the subject point.
pub fn any_threats_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8) -> bool {
    let mut result = false;

    let opposing_player_number = opposing_player(player_number);

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            // get opposing squares
            if s.player_number == opposing_player_number && s.kind != PieceKind::Oushou && s.kind != PieceKind::Gyokushou {
                // get opposing squares threatened points
                let threatened_matches_point = threats_matches_point(s.kind, s.player_number, (x as i8, y as i8), squares, point);
                // return the opposing square's point if a threatened point matches the specified point
                // i.e. the opposing square is threatening the target
                if threatened_matches_point {
                    result = true;
                    break;
                }
            }
        }
        if result {
            break;
        }
    }

    result
}

// Is the point actually threatened if the pin was not there?
// i.e. if the piece on pin moved to point, would it still be threatened?
pub fn any_threats_to_point_through_pin(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8, pin: (i8, i8)) -> bool {
    let mut result = false;

    let opposing_player_number = opposing_player(player_number);

    // identify ranging piece kinds by direction of pin
    let ranging_piece_kinds = if forwards_for_player(point, pin, player_number) {
        vec![PieceKind::Kyousha, PieceKind::Hisha, PieceKind::Ryuuou]
    } else if orthogonal(point, pin) {
        vec![PieceKind::Hisha, PieceKind::Ryuuou]
    } else if diagonal(point, pin) {
        vec![PieceKind::Kakugyou, PieceKind::Ryuuma]
    } else {
        vec![]
    };

    if !ranging_piece_kinds.is_empty() {
        let points = points_in_line(point, pin);
        for p in points.iter() {
            if let Some(s) = find_by_x_and_y(&squares, *p) {
                if ranging_piece_kinds.iter().any(|pk| *pk == s.kind) && s.player_number == opposing_player_number {
                    // point is occupied by opposing hisha or ryuuou
                    result = true; // is threat through pin
                    break;
                } else if s.player_number != 0 {
                    // point is occupied
                    break; // is not threat through pin
                }
                // do nothing - continue loop until piece is found or until board edge
            }
        }
    }

    result
}


// Returns all points owned by player number and between an ooposing ranging piece and the subject point
// Used to find pieces that can't move due to being in between a ranging piece and their king
pub fn pinned_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8, game_state: &GameState) -> Vec<(i8, i8)> {
    let opposing_player_number = opposing_player(player_number);

    let mut acc = vec![];

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            // square is occupied by an opposing piece with range movement
            if s.player_number == opposing_player_number && ranging(s.kind) {
                let opposing_point = (x as i8, y as i8);
                let threatened_points = destinations(s.kind, s.player_number, opposing_point, game_state, true);

                // if any threatened point matches the subject point
                if threatened_points.iter().any(|t| { return *t == point; }) {
                    let between_points = between(opposing_point, point);
                    between_points.iter().for_each(|b| {
                        if squares[b.1 as usize][b.0 as usize].player_number == player_number {
                            acc.push(*b);
                        }
                    });
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
    fn threats_to_point_direct_test() {
        let encoded = String::from("k8/9/9/9/9/9/7Bg/6g1p/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8, 8);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let expected = vec![
            (8, 7)
        ];
        let result = threats_to_point(squares, point, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn threats_to_point_potential_test() {
        let encoded = String::from("k8/9/9/9/9/9/7Bg/6g1p/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8, 7);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let expected = vec![
            (8, 6)
        ];
        let result = threats_to_point(squares, point, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn any_threats_to_point_direct_true_test() {
        let encoded = String::from("k8/9/9/9/9/9/7Bg/6g1p/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8, 8);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, true);
    }

    #[test]
    fn any_threats_to_point_direct_false_test() {
        let encoded = String::from("k8/9/9/9/9/9/7Bg/6g2/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8, 8);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, false);
    }

    #[test]
    fn any_threats_to_point_potential_true_test() {
        let encoded = String::from("k8/9/9/9/9/9/7Bg/6g1p/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8, 7);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, true);
    }

    #[test]
    fn any_threats_to_point_potential_false_test() {
        let encoded = String::from("k8/9/9/9/9/9/7B1/6g1p/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8, 7);
        let player_number = 1;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, false);
    }

    #[test]
    fn any_threats_to_point_through_pin_true_test() {
        let encoded = String::from("k8/9/4r4/9/9/9/4K4/9/9 b -");
        let game_state = parse(&encoded).unwrap();
        let point = (4, 7);
        let pin = (4, 6);
        let player_number = 1;
        let squares = game_state.squares;
        let result = any_threats_to_point_through_pin(&squares, point, player_number, pin);
        assert_eq!(result, true);
    }

    #[test]
    fn any_threats_to_point_through_pin_false_test() {
        let encoded = String::from("k8/9/9/9/9/9/4K4/9/9 b -");
        let game_state = parse(&encoded).unwrap();
        let point = (4, 7);
        let pin = (4, 6);
        let player_number = 1;
        let squares = game_state.squares;
        let result = any_threats_to_point_through_pin(&squares, point, player_number, pin);
        assert_eq!(result, false);
    }

    #[test]
    fn pinned_to_point_test() {
        let encoded = String::from("6Rbk/8P/9/9/9/9/9/8R/8K w -");
        let game_state = parse(&encoded).unwrap();
        let point = (8,0);
        let player_number = 2;
        let squares = &game_state.squares.clone();
        let result = pinned_to_point(squares, point, player_number, &game_state);
        let expected = vec![
            (7, 0)
        ];
        assert_eq!(result, expected);
    }
}
