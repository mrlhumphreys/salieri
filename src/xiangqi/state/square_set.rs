use crate::xiangqi::state::point::add;
use crate::xiangqi::state::point::direction_unit;
use crate::xiangqi::state::point::direction_unit_n;
use crate::xiangqi::state::point::length;
use crate::xiangqi::state::point::valid;
// use crate::xiangqi::state::point::between;
// use crate::xiangqi::state::point::orthogonal;
// use crate::xiangqi::state::point::diagonal;
use crate::xiangqi::state::point::orthogonal_or_diagonal;
// use crate::xiangqi::state::point::points_in_line;
use crate::xiangqi::state::square::opposing_player;
// use crate::xiangqi::state::square::ranging;
use crate::xiangqi::state::square::PieceKind;
use crate::xiangqi::state::square::Square;
// use crate::xiangqi::state::square::destinations;
use crate::xiangqi::state::square::threats_matches_point;
// use crate::xiangqi::state::game_state::GameState;

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

pub fn between_l_unoccupied(squares: &Vec<Vec<Square>>, from: (i8, i8), to: (i8, i8)) -> bool {
    if (to.1 - from.1).abs() == 2 {
        let direction = direction_unit_n(from.1, to.1);
        let between = (from.0, from.1 + direction);
        if let Some(square) = find_by_x_and_y(squares, between) {
            square.unoccupied()
        } else {
            true
        }
    } else if (to.0 - from.0).abs() == 2 {
        let direction = direction_unit_n(from.0, to.0);
        let between = (from.0 + direction, from.1);
        if let Some(square) = find_by_x_and_y(squares, between) {
            square.unoccupied()
        } else {
            true
        }
    } else {
        true
    }
}

pub fn between_occupied_by_one(squares: &Vec<Vec<Square>>, from: (i8, i8), to: (i8, i8)) -> bool {
    let mut occupied_count = 0;

    if orthogonal_or_diagonal(from, to) && length(from, to) > 1 {
        let direction_unit = direction_unit(from, to);
        let end = to;
        let mut counter = add(from, direction_unit);
        while counter != end {
            if let Some(square) = find_by_x_and_y(squares, counter) {
               if square.occupied() {
                   occupied_count += 1;
               }
            }
            counter = add(counter, direction_unit);
        }
    }
    occupied_count == 1
}

pub fn find_king_point_for_player(squares: &Vec<Vec<Square>>, player_number: i8) -> Option<(i8, i8)> {
    let mut king_point = None;

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if s.kind == PieceKind::King && s.player_number == player_number {
                king_point = Some((x as i8, y as i8));
                break;
            }
        }
        if king_point.is_some() {
            break;
        }
    }

    king_point
}

// Returns all opposing points that threaten the subject point.
// pub fn threats_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
//     let mut acc = vec![];
//
//     let opposing_player_number = opposing_player(player_number);
//
//     for (y, row) in squares.iter().enumerate() {
//         for (x, s) in row.iter().enumerate() {
//             // get opposing squares
//             if s.player_number == opposing_player_number && s.kind != PieceKind::King {
//                 // get opposing squares threatened points
//                 let threatened_matches_point = threats_matches_point(s.kind, s.player_number, (x as i8, y as i8), squares, point);
//                 // return the opposing square's point if a threatened point matches the specified point
//                 // i.e. the opposing square is threatening the target
//                 if threatened_matches_point {
//                     acc.push((x as i8, y as i8));
//                 }
//             }
//         }
//     }
//
//     acc
// }

// Returns true if any oppsoing points threatens the subject point.
pub fn any_threats_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8) -> bool {
    let mut result = false;

    let opposing_player_number = opposing_player(player_number);

    for (y, row) in squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            // get opposing squares
            if s.player_number == opposing_player_number {
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
// pub fn any_threats_to_point_through_pin(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8, pin: (i8, i8)) -> bool {
//     let mut result = false;
//
//     let opposing_player_number = opposing_player(player_number);
//
//     // identify ranging piece kinds by direction of pin
//     let ranging_piece_kinds = if orthogonal(point, pin) {
//         vec![PieceKind::Chariot, PieceKind::Cannon]
//     } else if diagonal(point, pin) {
//         vec![PieceKind::Elephant]
//     } else {
//         // TODO: add logic for horse
//         vec![]
//     };
//
//     if !ranging_piece_kinds.is_empty() {
//         // TODO: modify points_in_line logic to handle horse
//         let points = points_in_line(point, pin);
//         for p in points.iter() {
//             if let Some(s) = find_by_x_and_y(&squares, *p) {
//                 if ranging_piece_kinds.iter().any(|pk| *pk == s.kind) && s.player_number == opposing_player_number {
//                     // point is occupied by opposing hisha or ryuuou
//                     result = true; // is threat through pin
//                     break;
//                 } else if s.player_number != 0 {
//                     // point is occupied
//                     break; // is not threat through pin
//                 }
//                 // do nothing - continue loop until piece is found or until board edge
//             }
//         }
//     }
//
//     result
// }


// Returns all points owned by player number and between an ooposing ranging piece and the subject point
// Used to find pieces that can't move due to being in between a ranging piece and their king
// pub fn pinned_to_point(squares: &Vec<Vec<Square>>, point: (i8, i8), player_number: i8, game_state: &GameState) -> Vec<(i8, i8)> {
//     let opposing_player_number = opposing_player(player_number);
//
//     let mut acc = vec![];
//
//     for (y, row) in squares.iter().enumerate() {
//         for (x, s) in row.iter().enumerate() {
//             // square is occupied by an opposing piece with range movement
//             if s.player_number == opposing_player_number && ranging(s.kind) {
//                 let opposing_point = (x as i8, y as i8);
//                 let threatened_points = destinations(s.kind, s.player_number, opposing_point, game_state, true);
//
//                 // if any threatened point matches the subject point
//                 if threatened_points.iter().any(|t| { return *t == point; }) {
//                     let between_points = between(opposing_point, point);
//                     between_points.iter().for_each(|b| {
//                         if squares[b.1 as usize][b.0 as usize].player_number == player_number {
//                             acc.push(*b);
//                         }
//                     });
//                 }
//             }
//         }
//     }
//     acc
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xiangqi::state::game_state::parse;

    #[test]
    fn squares_set_find_by_x_and_y_mut_test() {
        let mut squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Soldier }
            ]
        ];

        let result = find_by_x_and_y_mut(&mut squares, (1, 1));
        match result {
            Some(s) => {
                assert_eq!(s.player_number, 1);
                assert_eq!(s.kind, PieceKind::Soldier);
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
                Square { player_number: 1, kind: PieceKind::Soldier }
            ]
        ];

        let result = find_by_x_and_y(&squares, (1, 1));
        match result {
            Some(s) => {
                assert_eq!(s.player_number, 1);
                assert_eq!(s.kind, PieceKind::Soldier);
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
                Square { player_number: 1, kind: PieceKind::Soldier }
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
    fn square_set_between_l_unoccupied_true_test() {
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
                Square { player_number: 1, kind: PieceKind::Horse }
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

        let origin_point: (i8, i8) = (1, 2);
        let destination_point: (i8, i8) = (0, 0);

        let result = between_l_unoccupied(&squares, origin_point, destination_point);
        assert_eq!(result, true);
    }

    #[test]
    fn square_set_between_l_unoccupied_false_test() {
        let squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 1, kind: PieceKind::Horse },
                Square { player_number: 2, kind: PieceKind::Soldier },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let origin_point: (i8, i8) = (0, 1);
        let destination_point: (i8, i8) = (2, 0);

        let result = between_l_unoccupied(&squares, origin_point, destination_point);
        assert_eq!(result, false);

    }

    #[test]
    fn square_set_between_occupied_by_one_true_test() {
        let squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Chariot }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Soldier }
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

        let result = between_occupied_by_one(&squares, origin_point, destination_point);
        assert_eq!(result, true);
    }

    #[test]
    fn square_set_between_occupied_by_one_false_test() {
        let squares = vec![
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Chariot }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Soldier }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 1, kind: PieceKind::Soldier }
            ],
            vec![
                Square { player_number: 0, kind: PieceKind::Empty },
                Square { player_number: 0, kind: PieceKind::Empty }
            ]
        ];

        let origin_point: (i8, i8) = (1, 1);
        let destination_point: (i8, i8) = (1, 4);

        let result = between_occupied_by_one(&squares, origin_point, destination_point);
        assert_eq!(result, false);

    }

    #[test]
    fn find_king_point_for_player_test() {
        let squares = vec![
            vec![
                Square { player_number: 2, kind: PieceKind::King },
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
                Square { player_number: 1, kind: PieceKind::King }
            ]
        ];
        let player_number = 1;
        let expected = Some((2, 2));
        let result = find_king_point_for_player(&squares, player_number);
        assert_eq!(result, expected);
    }

    // #[test]
    // fn threats_to_point_direct_test() {
    //     let encoded = String::from("4k4/9/9/9/9/9/9/9/4R4/4K4 w - - 0 0");
    //     let game_state = parse(&encoded).unwrap();
    //     let point = (4, 0);
    //     let player_number = 2;
    //     let squares = &game_state.squares.clone();
    //     let expected = vec![
    //         (4, 8)
    //     ];
    //     let result = threats_to_point(squares, point, player_number);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn threats_to_point_potential_test() {
    //     let encoded = String::from("4k4/4C4/9/9/9/9/9/9/4R4/4K4 w - - 0 0");
    //     let game_state = parse(&encoded).unwrap();
    //     let point = (4, 1);
    //     let player_number = 2;
    //     let squares = &game_state.squares.clone();
    //     let expected = vec![
    //         (4, 8)
    //     ];
    //     let result = threats_to_point(squares, point, player_number);
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn any_threats_to_point_direct_true_test() {
        let encoded = String::from("4k4/9/9/9/9/9/9/9/4R4/4K4 w - - 0 0");
        let game_state = parse(&encoded).unwrap();
        let point = (4, 0);
        let player_number = 2;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, true);
    }

    #[test]
    fn any_threats_to_point_direct_false_test() {
        let encoded = String::from("4k4/9/9/9/9/9/9/9/4A4/4K4 w - - 0 0");
        let game_state = parse(&encoded).unwrap();
        let point = (4, 0);
        let player_number = 2;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, false);
    }

    #[test]
    fn any_threats_to_point_potential_true_test() {
        let encoded = String::from("4k4/4C4/9/9/9/9/9/9/4R4/4K4 w - - 0 0");
        let game_state = parse(&encoded).unwrap();
        let point = (4, 1);
        let player_number = 2;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, true);
    }

    #[test]
    fn any_threats_to_point_potential_false_test() {
        let encoded = String::from("4k4/4C4/9/9/9/9/9/9/4A4/4K4 w - - 0 0");
        let game_state = parse(&encoded).unwrap();
        let point = (4, 1);
        let player_number = 2;
        let squares = &game_state.squares.clone();
        let result = any_threats_to_point(squares, point, player_number);
        assert_eq!(result, false);
    }

    // #[test]
    // fn any_threats_to_point_through_pin_true_test() {
    //     let encoded = String::from("4k4/4a4/9/9/9/9/9/9/4R4/4K4 w - - 0 0");
    //     let game_state = parse(&encoded).unwrap();
    //     let point = (4, 0);
    //     let pin = (4, 1);
    //     let player_number = 2;
    //     let squares = game_state.squares;
    //     let result = any_threats_to_point_through_pin(&squares, point, player_number, pin);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn any_threats_to_point_through_pin_false_test() {
    //     let encoded = String::from("4k4/4a4/9/9/9/9/9/9/4A4/4K4 w - - 0 0");
    //     let game_state = parse(&encoded).unwrap();
    //     let point = (4, 0);
    //     let pin = (4, 1);
    //     let player_number = 2;
    //     let squares = game_state.squares;
    //     let result = any_threats_to_point_through_pin(&squares, point, player_number, pin);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn pinned_to_point_test() {
    //     let encoded = String::from("4k4/4a4/9/9/9/9/9/9/4R4/4K4 w - - 0 0");
    //     let game_state = parse(&encoded).unwrap();
    //     let point = (4, 0);
    //     let player_number = 2;
    //     let squares = &game_state.squares.clone();
    //     let result = pinned_to_point(squares, point, player_number, &game_state);
    //     let expected = vec![
    //         (4, 1)
    //     ];
    //     assert_eq!(result, expected);
    // }
}
